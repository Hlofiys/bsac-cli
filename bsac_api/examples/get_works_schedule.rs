#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::BsacApiClient;
#[tokio::main]
async fn main() {
    let client = BsacApiClient::from_env();
    let response = client
        .get_works_schedule()
        .group_id(1)
        .lesson_id(1)
        .teacher_id(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}
