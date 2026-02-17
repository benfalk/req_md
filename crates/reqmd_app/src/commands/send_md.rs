use crate::{Error, command::Command, env::Env};
use ::reqmd_core::{MdRequest, http::Response};

/// # Send MD Request Command
///
/// Sends a single [markdown request] and returns whatever
/// [response] is reveived from the server.  Note that any
/// response the server sends back is concidered valid, a
/// response of 500 will be return `Ok(Response)`.  It is
/// on the caller to interpret the response as needed.
///
/// [markdown request]: ::reqmd_core::MdRequest
/// [response]: Response
/// ---
#[derive(Clone, Debug)]
pub struct SendMd<'a> {
    pub request: &'a MdRequest,
}

impl Command for SendMd<'_> {
    type Output = Response;
    async fn execute(self, env: &impl Env) -> Result<Self::Output, Error> {
        let response = env.send_request(self.request).await?;
        Ok(response)
    }
}
