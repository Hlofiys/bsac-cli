use std::io;
use std::io::{BufWriter, Error, ErrorKind, Write};
use bsac_api::{model, BsacApiClient};
use bsac_api::model::{Group, GroupListServiceResponse, Teacher, TeacherListServiceResponse};
use chrono::{Datelike, Local, Weekday};
use inquire::{Confirm, DateSelect, Select, Text};
use crate::{config, printer};

pub async fn find_group(
    client: &BsacApiClient,
    groups: Option<GroupListServiceResponse>,
    mut group_name: Option<String>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let groups_response: GroupListServiceResponse = match groups.as_ref() {
        Some(..) => groups.clone().unwrap(),
        None => client.get_groups().await?,
    };
    if group_name.is_none() {
        group_name = Option::from(Text::new("Твоя группа?").prompt().unwrap());
    }
    let groups_data = groups_response.data.as_ref().unwrap();
    let mut found_groups: Vec<&Group> = vec![];
    for group in groups_data {
        if group
            .group_number
            .as_ref()
            .unwrap()
            .to_uppercase()
            .contains(&group_name.as_ref().unwrap().to_uppercase())
        {
            found_groups.push(group);
        };
    }
    if found_groups.is_empty() {
        Err(Box::new(Error::new(
            ErrorKind::NotFound,
            "Группа не найдена",
        )))
    } else if found_groups.len() == 1 {
        return Ok(found_groups.first().unwrap().id.unwrap());
    } else if found_groups.len() > 1 {
        let mut group_names: Vec<&String> = vec![];
        for group in found_groups.clone() {
            group_names.push(group.group_number.as_ref().unwrap())
        }
        let ans = Select::new("Выбери группу из списка", group_names).prompt();
        let group = Box::pin(find_group(
            client,
            groups,
            Option::from(ans.unwrap().to_string()),
        ))
            .await?;
        return Ok(group);
    } else {
        Err(Box::new(Error::new(ErrorKind::InvalidData, "Ошибка")))
    }
}

pub async fn find_teacher(
    client: &BsacApiClient,
    teachers: Option<TeacherListServiceResponse>,
    mut teacher_name: Option<String>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let teachers_response: TeacherListServiceResponse = match teachers.as_ref() {
        Some(..) => teachers.clone().unwrap(),
        None => client.get_teachers().await?,
    };
    if teacher_name.is_none() {
        teacher_name = Option::from(Text::new("Фамилия преподавателя?").prompt()?);
    }
    let teachers_data = teachers_response.data.as_ref().unwrap();
    let mut found_teachers: Vec<&Teacher> = vec![];
    for teacher in teachers_data {
        if teacher
            .fio
            .as_ref()
            .unwrap()
            .to_uppercase()
            .contains(&teacher_name.as_ref().unwrap().to_uppercase())
        {
            found_teachers.push(teacher);
        };
    }
    if found_teachers.is_empty() {
        Err(Box::new(Error::new(
            ErrorKind::NotFound,
            "Преподаватель не найдена",
        )))
    } else if found_teachers.len() == 1 {
        return Ok(found_teachers.first().unwrap().id.unwrap());
    } else if found_teachers.len() > 1 {
        let mut teacher_names: Vec<&String> = vec![];
        for teacher in found_teachers.clone() {
            teacher_names.push(teacher.fio.as_ref().unwrap())
        }
        let ans = Select::new("Выбери преподавателя из списка", teacher_names).prompt();
        let teacher = Box::pin(find_teacher(
            client,
            teachers,
            Option::from(ans.unwrap().to_string()),
        ))
            .await?;
        return Ok(teacher);
    } else {
        Err(Box::new(Error::new(ErrorKind::InvalidData, "Ошибка")))
    }
}

