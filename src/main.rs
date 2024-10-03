use bsac_api::model::{Group, GroupListServiceResponse, Teacher, TeacherListServiceResponse};
use bsac_api::{bsac_apiClient, model};
use chrono::{Datelike, Local, Weekday};
use clap::{command, Arg, ArgAction};
use colored::Colorize;
use inquire::{Confirm, DateSelect, Select, Text};
use serde_derive::{Deserialize, Serialize};
use std::io::{self, BufWriter, Error, ErrorKind, Stdout, Write};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
struct AppConfig {
    sex: u8,
    sub_group: u8,
    group_id: i64,
    set: bool,
}

struct PrintSchedule<'a> {
    cfg: &'a AppConfig,
    handle: &'a mut BufWriter<Stdout>,
    schedule_type: ScheduleType,
    lesson_id: i64,
    lesson_number: i64,
    sub_lesson_number: i64,
    sub_group_number: i64,
    lesson_name: &'a String,
    teacher_fio: &'a String,
    group_name: &'a String,
    cabinet: i64,
    work_type: Option<&'a model::WorkTypeEnum>,
    display_type: &'a DisplayType,
}

enum ScheduleType {
    Add,
    Move,
    Default,
}

enum DisplayType {
    Teacher,
    Group,
    TeacherAndGroup,
    None,
}

