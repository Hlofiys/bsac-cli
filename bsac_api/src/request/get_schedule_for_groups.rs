use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`bsac_apiClient::get_schedule_for_groups`].

On request success, this will return a [`GetScheduleForListOfGroupsListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleForGroupsRequest {
    pub groups_ids: Option<Vec<i64>>,
}
impl FluentRequest<'_, GetScheduleForGroupsRequest> {
    ///Set the value of the groups_ids field.
    pub fn groups_ids(mut self, groups_ids: Vec<i64>) -> Self {
        self.params.groups_ids = Some(groups_ids);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetScheduleForGroupsRequest> {
    type Output = httpclient::InMemoryResult<
        GetScheduleForListOfGroupsListServiceResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/schedules/groups";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
