#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::BsacApiClient;
#[tokio::main]
async fn main() {
    let client = BsacApiClient::from_env();
    let group_id = 1;
    let response = client
        .get_group_schedule_for_date(group_id)
        .dates(vec![chrono::Utc::now().date_naive()])
        .await
        .unwrap();
    println!("{:#?}", response);
}
