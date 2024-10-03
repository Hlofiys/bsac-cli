#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::BsacApiClient;
#[tokio::main]
async fn main() {
    let client = BsacApiClient::from_env();
    let teacher_id = 1;
    let response = client
        .get_teacher_schedule_for_date(teacher_id)
        .dates(vec![chrono::Utc::now().date_naive()])
        .await
        .unwrap();
    println!("{:#?}", response);
}
