#![allow(unused_imports)]
use bsac_api::model::*;
use bsac_api::BsacApiClient;
#[tokio::main]
async fn main() {
    let client = BsacApiClient::from_env();
    let response = client.get_practices().group_id(1).await.unwrap();
    println!("{:#?}", response);
}
