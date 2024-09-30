use bsac_api_client::{apis, models};
use chrono::{Datelike, Local, Weekday};
use clap::{command, Arg, ArgMatches};
use colored::Colorize;
use inquire::{error::InquireError, Confirm, DateSelect, Select, Text};
use serde_derive::{Deserialize, Serialize};
use std::io::{self, BufWriter, Stdout, Write};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
struct AppConfig {
    sex: u8,
    sub_group: u8,
    group_id: i32,
    set: bool,
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
    work_type: Option<models::WorkTypeEnum>,
}

enum ScheduleType {
    Add,
    Move,
    Default,
}

fn find_group(
    groups: &Result<
        models::GroupListServiceResponse,
        apis::Error<apis::groups_api::GetGroupsError>,
    >,
) -> Option<i32> {
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
            return group.id;
        };
    }
    None
}

async fn find_group_from_arg(
    group_arg: &ArgMatches,
    conf: &apis::configuration::Configuration,
) -> Option<i32> {
    let group_name_arg = group_arg.get_one::<String>("group");
    if group_name_arg.is_some() {
        let groups = apis::groups_api::get_groups(conf).await;
        if groups.is_err() {
            panic!("{:?}", groups.as_ref().unwrap_err());
        }
        let group_id_found = find_group(&groups);
        if group_id_found.is_none() {
            eprintln!("Группа не найдена");
            return None;
        }
        return group_id_found;
    }
    None
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
            models::WorkTypeEnum::Laboratory => work_type_string = "Лабораторная ",
            models::WorkTypeEnum::Practical => work_type_string = "Практическая ",
            models::WorkTypeEnum::Okr => work_type_string = "Окр ",
            models::WorkTypeEnum::DifferentiatedTest => work_type_string = "Дифф. Тестирование ",
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

fn print_practice_exam(
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

async fn schedule(cfg: &AppConfig, conf: &apis::configuration::Configuration, group_id: &i32) {
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
    let schedules =
        apis::schedules_api::get_schedule_for_date(conf, cfg.group_id, Option::from(date)).await;
    match schedules {
        Ok(schedules) => {
            let stdout = io::stdout(); // get the global stdout entity
            let mut handle = BufWriter::new(stdout); // optional: wrap that handle in a buffer
                                                     // Достаем расписание нашей группы и клонируем временную переменную
            let schedules_data = schedules.data.unwrap().unwrap().first().unwrap().clone();
            // Достаем расписание на день
            let group_schedules_data = schedules_data.schedules.as_ref().unwrap().as_ref().unwrap();
            if schedules_data.practice.is_some() {
                // Если есть практика - выводим
                let practice = schedules_data.practice.unwrap();
                let mut teacher_fio: Option<&String> = None;
                if practice.teacher.is_some() {
                    teacher_fio = Option::from(
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
                print_practice_exam(
                    &mut handle,
                    practice.name.unwrap().as_ref().unwrap(),
                    &mut practice.start_time.unwrap(),
                    &practice.end_time.unwrap(),
                    teacher_fio,
                    practice.cabinet.unwrap(),
                );
                return;
            } else if !schedules_data
                .exam
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
                .is_empty()
            {
                // Если есть экзамены - выводим
                let exams = schedules_data.exam.unwrap().unwrap();
                for exam in exams {
                    print_practice_exam(
                        &mut handle,
                        exam.lesson
                            .as_ref()
                            .unwrap()
                            .name
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                        &mut exam.exam_start.unwrap(),
                        "",
                        Option::from(
                            exam.teacher
                                .as_ref()
                                .unwrap()
                                .fio
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unwrap(),
                        ),
                        exam.cabinet.unwrap(),
                    );
                }
            } else {
                // Иначе выводим расписание на день
                for schedule in group_schedules_data {
                    if schedule.schedule_add.is_some() {
                        let mut work_type: Option<models::WorkTypeEnum> = None;
                        if schedule.work.is_some() {
                            work_type = schedule.work.as_ref().unwrap().work_type;
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
                        let mut work_type: Option<models::WorkTypeEnum> = None;
                        if schedule.work.is_some() {
                            work_type = schedule.work.as_ref().unwrap().work_type;
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
                        let mut work_type: Option<models::WorkTypeEnum> = None;
                        if schedule.work.is_some() {
                            work_type = schedule.work.as_ref().unwrap().work_type;
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
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    let ans = Confirm::new("Запустить заново?")
        .with_default(false)
        .with_help_message("Если вы откажетесь произойдет выход из программы")
        .prompt();

    if ans.unwrap() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
        Box::pin(schedule(cfg, conf, group_id)).await;
    }
}

async fn ask_config(conf: &apis::configuration::Configuration) -> AppConfig {
    let mut group_id: Option<i32> = None;
    let groups = apis::groups_api::get_groups(conf).await;
    if groups.is_err() {
        panic!("{:?}", groups.as_ref().unwrap_err());
    }
    while group_id.is_none() {
        group_id = find_group(&groups);
        if group_id.is_none() {
            eprintln!("Группа не найдена");
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

    let cfg = AppConfig {
        sex,
        sub_group,
        group_id: group_id.unwrap(),
        set: true,
    };

    confy::store("rust-bsac-cli", None, cfg).unwrap();
    cfg
}

#[tokio::main]
async fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("group")
                .short('g')
                .long("group")
                .required(false)
                .help("Укажите номер группы чтобы посмотреть ее расписание вместо вашей группы"),
        )
        .get_matches();
    let conf: apis::configuration::Configuration = apis::configuration::Configuration {
        base_path: "https://bsac.hlofiys.xyz".to_owned(),
        ..Default::default()
    };
    let mut cfg: AppConfig = confy::load("rust-bsac-cli", None).unwrap();
    if !cfg.set {
        cfg = ask_config(&conf).await;
    }
    let mut group_id = find_group_from_arg(&matches, &conf).await;
    if group_id.is_none() {
        group_id = Option::from(cfg.group_id);
    }
    schedule(&cfg, &conf, &group_id.unwrap()).await;
}
