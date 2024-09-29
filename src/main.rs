use chrono::{Datelike, Local, Weekday};
use colored::Colorize;
use inquire::{error::InquireError, DateSelect, Select, Text};
use openapi;
use openapi::models::WorkTypeEnum;
use serde_derive::{Deserialize, Serialize};
use std::io::{self, BufWriter, Stdout, Write};

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    sex: u8,
    sub_group: u8,
    group_id: i32,
}

enum ScheduleType {
    Add,
    Move,
    Default,
}

fn print_schedule(
    cfg: &AppConfig,
    mut handle: &mut BufWriter<Stdout>,
    schedule_type: ScheduleType,
    lesson_id: i32,
    lesson_number: i32,
    sub_lesson_number: i32,
    sub_group_number: i32,
    lesson_name: &String,
    teacher_fio: &String,
    cabinet: i32,
    work_type: Option<WorkTypeEnum>,
) {
    let mut cabinet = cabinet.to_string();
    if lesson_id != 5 && sub_group_number != 0 && sub_group_number != cfg.sub_group as i32 {
        return;
    } else if lesson_id == 5 && sub_group_number != 0 && sub_group_number != cfg.sex as i32 {
        return;
    }
    if lesson_id == 5 {
        cabinet = "Спортзал".parse().unwrap()
    }
    let hours: &str;
    match sub_lesson_number {
        0 => hours = "",
        1 => hours = "(первый час) ",
        2 => hours = "(второй час) ",
        _ => hours = "",
    }

    let schedule_type_string: &str;
    match schedule_type {
        ScheduleType::Add => schedule_type_string = "Доставленная ",
        ScheduleType::Move => schedule_type_string = "Перенесённая ",
        ScheduleType::Default => schedule_type_string = "",
    }

    let mut work_type_string: &str = "Лекция ";
    if work_type.is_some() {
        match work_type.unwrap() {
            WorkTypeEnum::Laboratory => work_type_string = "Лабораторная ",
            WorkTypeEnum::Practical => work_type_string = "Практическая ",
            WorkTypeEnum::Okr => work_type_string = "Окр ",
            WorkTypeEnum::DifferentiatedTest => work_type_string = "Дифф. Тестирование ",
        }
    }
    writeln!(
        &mut handle,
        "{} {}{}: {} \n  {}{} \n  {}: {} \n  {}: {} \n",
        lesson_number.to_string().blue(),
        "пара".blue(),
        hours.magenta(),
        lesson_name.green(),
        schedule_type_string.cyan(),
        work_type_string.white(),
        "Преподаватель".blue(),
        teacher_fio.cyan(),
        "Кабинет".blue(),
        cabinet.yellow(),
    )
    .expect("Error");
}

#[tokio::main]
async fn main() {
    let conf: openapi::apis::configuration::Configuration =
        openapi::apis::configuration::Configuration {
            base_path: "https://bsac.hlofiys.xyz".to_owned(),
            ..Default::default()
        };
    let mut cfg: AppConfig = confy::load("rust-bsac-cli", None).unwrap();
    if cfg.group_id == 0 {
        let mut user_group_id: i32 = 0;
        let mut user_group_id_found: bool = false;
        let sex: u8;
        let sub_group: u8;
        let groups = openapi::apis::groups_api::get_groups(&conf).await;
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
        if ans.unwrap() == "Девушка" {
            sex = 2;
        } else {
            sex = 1;
        }

        let options: Vec<u8> = vec![1, 2];
        let ans: Result<u8, InquireError> = Select::new("Подгруппа?", options).prompt();
        if ans.is_err() {
            panic!("{}", ans.as_ref().unwrap_err());
        }
        sub_group = ans.unwrap();

        cfg = AppConfig {
            sex,
            sub_group,
            group_id: user_group_id,
        };

        confy::store("rust-bsac-cli", None, &cfg).unwrap();
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
    let schedules = openapi::apis::schedules_api::get_schedule_for_date(
        &conf,
        cfg.group_id,
        Option::from(date),
    )
    .await;
    match schedules {
        Ok(schedules) => {
            // Достаем расписание нашей группы и клонируем временную переменную
            let schedules_data = schedules.data.unwrap().unwrap().first().unwrap().clone();
            // Достаем расписание на день
            let group_schedules_data = schedules_data.schedules.as_ref().unwrap().as_ref().unwrap();
            for schedule in group_schedules_data {
                if schedule.schedule_add.is_some() {
                    let work_type: Option<WorkTypeEnum> = None;
                    if schedule.work.is_some() {
                        schedule.work.as_ref().unwrap().work_type;
                    }
                    let schedule = schedule.schedule_add.as_ref().unwrap();
                    print_schedule(
                        &cfg,
                        &mut handle,
                        ScheduleType::Add,
                        schedule.lesson_id.unwrap(),
                        schedule.to_lesson_number.unwrap(),
                        schedule.to_sub_lesson_number.unwrap(),
                        schedule.to_sub_group.unwrap(),
                        schedule
                            .lesson
                            .as_ref()
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        schedule
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        schedule.cabinet.unwrap(),
                        work_type,
                    )
                }

                if schedule.schedule_move.is_some() {
                    let work_type: Option<WorkTypeEnum> = None;
                    if schedule.work.is_some() {
                        schedule.work.as_ref().unwrap().work_type;
                    }
                    let schedule = schedule.schedule_move.as_ref().unwrap();
                    let from_lesson_schedule = schedule.from_lesson_schedule.as_ref().unwrap();
                    print_schedule(
                        &cfg,
                        &mut handle,
                        ScheduleType::Move,
                        from_lesson_schedule.lesson_id.unwrap(),
                        schedule.to_lesson_number.unwrap(),
                        schedule.to_sub_lesson_number.unwrap(),
                        from_lesson_schedule.sub_group.unwrap(),
                        from_lesson_schedule
                            .lesson
                            .as_ref()
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        from_lesson_schedule
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        from_lesson_schedule.cabinet.unwrap(),
                        work_type,
                    )
                }

                if schedule.lesson_schedule.is_some() {
                    let work_type: Option<WorkTypeEnum> = None;
                    if schedule.work.is_some() {
                        schedule.work.as_ref().unwrap().work_type;
                    }
                    let schedule = schedule.lesson_schedule.as_ref().unwrap();
                    print_schedule(
                        &cfg,
                        &mut handle,
                        ScheduleType::Default,
                        schedule.lesson_id.unwrap(),
                        schedule.lesson_number.unwrap(),
                        schedule.sub_number.unwrap(),
                        schedule.sub_group.unwrap(),
                        schedule
                            .lesson
                            .as_ref()
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        schedule
                            .teacher
                            .as_ref()
                            .unwrap()
                            .fio
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        schedule.cabinet.unwrap(),
                        work_type,
                    )
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
