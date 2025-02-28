use std::error::Error;
use raw_window_handle::{HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SoftBufferError<W: HasRawWindowHandle> {
    #[error(
        "The provided window returned an unsupported platform: {human_readable_window_platform_name}, {human_readable_display_platform_name}."
    )]
    UnsupportedPlatform {
        window: W,
        human_readable_window_platform_name: &'static str,
        human_readable_display_platform_name: &'static str,
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle
    },
    #[error("Platform error")]
    PlatformError(Option<String>, Option<Box<dyn Error>>)
}

#[allow(unused)] // This isn't used on all platforms
pub(crate) fn unwrap<T, E: std::error::Error + 'static, W: HasRawWindowHandle>(res: Result<T, E>, str: &str) -> Result<T, SoftBufferError<W>>{
    match res{
        Ok(t) => Ok(t),
        Err(e) => Err(SoftBufferError::PlatformError(Some(str.into()), Some(Box::new(e))))
    }
}