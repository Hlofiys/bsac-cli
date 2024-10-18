use crate::finder;
use bsac_api::BsacApiClient;
use inquire::Select;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AppConfig {
    pub sex: u8,
    pub sub_group: u8,
    pub group_id: i64,
    pub set: bool,
}

pub async fn ask_config(client: &BsacApiClient) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let group_id;
    loop {
        let group_id_res = finder::find_group(client, None, None).await;
        if group_id_res.is_ok() {
            group_id = group_id_res?;
            break;
        } else {
            eprintln!("Группа не найдена")
        }
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
