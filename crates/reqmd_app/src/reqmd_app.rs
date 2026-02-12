use crate::{
    Error,
    command::Command,
    env::{Env, Environment},
};
use ::reqmd_core as core;
use ::std::{sync::Arc, time::Duration};

/// # ReqMD Application
///
/// This is the main entry point for using ReqMD as a library.
/// It contains the core factory and HTTP client used to work
/// with [ReqMD files] and [requests].  There is no need for
/// any kind of synchronization or locking, as all internal
/// state is behind an `Arc` and safe for concurrent use.
///
/// ```rust
/// # use ::reqmd_app::ReqmdApp;
/// # #[tokio::main]
/// # async fn main() -> Result<(), ::reqmd_app::Error> {
/// use ::reqmd_app::{providers, processors, commands::ParseRequests};
///
/// let req_md = ReqmdApp::builder()
///     .http_timeout(::std::time::Duration::from_secs(10))
///     .provider(providers::EnvProvider::default())
///     .processor(processors::EnvVarExpansion::default())
///     .processor(processors::YamlAsJson::default())
///     .build();
///
/// let list = req_md.run(ParseRequests { markdown: "" }).await?;
/// assert!(list.is_empty());
/// # Ok(()) }
/// ```
///
/// [ReqMD files]: ::reqmd_core::File
/// [requests]: ::reqmd_core::MdRequest
///
/// ---
#[derive(Clone)]
pub struct ReqmdApp<E: Env = Environment> {
    env: Arc<E>,
}

impl<E: Env> ReqmdApp<E> {
    /// # Run a Command
    ///
    /// This is the entry point for running [commands]
    /// aginst the ReqMD application, found in the
    /// `reqmd_app::commands` module.
    ///
    /// [commands]: crate::commands
    pub async fn run<C: Command>(&self, command: C) -> Result<C::Output, Error> {
        command.execute(self.env.as_ref()).await
    }
}

impl ::std::fmt::Debug for ReqmdApp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("ReqmdApp").finish()
    }
}

impl<T: Env> From<T> for ReqmdApp<T> {
    fn from(env: T) -> Self {
        Self { env: Arc::new(env) }
    }
}

#[bon::bon]
impl ReqmdApp {
    #[builder]
    pub fn new(
        #[builder(field)] factory: core::Factory,
        http_timeout: Option<Duration>,
    ) -> Self {
        let client = core::HttpClient::builder()
            .maybe_timeout(http_timeout)
            .build();

        let env = Environment::new(factory, client);
        Self::from(env)
    }
}

impl<S: reqmd_app_builder::State> ReqmdAppBuilder<S> {
    /// # Add a Default Provider
    ///
    /// Adds a [default provider] to help in the creation
    /// of http requests by supplying global defaults. Read
    /// about these providers in the documentation to get
    /// a better understanding of how they work.
    ///
    /// **Note:** you can add multiple providers, and they
    /// will be run in the order they were added.
    ///
    /// [default provider]: ::reqmd_core::DefaultProvider
    ///
    /// ---
    pub fn provider<P>(self, provider: P) -> Self
    where
        P: core::DefaultProvider,
    {
        let mut factory = self.factory;
        factory.register_default_provider(provider);
        Self { factory, ..self }
    }

    /// # Add a Factory Processor
    ///
    /// Adds a [factory processor] to process requests
    /// parsed by the application.  These processors can
    /// alter requests in various ways.  Read about these
    /// to get a better understanding of how they work.
    ///
    /// **Note:** you can add multiple processors, and they
    /// will be run in the order they were added.
    ///
    /// [factory processor]: ::reqmd_core::FactoryProcessor
    ///
    /// ---
    pub fn processor<P>(self, processor: P) -> Self
    where
        P: core::FactoryProcessor,
    {
        let mut factory = self.factory;
        factory.register_factory_processor(processor);
        Self { factory, ..self }
    }
}
