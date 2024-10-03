use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_practices`].

On request success, this will return a [`PracticeListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPracticesRequest {
    pub group_id: Option<i64>,
}
impl FluentRequest<'_, GetPracticesRequest> {
    ///Set the value of the group_id field.
    pub fn group_id(mut self, group_id: i64) -> Self {
        self.params.group_id = Some(group_id);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetPracticesRequest> {
    type Output = httpclient::InMemoryResult<PracticeListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/practices";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
