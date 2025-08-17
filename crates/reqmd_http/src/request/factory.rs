use crate::request::Request;
use std::sync::Arc;

/// # Request Factory
///
/// This is a starting point for building HTTP requests with
/// a configured server address, method, path, query parameters.
/// Handy when you need to create multiple requests tha all
/// require the same base configuration.
///
/// For more docs see [crate::request::Request::factory].
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RequestFactory {
    request: Arc<Request>,
}

impl RequestFactory {
    #[doc(hidden)]
    pub(super) fn new(request: Request) -> Self {
        Self {
            request: Arc::new(request),
        }
    }
}
