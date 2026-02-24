use crate::FactoryProcessor;
use ::reqmd_ast as ast;
use ::reqmd_http as http;

/// # Server From Hostname Processor
///
/// This processor extractts the server information (host, port, scheme)
/// from the `Host` header of the HTTP request and populates the request's
/// `address` field with this information.  If the `Host` header is missing
/// or cannot be parsed, the processor leaves the request unchanged.
///
/// ---
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ServerFromHostname;

impl FactoryProcessor for ServerFromHostname {
    fn name(&self) -> &str {
        "ServerFromHostname"
    }

    fn update_request(
        &self,
        _data: &ast::HttpData,
        request: &mut http::Request,
    ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        let Some(hostname) = request.headers.first("host") else {
            return Ok(());
        };

        let Ok(address_string) = hostname.parse::<ast::AddressString>() else {
            return Ok(());
        };

        request.address = address_string.into();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::fixtures::sample_server_from_hostname_request_file as file;

    #[rstest::rstest]
    fn update_works(file: crate::File) {
        let mut factory = crate::Factory::default();
        factory.register_factory_processor(ServerFromHostname::default());
        let requests = factory.build_requests(&file).expect("parsed requests");
        let req = &requests.first().expect("at least one request").request;

        assert_eq!(req.address.port, Some(6767));
        assert_eq!(req.address.host.to_string(), "lmfao.com");
        assert_eq!(req.address.scheme, http::Scheme::Https);
    }
}