async fn find_group(
    client: &bsac_apiClient,
    groups: Option<GroupListServiceResponse>,
    mut group_name: Option<String>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let groups_response: GroupListServiceResponse;
    if groups.is_none() {
        groups_response = client.get_groups().await?;
    } else {
        groups_response = groups.clone().unwrap()
    }
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
    if found_groups.len() == 0 {
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

async fn find_teacher(
    client: &bsac_apiClient,
    teachers: Option<TeacherListServiceResponse>,
    mut teacher_name: Option<String>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let teachers_response: TeacherListServiceResponse;
    if teachers.is_none() {
        teachers_response = client.get_teachers().await?;
    } else {
        teachers_response = teachers.clone().unwrap()
    }
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
    if found_teachers.len() == 0 {
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

fn print_schedule(print_schedule: &mut PrintSchedule) {
    let mut cabinet = print_schedule.cabinet.to_string();
    if print_schedule.lesson_id != 5
        && print_schedule.sub_group_number != 0
        && print_schedule.sub_group_number != print_schedule.cfg.sub_group as i64
    {
        return;
    }
    if print_schedule.lesson_id == 5
        && print_schedule.sub_group_number != 0
        && print_schedule.sub_group_number != print_schedule.cfg.sex as i64
    {
        return;
    }
    if print_schedule.lesson_id == 5 {
        cabinet = "Спортзал".parse().unwrap()
    }
    let hours = match print_schedule.sub_lesson_number {
        0 => "",
        1 => "(первый час) ",
        2 => "(второй час) ",
        _ => "",
    };

    let schedule_type_string = match print_schedule.schedule_type {
        ScheduleType::Add => "Доставленная ",
        ScheduleType::Move => "Перенесённая ",
        ScheduleType::Default => "",
    };

    let mut work_type_string: &str = "Лекция ";
    if print_schedule.work_type.is_some() {
        match print_schedule.work_type.as_ref().unwrap() {
            model::WorkTypeEnum::Laboratory => work_type_string = "Лабораторная ",
            model::WorkTypeEnum::Practical => work_type_string = "Практическая ",
            model::WorkTypeEnum::Okr => work_type_string = "Окр ",
            model::WorkTypeEnum::DifferentiatedTest => work_type_string = "Дифф. Тестирование ",
        }
    }

    let group_teacher_display: String = match print_schedule.display_type {
        DisplayType::Teacher => "Преподаватель: ".to_string() + print_schedule.teacher_fio,
        DisplayType::Group => "Группа: ".to_string() + print_schedule.group_name,
        DisplayType::TeacherAndGroup => {
            "Преподаватель: ".to_string()
                + print_schedule.teacher_fio
                + "\n  Группа: "
                + print_schedule.group_name
        }
        DisplayType::None => "".to_string(),
    };
    writeln!(
        &mut print_schedule.handle,
        "{} {}{}: {} \n  {}{} \n  {} \n  {}: {} \n",
        print_schedule.lesson_number.to_string().blue(),
        "пара".blue(),
        hours.magenta(),
        print_schedule.lesson_name.green(),
        schedule_type_string.cyan(),
        work_type_string.green(),
        group_teacher_display,
        "Кабинет".blue(),
        cabinet.yellow(),
    )
    .expect("Error");
}

fn print_practice_exam(
    mut handle: &mut BufWriter<Stdout>,
    start_time: &mut String,
    name: &String,
    end_time: &str,
    teacher_fio: Option<&String>,
    group_name: Option<&String>,
    cabinet: i64,
    display_type: &DisplayType,
) {
    start_time.push('-');
    start_time.push_str(end_time);
    let group_teacher_display: String = match display_type {
        DisplayType::Teacher => {
            if teacher_fio.is_some() {
                "\n  Преподаватель: ".to_string() + teacher_fio.unwrap()
            }
            else { "".to_string() }
        },
        DisplayType::Group => "\n  Группа: ".to_string() + group_name.unwrap(),
        DisplayType::TeacherAndGroup => {
            "\n  Преподаватель: ".to_string()
                + teacher_fio.unwrap()
                + "\n  Группа: "
                + group_name.unwrap()
        }
        DisplayType::None => "".to_string(),
    };
    writeln!(
        &mut handle,
        "{}\n  {}: {}{}\n  {}: {}\n",
        start_time.blue(),
        "Практика".blue(),
        name.green(),
        group_teacher_display,
        "Кабинет".blue(),
        cabinet.to_string().yellow(),
    )
    .expect("Error");
}

async fn schedule(
    cfg: &AppConfig,
    client: &bsac_apiClient,
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
    let display_type: DisplayType;
    if group_id.is_some() {
        schedules = client
            .get_group_schedule_for_date(group_id.unwrap())
            .dates(date)
            .await
            .unwrap();
        display_type = DisplayType::Teacher;
    } else if teacher_id.is_some() {
        schedules = client
            .get_teacher_schedule_for_date(teacher_id.unwrap())
            .dates(date)
            .await
            .unwrap();
        display_type = DisplayType::Group;
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
        print_practice_exam(
            &mut handle,
            &mut practice.start_time.unwrap(),
            practice.name.as_ref().unwrap(),
            &practice.end_time.unwrap(),
            teacher_fio,
            practice.group.as_ref().unwrap().group_number.as_ref(),
            practice.cabinet.unwrap(),
            &display_type,
        );
    } else if !schedules_data.exam.as_ref().unwrap().is_empty() {
        // Если есть экзамены - выводим
        let exams = schedules_data.exam.unwrap();
        for exam in exams {
            print_practice_exam(
                &mut handle,
                &mut exam.exam_start.unwrap().time().to_string(),
                exam.lesson.unwrap().name.as_ref().unwrap(),
                "",
                exam.teacher.as_ref().unwrap().fio.as_ref(),
                exam.group.as_ref().unwrap().group_number.as_ref(),
                exam.cabinet.unwrap(),
                &display_type,
            );
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
                print_schedule(&mut PrintSchedule {
                    cfg,
                    handle: &mut handle,
                    schedule_type: ScheduleType::Add,
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
                print_schedule(&mut PrintSchedule {
                    cfg,
                    handle: &mut handle,
                    schedule_type: ScheduleType::Move,
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
                print_schedule(&mut PrintSchedule {
                    cfg,
                    handle: &mut handle,
                    schedule_type: ScheduleType::Default,
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
        Box::pin(schedule(cfg, client, group_id, teacher_id)).await;
    }
}

async fn ask_config(client: &bsac_apiClient) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let group_id;
    loop {
        group_id = find_group(client, None, None).await?;
        break;
    }

    let options: Vec<&str> = vec!["Девушка", "Парень"];
    let ans: &str = Select::new("Твой пол?", options).prompt()?;
    let sex = match ans {
        "Девушка" => 2,
        "Парень" => 1,
        _ => 0,
    };

    let options: Vec<u8> = vec![1, 2];
    let ans: u8 = Select::new("Подгруппа?", options).prompt()?;

    let cfg = AppConfig {
        sex,
        sub_group: ans,
        group_id,
        set: true,
    };

    confy::store("rust-bsac-cli", None, cfg).unwrap();
    Ok(cfg)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("group")
                .short('g')
                .long("group")
                .required(false)
                .help("Посмотреть расписание указанной группы"),
        )
        .arg(
            Arg::new("reset")
                .short('r')
                .long("reset")
                .action(ArgAction::SetTrue)
                .help("Сбросить конфиг и заполнить его заново"),
        )
        .arg(
            Arg::new("teacher")
                .short('t')
                .long("teacher")
                .required(false)
                .help("Посмотреть расписание преподавателя"),
        )
        .get_matches();
    let client = bsac_apiClient::new();
    let mut cfg: AppConfig = confy::load("rust-bsac-cli", None).unwrap();
    if !cfg.set || matches.get_flag("reset") {
        cfg = ask_config(&client).await?;
    }
    let group_name_arg = matches.get_one::<String>("group");
    let teacher_name_arg = matches.get_one::<String>("teacher");
    if teacher_name_arg.is_some() {
        let teacher_id: i64 = find_teacher(
            &client,
            None,
            Option::from(teacher_name_arg.unwrap().clone()),
        )
        .await?;
        schedule(&cfg, &client, None, Option::from(teacher_id)).await;
        return Ok(());
    }
    let group_id: i64 = match group_name_arg {
        Some(..) => {
            find_group(&client, None, Option::from(group_name_arg.unwrap().clone())).await?
        }
        None => cfg.group_id,
    };
    schedule(&cfg, &client, Option::from(group_id), None).await;
    Ok(())
}
