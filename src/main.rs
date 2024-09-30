use bsac_api_client::apis::configuration::Configuration;
use bsac_api_client::apis::groups_api::get_groups;
use bsac_api_client::apis::schedules_api::get_schedule_for_date;
use bsac_api_client::models::WorkTypeEnum;
use chrono::{Datelike, Local, Weekday};
use colored::Colorize;
use inquire::{error::InquireError, DateSelect, Select, Text};
use serde_derive::{Deserialize, Serialize};
use std::io::{self, BufWriter, Stdout, Write};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
struct AppConfig {
    sex: u8,
    sub_group: u8,
    group_id: i32,
}

struct PrintSchedule<'a> {
    cfg: &'a AppConfig,
    handle: &'a mut BufWriter<Stdout>,
    schedule_type: ScheduleType,
    lesson_id: i32,
    lesson_number: i32,
    sub_lesson_number: i32,
    sub_group_number: i32,
    lesson_name: &'a String,
    teacher_fio: &'a String,
    cabinet: i32,
    work_type: Option<WorkTypeEnum>,
}

enum ScheduleType {
    Add,
    Move,
    Default,
}

fn print_schedule(print_schedule: &mut PrintSchedule) {
    let mut cabinet = print_schedule.cabinet.to_string();
    if print_schedule.lesson_id != 5
        && print_schedule.sub_group_number != 0
        && print_schedule.sub_group_number != print_schedule.cfg.sub_group as i32
    {
        return;
    }
    if print_schedule.lesson_id == 5
        && print_schedule.sub_group_number != 0
        && print_schedule.sub_group_number != print_schedule.cfg.sex as i32
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
        match print_schedule.work_type.unwrap() {
            WorkTypeEnum::Laboratory => work_type_string = "Лабораторная ",
            WorkTypeEnum::Practical => work_type_string = "Практическая ",
            WorkTypeEnum::Okr => work_type_string = "Окр ",
            WorkTypeEnum::DifferentiatedTest => work_type_string = "Дифф. Тестирование ",
        }
    }
    writeln!(
        &mut print_schedule.handle,
        "{} {}{}: {} \n  {}{} \n  {}: {} \n  {}: {} \n",
        print_schedule.lesson_number.to_string().blue(),
        "пара".blue(),
        hours.magenta(),
        print_schedule.lesson_name.green(),
        schedule_type_string.cyan(),
        work_type_string.green(),
        "Преподаватель".blue(),
        print_schedule.teacher_fio.cyan(),
        "Кабинет".blue(),
        cabinet.yellow(),
    )
    .expect("Error");
}

fn print_practice(
    mut handle: &mut BufWriter<Stdout>,
    name: &String,
    start_time: &mut String,
    end_time: &str,
    teacher_fio: Option<&String>,
    cabinet: i32,
) {
    start_time.push('-');
    start_time.push_str(end_time);
    let mut fio: String = "".to_string();
    if teacher_fio.is_some() {
        fio = "\n   Преподаватель: ".to_string();
        fio.push_str(teacher_fio.unwrap());
    }
    writeln!(
        &mut handle,
        "{}\n  {}: {}{}\n  {}: {}\n",
        start_time.blue(),
        "Практика".blue(),
        name.green(),
        fio.cyan(),
        "Кабинет".blue(),
        cabinet.to_string().yellow(),
    )
    .expect("Error");
}

