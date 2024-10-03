#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::bsac_apiClient;
#[tokio::main]
async fn main() {
    let client = bsac_apiClient::from_env();
    let group_id = 1;
    let response = client.get_schedule_additions(group_id).await.unwrap();
    println!("{:#?}", response);
}
