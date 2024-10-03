#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::bsac_apiClient;
#[tokio::main]
async fn main() {
    let client = bsac_apiClient::from_env();
    let response = client
        .get_works_schedule()
        .group_id(1)
        .lesson_id(1)
        .teacher_id(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}
