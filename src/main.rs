mod finder;
mod printer;
mod config;

use bsac_api::BsacApiClient;
use clap::{command, Arg, ArgAction};

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
    let client = BsacApiClient::new();
    let mut cfg: config::AppConfig = confy::load("rust-bsac-cli", None).unwrap();
    if !cfg.set || matches.get_flag("reset") {
        cfg = config::ask_config(&client).await?;
    }
    let group_name_arg = matches.get_one::<String>("group");
    let teacher_name_arg = matches.get_one::<String>("teacher");
    if teacher_name_arg.is_some() {
        let teacher_id: i64 = finder::find_teacher(
            &client,
            None,
            Option::from(teacher_name_arg.unwrap().clone()),
        )
        .await?;
        finder::find_schedule(&cfg, &client, None, Option::from(teacher_id)).await;
        return Ok(());
    }
    let group_id: i64 = match group_name_arg {
        Some(..) => {
            finder::find_group(&client, None, Option::from(group_name_arg.unwrap().clone())).await?
        }
        None => cfg.group_id,
    };
    finder::find_schedule(&cfg, &client, Option::from(group_id), None).await;
    Ok(())
}
