use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`bsac_apiClient::get_works`].

On request success, this will return a [`GetWorksForAllLessonsGroupDtoListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorksRequest {
    pub group_id: i64,
}
impl FluentRequest<'_, GetWorksRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetWorksRequest> {
    type Output = httpclient::InMemoryResult<
        GetWorksForAllLessonsGroupDtoListServiceResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/api/works/{group_id}", group_id = self.params.group_id);
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
