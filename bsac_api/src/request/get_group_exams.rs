use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_group_exams`].

On request success, this will return a [`ExamListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupExamsRequest {
    pub group_id: i64,
}
impl FluentRequest<'_, GetGroupExamsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetGroupExamsRequest> {
    type Output = httpclient::InMemoryResult<ExamListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/api/exams/{group_id}", group_id = self.params.group_id);
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
