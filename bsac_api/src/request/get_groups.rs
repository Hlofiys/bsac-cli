use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`bsac_apiClient::get_groups`].

On request success, this will return a [`GroupListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupsRequest {}
impl FluentRequest<'_, GetGroupsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetGroupsRequest> {
    type Output = httpclient::InMemoryResult<GroupListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/api/groups";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
