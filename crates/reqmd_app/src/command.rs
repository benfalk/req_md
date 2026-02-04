use crate::{Error, env::Env};

/// # ReqMD Command
///
/// The ReqMD Application supports executing commands
/// which implement this trait.  Currently this trait is
/// sealed within the crate, but in the future it may be
/// opened up for external implementations.
///
/// ---
pub trait Command {
    type Output;
    fn execute(
        self,
        env: &impl Env,
    ) -> impl Future<Output = Result<Self::Output, Error>>;
}
