#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::BsacApiClient;
#[tokio::main]
async fn main() {
    let client = BsacApiClient::from_env();
    let group_id = 1;
    let response = client.get_group_exams(group_id).await.unwrap();
    println!("{:#?}", response);
}
