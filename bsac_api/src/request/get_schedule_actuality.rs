use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_schedule_actuality`].

On request success, this will return a [`BooleanServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleActualityRequest {
    pub last_interact: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, GetScheduleActualityRequest> {
    ///Set the value of the last_interact field.
    pub fn last_interact(
        mut self,
        last_interact: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.last_interact = Some(last_interact);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetScheduleActualityRequest> {
    type Output = httpclient::InMemoryResult<BooleanServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/schedules/actuality";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
