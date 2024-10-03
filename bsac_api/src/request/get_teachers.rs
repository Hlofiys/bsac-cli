use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::bsac_apiClient;
/**You should use this struct via [`bsac_apiClient::get_teachers`].

On request success, this will return a [`TeacherListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeachersRequest {}
impl FluentRequest<'_, GetTeachersRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetTeachersRequest> {
    type Output = httpclient::InMemoryResult<TeacherListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/teachers";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
