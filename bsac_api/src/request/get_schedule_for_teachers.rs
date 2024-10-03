use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`bsac_apiClient::get_schedule_for_teachers`].

On request success, this will return a [`GetScheduleForListOfGroupsListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleForTeachersRequest {
    pub teachers_ids: Option<Vec<i64>>,
}
impl FluentRequest<'_, GetScheduleForTeachersRequest> {
    ///Set the value of the teachers_ids field.
    pub fn teachers_ids(mut self, teachers_ids: Vec<i64>) -> Self {
        self.params.teachers_ids = Some(teachers_ids);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetScheduleForTeachersRequest> {
    type Output = httpclient::InMemoryResult<
        GetScheduleForListOfGroupsListServiceResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/schedules/teachers";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
