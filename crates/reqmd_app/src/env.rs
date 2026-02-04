use ::reqmd_core as core;
use ::reqmd_http as http;

pub struct Environment {
    factory: core::Factory,
    client: core::HttpClient,
}

impl Environment {
    pub(crate) fn new(factory: core::Factory, client: core::HttpClient) -> Self {
        Self { factory, client }
    }
}

pub trait Env {
    fn parse_list(&self, input: &str) -> Result<core::MdRequestList, core::Error>;
    fn send_request(
        &self,
        request: &core::MdRequest,
    ) -> impl Future<Output = Result<http::Response, core::Error>>;
}

impl Env for Environment {
    fn parse_list(&self, input: &str) -> Result<core::MdRequestList, core::Error> {
        let file = core::File::load(input, None)?;
        self.factory.build_requests(&file)
    }

    async fn send_request(
        &self,
        request: &core::MdRequest,
    ) -> Result<http::Response, core::Error> {
        Ok(self.client.send_md_request(request).await?)
    }
}
