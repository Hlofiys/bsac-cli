use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`bsac_apiClient::get_ktps`].

On request success, this will return a [`GetKtpDtoListServiceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKtpsRequest {
    pub lesson_id: i64,
}
impl FluentRequest<'_, GetKtpsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetKtpsRequest> {
    type Output = httpclient::InMemoryResult<GetKtpDtoListServiceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/api/ktp/{lesson_id}", lesson_id = self.params.lesson_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
