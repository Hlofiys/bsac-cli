use std::io::{BufWriter, Stdout, Write};
use bsac_api::model;
use colored::Colorize;
use crate::config;

pub enum ScheduleType {
    Add,
    Move,
    Default,
}

pub enum DisplayType {
    Teacher,
    Group,
}

pub struct PrintScheduleArgs<'a> {
    pub cfg: &'a config::AppConfig,
    pub handle: &'a mut BufWriter<Stdout>,
    pub schedule_type: ScheduleType,
    pub lesson_id: i64,
    pub lesson_number: i64,
    pub sub_lesson_number: i64,
    pub sub_group_number: i64,
    pub lesson_name: &'a String,
    pub teacher_fio: &'a String,
    pub group_name: &'a String,
    pub cabinet: i64,
    pub work_type: Option<&'a model::WorkTypeEnum>,
    pub display_type: &'a DisplayType,
}

pub struct PrintPracticeExamArgs<'a> {
    pub handle: &'a mut BufWriter<Stdout>,
    pub start_time: &'a mut String,
    pub name: &'a String,
    pub end_time: &'a str,
    pub teacher_fio: Option<&'a String>,
    pub group_name: Option<&'a String>,
    pub cabinet: i64,
    pub display_type: &'a DisplayType,
}

pub fn print_schedule(print_schedule: &mut PrintScheduleArgs) {
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
        // DisplayType::TeacherAndGroup => {
        //     "Преподаватель: ".to_string()
        //         + print_schedule.teacher_fio
        //         + "\n  Группа: "
        //         + print_schedule.group_name
        // }
        // DisplayType::None => "".to_string(),
    };
    writeln!(
        print_schedule.handle,
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

pub fn print_practice_exam(args: &mut PrintPracticeExamArgs) {
    args.start_time.push('-');
    args.start_time.push_str(args.end_time);
    let group_teacher_display: String = match args.display_type {
        DisplayType::Teacher => {
            if args.teacher_fio.is_some() {
                "\n  Преподаватель: ".to_string() + args.teacher_fio.unwrap()
            } else {
                "".to_string()
            }
        }
        DisplayType::Group => "\n  Группа: ".to_string() + args.group_name.unwrap(),
        // DisplayType::TeacherAndGroup => {
        //     "\n  Преподаватель: ".to_string()
        //         + teacher_fio.unwrap()
        //         + "\n  Группа: "
        //         + group_name.unwrap()
        // }
        // DisplayType::None => "".to_string(),
    };
    writeln!(
        args.handle,
        "{}\n  {}: {}{}\n  {}: {}\n",
        args.start_time.blue(),
        "Практика".blue(),
        args.name.green(),
        group_teacher_display,
        "Кабинет".blue(),
        args.cabinet.to_string().yellow(),
    )
        .expect("Error");
}
