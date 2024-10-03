use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_teacher_schedule_for_date`].

On request success, this will return a [`GetScheduleForOneGroupListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeacherScheduleForDateRequest {
    pub dates: Option<Vec<chrono::NaiveDate>>,
    pub teacher_id: i64,
}
impl FluentRequest<'_, GetTeacherScheduleForDateRequest> {
    ///Set the value of the dates field.
    pub fn dates(mut self, dates: Vec<chrono::NaiveDate>) -> Self {
        self.params.dates = Some(dates);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTeacherScheduleForDateRequest> {
    type Output = httpclient::InMemoryResult<GetScheduleForOneGroupListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/api/schedules/teachers/{teacher_id}/date", teacher_id = self.params
                .teacher_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
