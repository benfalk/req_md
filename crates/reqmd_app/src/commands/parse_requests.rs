use crate::{Error, command::Command, env::Env};
use ::reqmd_core::MdRequestList;
use ::std::fmt;

/// # Parse Requests Command
///
/// Reads a markdown string and parses out the [requests]
/// represented within it, returning the [list] of requests.
/// This is meant to be used as a parameter to the [`ReqmdApp::run`]
/// method.
///
/// [requests]: ::reqmd_core::MdRequest
/// [list]: ::reqmd_core::MdRequestList
/// [`ReqmdApp::run`]: crate::ReqmdApp::run
/// ---
pub struct ParseRequests<S: AsRef<str>> {
    pub markdown: S,
}

impl<S: AsRef<str>> Command for ParseRequests<S> {
    type Output = MdRequestList;
    async fn execute(self, env: &impl Env) -> Result<Self::Output, Error> {
        Ok(env.parse_list(self.markdown.as_ref())?)
    }
}

impl<S: fmt::Debug + AsRef<str>> fmt::Debug for ParseRequests<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParseRequests")
            .field("markdown", &self.markdown)
            .finish()
    }
}

impl<S: Clone + AsRef<str>> Clone for ParseRequests<S> {
    fn clone(&self) -> Self {
        ParseRequests {
            markdown: self.markdown.clone(),
        }
    }
}
