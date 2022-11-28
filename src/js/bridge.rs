use crate::error::FrError;
use anyhow::Result;
use std::fmt::Debug;
use std::future::Future;

pub async fn log_wrap_new<Fut, T, U>(
    label: &str,
    pars: T,
    handler: impl FnOnce(T) -> Fut + Send,
) -> Result<U, FrError>
where
    T: Debug + Clone,
    Fut: Future<Output = Result<U, FrError>>,
{
    log::debug!("{label}, pars: {:?}", pars);
    let res = handler(pars.clone()).await;
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}, pars: {pars:?}");
    }
    res
}

/// Wrap function call with logging
/// Used for sync functions without parameters
pub async fn log_wrap_new_sync_no_pars<T>(
    label: &str,
    handler: impl FnOnce() -> Result<T, FrError> + Send,
) -> Result<T, FrError> {
    log::debug!("{label}");
    let res = handler();
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}");
    }
    res
}
