use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_week`].

On request success, this will return a [`Int32ServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWeekRequest {}
impl FluentRequest<'_, GetWeekRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetWeekRequest> {
    type Output = httpclient::InMemoryResult<Int32ServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/schedules/week";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
