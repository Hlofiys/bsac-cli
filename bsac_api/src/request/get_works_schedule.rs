use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_works_schedule`].

On request success, this will return a [`GetWorksScheduleDtoListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorksScheduleRequest {
    pub group_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub teacher_id: Option<i64>,
}
impl FluentRequest<'_, GetWorksScheduleRequest> {
    ///Set the value of the group_id field.
    pub fn group_id(mut self, group_id: i64) -> Self {
        self.params.group_id = Some(group_id);
        self
    }
    ///Set the value of the lesson_id field.
    pub fn lesson_id(mut self, lesson_id: i64) -> Self {
        self.params.lesson_id = Some(lesson_id);
        self
    }
    ///Set the value of the teacher_id field.
    pub fn teacher_id(mut self, teacher_id: i64) -> Self {
        self.params.teacher_id = Some(teacher_id);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetWorksScheduleRequest> {
    type Output = httpclient::InMemoryResult<GetWorksScheduleDtoListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/schedules/works";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