#[tokio::main]
async fn main() {
    let conf: Configuration = Configuration {
        base_path: "https://bsac.hlofiys.xyz".to_owned(),
        ..Default::default()
    };
    let mut cfg: AppConfig = confy::load("rust-bsac-cli", None).unwrap();
    if cfg.group_id == 0 {
        let mut user_group_id: i32 = 0;
        let mut user_group_id_found: bool = false;
        let groups = get_groups(&conf).await;
        if groups.is_err() {
            panic!("{:?}", groups.as_ref().unwrap_err());
        }
        while !user_group_id_found {
            let group_name = Text::new("Твоя группа?").prompt().unwrap();
            let groups_data = groups
                .as_ref()
                .unwrap()
                .data
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            for group in groups_data {
                if group.group_number.as_ref().unwrap().as_ref().unwrap() == &group_name {
                    user_group_id = group.id.unwrap();
                    user_group_id_found = true;
                }
            }
            if !user_group_id_found {
                eprintln!("Группа {} не найдена", group_name);
            }
        }

        let options: Vec<&str> = vec!["Девушка", "Парень"];
        let ans: Result<&str, InquireError> = Select::new("Твой пол?", options).prompt();
        if ans.is_err() {
            panic!("{}", ans.as_ref().unwrap_err());
        }
        let sex = match ans.unwrap() {
            "Девушка" => 2,
            "Парень" => 1,
            _ => 0,
        };

        let options: Vec<u8> = vec![1, 2];
        let ans: Result<u8, InquireError> = Select::new("Подгруппа?", options).prompt();
        if ans.is_err() {
            panic!("{}", ans.as_ref().unwrap_err());
        }
        let sub_group = ans.unwrap();

        cfg = AppConfig {
            sex,
            sub_group,
            group_id: user_group_id,
        };

        confy::store("rust-bsac-cli", None, cfg).unwrap();
    }
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = BufWriter::new(stdout); // optional: wrap that handle in a buffer
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

    let date = vec![date.unwrap().to_string()];
    let schedules = get_schedule_for_date(&conf, cfg.group_id, Option::from(date)).await;
    match schedules {
        Ok(schedules) => {
            // Достаем расписание нашей группы и клонируем временную переменную
            let schedules_data = schedules.data.unwrap().unwrap().first().unwrap().clone();
            // Достаем расписание на день
            let group_schedules_data = schedules_data.schedules.as_ref().unwrap().as_ref().unwrap();
            if schedules_data.practice.is_some() {
                // Если есть практика - выводим
                let practice = schedules_data.practice.unwrap();
                let mut teacher_fio: Option<&String> = None;
                if practice.teacher.is_some() {
                    teacher_fio = Some(
                        practice
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    );
                }
                print_practice(
                    &mut handle,
                    practice.name.unwrap().as_ref().unwrap(),
                    &mut practice.start_time.unwrap(),
                    &practice.end_time.unwrap(),
                    teacher_fio,
                    practice.cabinet.unwrap(),
                );
                return;
            }
            // Иначе выводим расписание на день
            for schedule in group_schedules_data {
                if schedule.schedule_add.is_some() {
                    let mut work_type: Option<WorkTypeEnum> = None;
                    if schedule.work.is_some() {
                        work_type = schedule.work.as_ref().unwrap().work_type;
                    }
                    let schedule = schedule.schedule_add.as_ref().unwrap();
                    print_schedule(&mut PrintSchedule {
                        cfg: &cfg,
                        handle: &mut handle,
                        schedule_type: ScheduleType::Add,
                        lesson_id: schedule.lesson_id.unwrap(),
                        lesson_number: schedule.to_lesson_number.unwrap(),
                        sub_lesson_number: schedule.to_sub_lesson_number.unwrap(),
                        sub_group_number: schedule.to_sub_group.unwrap(),
                        lesson_name: schedule
                            .lesson
                            .as_ref()
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        teacher_fio: schedule
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        cabinet: schedule.cabinet.unwrap(),
                        work_type,
                    });
                    continue;
                }

                if schedule.schedule_move.is_some() {
                    let mut work_type: Option<WorkTypeEnum> = None;
                    if schedule.work.is_some() {
                        work_type = schedule.work.as_ref().unwrap().work_type;
                    }
                    let schedule = schedule.schedule_move.as_ref().unwrap();
                    let from_lesson_schedule = schedule.from_lesson_schedule.as_ref().unwrap();
                    print_schedule(&mut PrintSchedule {
                        cfg: &cfg,
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
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        teacher_fio: from_lesson_schedule
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        cabinet: from_lesson_schedule.cabinet.unwrap(),
                        work_type,
                    });
                    continue;
                }

                if schedule.lesson_schedule.is_some() {
                    let mut work_type: Option<WorkTypeEnum> = None;
                    if schedule.work.is_some() {
                        work_type = schedule.work.as_ref().unwrap().work_type;
                    }
                    let schedule = schedule.lesson_schedule.as_ref().unwrap();
                    print_schedule(&mut PrintSchedule {
                        cfg: &cfg,
                        handle: &mut handle,
                        schedule_type: ScheduleType::Default,
                        lesson_id: schedule.lesson_id.unwrap(),
                        lesson_number: schedule.lesson_number.unwrap(),
                        sub_lesson_number: schedule.sub_number.unwrap(),
                        sub_group_number: schedule.sub_group.unwrap(),
                        lesson_name: schedule
                            .lesson
                            .as_ref()
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        teacher_fio: schedule
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        cabinet: schedule.cabinet.unwrap(),
                        work_type,
                    });
                    continue;
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