pub async fn find_schedule(
    cfg: &config::AppConfig,
    client: &BsacApiClient,
    group_id: Option<i64>,
    teacher_id: Option<i64>,
) {
    let date = DateSelect::new("На какой день показать расписание?")
        .with_starting_date(Local::now().date_naive())
        .with_min_date(Local::now().date_naive())
        .with_max_date(
            Local::now()
                .date_naive()
                .with_year(Local::now().year() + 1)
                .unwrap(),
        )
        .with_week_start(Weekday::Mon)
        .with_help_message("Расписание будет показано на выбранный день")
        .prompt();
    let date = vec![date.unwrap()];
    let schedules: model::GetScheduleForOneGroupListServiceResponse;
    let display_type: printer::DisplayType;
    if group_id.is_some() {
        schedules = client
            .get_group_schedule_for_date(group_id.unwrap())
            .dates(date)
            .await
            .unwrap();
        display_type = printer::DisplayType::Teacher;
    } else if teacher_id.is_some() {
        schedules = client
            .get_teacher_schedule_for_date(teacher_id.unwrap())
            .dates(date)
            .await
            .unwrap();
        display_type = printer::DisplayType::Group;
    } else {
        panic!("No groupId nor teacherId");
    }
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = BufWriter::new(stdout); // optional: wrap that handle in a buffer
    // Достаем расписание нашей группы и клонируем временную переменную
    let schedules_data = schedules.data.unwrap().first().unwrap().clone();
    // Достаем расписание на день
    let group_schedules_data = schedules_data.schedules.as_ref().unwrap();
    if schedules_data.practice.is_some() {
        // Если есть практика - выводим
        let practice = schedules_data.practice.unwrap();
        let mut teacher_fio: Option<&String> = None;
        if practice.teacher.is_some() {
            teacher_fio = Option::from(practice.teacher.as_ref().unwrap().fio.as_ref().unwrap());
        }
        printer::print_practice_exam(&mut printer::PrintPracticeExamArgs {
            handle: &mut handle,
            start_time: &mut practice.start_time.unwrap(),
            name: practice.name.as_ref().unwrap(),
            end_time: &practice.end_time.unwrap(),
            teacher_fio,
            group_name: practice.group.as_ref().unwrap().group_number.as_ref(),
            cabinet: practice.cabinet.unwrap(),
            display_type: &display_type,
        });
    } else if !schedules_data.exam.as_ref().unwrap().is_empty() {
        // Если есть экзамены - выводим
        let exams = schedules_data.exam.unwrap();
        for exam in exams {
            printer::print_practice_exam(&mut printer::PrintPracticeExamArgs {
                handle: &mut handle,
                start_time: &mut exam.exam_start.unwrap().time().to_string(),
                name: exam.lesson.unwrap().name.as_ref().unwrap(),
                end_time: "",
                teacher_fio: exam.teacher.as_ref().unwrap().fio.as_ref(),
                group_name: exam.group.as_ref().unwrap().group_number.as_ref(),
                cabinet: exam.cabinet.unwrap(),
                display_type: &display_type,
            });
        }
    } else {
        // Иначе выводим расписание на день
        for schedule in group_schedules_data {
            if schedule.schedule_add.is_some() {
                let mut work_type: Option<&model::WorkTypeEnum> = None;
                if schedule.work.is_some() {
                    work_type = schedule.work.as_ref().unwrap().work_type.as_ref();
                }
                let schedule = schedule.schedule_add.as_ref().unwrap();
                printer::print_schedule(&mut printer::PrintScheduleArgs {
                    cfg,
                    handle: &mut handle,
                    schedule_type: printer::ScheduleType::Add,
                    lesson_id: schedule.lesson_id.unwrap(),
                    lesson_number: schedule.to_lesson_number.unwrap(),
                    sub_lesson_number: schedule.to_sub_lesson_number.unwrap(),
                    sub_group_number: schedule.to_sub_group.unwrap(),
                    lesson_name: schedule.lesson.as_ref().unwrap().name.as_ref().unwrap(),
                    teacher_fio: schedule.teacher.as_ref().unwrap().fio.as_ref().unwrap(),
                    group_name: schedule
                        .group
                        .as_ref()
                        .unwrap()
                        .group_number
                        .as_ref()
                        .unwrap(),
                    cabinet: schedule.cabinet.unwrap(),
                    work_type,
                    display_type: &display_type,
                });
                continue;
            }

            if schedule.schedule_move.is_some() {
                let mut work_type: Option<&model::WorkTypeEnum> = None;
                if schedule.work.is_some() {
                    work_type = schedule.work.as_ref().unwrap().work_type.as_ref();
                }
                let schedule = schedule.schedule_move.as_ref().unwrap();
                let from_lesson_schedule = schedule.from_lesson_schedule.as_ref().unwrap();
                printer::print_schedule(&mut printer::PrintScheduleArgs {
                    cfg,
                    handle: &mut handle,
                    schedule_type: printer::ScheduleType::Move,
                    lesson_id: from_lesson_schedule.lesson_id.unwrap(),
                    lesson_number: schedule.to_lesson_number.unwrap(),
                    sub_lesson_number: schedule.to_sub_lesson_number.unwrap(),
                    sub_group_number: from_lesson_schedule.sub_group.unwrap(),
                    lesson_name: from_lesson_schedule
                        .lesson
                        .as_ref()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap(),
                    teacher_fio: from_lesson_schedule
                        .teacher
                        .as_ref()
                        .unwrap()
                        .fio
                        .as_ref()
                        .unwrap(),
                    group_name: from_lesson_schedule
                        .group
                        .as_ref()
                        .unwrap()
                        .group_number
                        .as_ref()
                        .unwrap(),
                    cabinet: from_lesson_schedule.cabinet.unwrap(),
                    work_type,
                    display_type: &display_type,
                });
                continue;
            }

            if schedule.lesson_schedule.is_some() {
                let mut work_type: Option<&model::WorkTypeEnum> = None;
                if schedule.work.is_some() {
                    work_type = schedule.work.as_ref().unwrap().work_type.as_ref();
                }
                let schedule = schedule.lesson_schedule.as_ref().unwrap();
                printer::print_schedule(&mut printer::PrintScheduleArgs {
                    cfg,
                    handle: &mut handle,
                    schedule_type: printer::ScheduleType::Default,
                    lesson_id: schedule.lesson_id.unwrap(),
                    lesson_number: schedule.lesson_number.unwrap(),
                    sub_lesson_number: schedule.sub_number.unwrap(),
                    sub_group_number: schedule.sub_group.unwrap(),
                    lesson_name: schedule.lesson.as_ref().unwrap().name.as_ref().unwrap(),
                    teacher_fio: schedule.teacher.as_ref().unwrap().fio.as_ref().unwrap(),
                    group_name: schedule
                        .group
                        .as_ref()
                        .unwrap()
                        .group_number
                        .as_ref()
                        .unwrap(),
                    cabinet: schedule.cabinet.unwrap(),
                    work_type,
                    display_type: &display_type,
                });
                continue;
            }
        }
    }
    handle.flush().unwrap();

    let ans = Confirm::new("Запустить заново?")
        .with_default(false)
        .with_help_message("Если вы откажетесь произойдет выход из программы")
        .prompt();

    if ans.unwrap() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
        Box::pin(find_schedule(cfg, client, group_id, teacher_id)).await;
    }
}