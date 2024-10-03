use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_schedule_additions`].

On request success, this will return a [`ScheduleAddListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleAdditionsRequest {
    pub group_id: i64,
}
impl FluentRequest<'_, GetScheduleAdditionsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetScheduleAdditionsRequest> {
    type Output = httpclient::InMemoryResult<ScheduleAddListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/api/schedulechange/additions/{group_id}", group_id = self.params
                .group_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
