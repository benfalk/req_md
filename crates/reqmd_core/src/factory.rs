use crate::{Error, File, MdRequest, MdRequestList};
use ::reqmd_http as http;
use ::reqmd_markdown::ast;

/// # ReqMD Request Factory
///
/// This is the main orchestration point for creating `MdRequest`
/// instances from [files] that have been loaded.  On it's own this
/// factory does very little, but, by registering [default providers]
/// and [factory processors] you can customize how requests are built.
///
/// [files]: File
/// [default providers]: DefaultProvider
/// [factory processors]: FactoryProcessor
/// ---
#[derive(Default)]
pub struct Factory {
    default_providers: Vec<Box<dyn DefaultProvider>>,
    factory_processors: Vec<Box<dyn FactoryProcessor>>,
}

impl Factory {
    /// Registers a Default Provider
    pub fn register_default_provider<P>(&mut self, provider: P)
    where
        P: DefaultProvider,
    {
        self.default_providers.push(Box::new(provider));
    }

    /// Registers a Factory Processor
    pub fn register_factory_processor<P>(&mut self, processor: P)
    where
        P: FactoryProcessor,
    {
        self.factory_processors.push(Box::new(processor));
    }

    /// Creates a list of `MdRequest` instances from the provided file
    pub fn build_requests(&self, file: &File) -> Result<MdRequestList, Error> {
        if file.document().requests.is_empty() {
            return Ok(MdRequestList::default());
        }

        let mut defaults = file.document().meta.http.clone();
        let mut requests = Vec::with_capacity(file.document().requests.len());

        for provider in &self.default_providers {
            provider
                .apply_global_defaults(&mut defaults)
                .map_err(|source| Error::DefaultProviderError {
                    provider: provider.name().to_string(),
                    source,
                })?;
        }

        let init = defaults.factory();

        for data in file.document().requests.clone() {
            let mut request = init
                .builder()
                .method(data.method)
                .path(data.path.clone())
                .multiple_headers(data.headers.clone())
                .multiple_query_params(data.query.clone())
                .body(data.body.content.clone())
                .build();

            for processor in &self.factory_processors {
                processor
                    .update_request(&data, &mut request)
                    .map_err(|source| Error::FactoryProcessor {
                        processor: processor.name().to_string(),
                        source,
                    })?;
            }

            requests.push(MdRequest {
                request,
                title: data.title.clone(),
                description: data.description.clone(),
                data: Box::new(data),
            });
        }

        Ok(MdRequestList::new(requests))
    }
}

/// # Default Provider
///
/// Provides a way to apply global [HTTP defaults] to **all** requests
/// created by a [factory].  Any number of providers can be registered
/// so take care to avoid conflicts between them in your factory setup.
///
/// **Note:** a provider must own any state it requires which is why
/// this trait requires `'static` lifetime.
///
/// [HTTP defaults]: ast::GlobalHttpDefaults
/// [factory]: Factory
///
/// ---
pub trait DefaultProvider: 'static {
    fn name(&self) -> &str;

    fn apply_global_defaults(
        &self,
        defaults: &mut ast::GlobalHttpDefaults,
    ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>>;
}

/// # Factory Processor
///
/// Processors are invoked for **each** request created from a [factory].
/// They provide a way to customize or modify [requests] based on the
/// [data] extracted from the [markdown AST].
///
/// **Note:** a processor must own any state it requires which is why
/// this trait requires `'static` lifetime.
///
/// [factory]: Factory
/// [requests]: http::Request
/// [data]: ast::HttpData
/// [markdown AST]: ast::Document
///
/// ---
pub trait FactoryProcessor: 'static {
    fn name(&self) -> &str;

    fn update_request(
        &self,
        data: &ast::HttpData,
        request: &mut http::Request,
    ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::fixtures::sample_request_file as file;

    struct TestDefaultsProvider;

    impl DefaultProvider for TestDefaultsProvider {
        fn name(&self) -> &str {
            "TestDefaultsProvider"
        }

        fn apply_global_defaults(
            &self,
            defaults: &mut ast::GlobalHttpDefaults,
        ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
            defaults.headers.add("X-Test-Default", "DefaultValue");
            Ok(())
        }
    }

    struct TestFactoryProcessor;

    impl FactoryProcessor for TestFactoryProcessor {
        fn name(&self) -> &str {
            "TestFactoryProcessor"
        }

        fn update_request(
            &self,
            data: &ast::HttpData,
            request: &mut http::Request,
        ) -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
            if let Some(title) = &data.title {
                request.headers.add("X-Test-Title", title.as_str());
            }
            Ok(())
        }
    }

    #[rstest::fixture]
    fn factory() -> Factory {
        let mut factory = Factory::default();
        factory.register_default_provider(TestDefaultsProvider);
        factory.register_factory_processor(TestFactoryProcessor);
        factory
    }

    #[rstest::rstest]
    fn factory_building(file: File, factory: Factory) {
        let requests = factory
            .build_requests(&file)
            .expect("Failed to build requests");

        assert_eq!(requests.len(), 1);

        let req = requests.first().unwrap();
        assert_eq!(req.request.method, http::Method::Get);
        assert_eq!(req.request.path.as_str(), "/api/v1/resources");
        assert_eq!(
            req.request.headers.first("X-Test-Default"),
            Some("DefaultValue")
        );
        assert_eq!(
            req.request.headers.first("X-Test-Title"),
            Some("Sample Request")
        );
        assert_eq!(
            req.request.headers.first("accept"),
            Some("application/json")
        );
        assert_eq!(req.request.body, http::RequestBody::None);
        assert_eq!(req.title.as_deref(), Some("Sample Request"));
        assert_eq!(
            req.description.as_deref(),
            Some("This is how you make a sample request for resources.")
        );
    }
}
