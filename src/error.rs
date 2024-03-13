use std::error::Error;
use std::fmt;

/// Splash screen error.
#[derive(Debug)]
pub struct SplashError {
    /// Error message.
    pub message: String,
}

impl fmt::Display for SplashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Splash screen error: {}", self.message)
    }
}

impl Error for SplashError {}
