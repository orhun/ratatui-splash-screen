use crate::SplashError;

/// Splash screen configuration.
#[derive(Clone, Copy, Debug)]
pub struct SplashConfig<'a> {
    /// Image data.
    pub image_data: &'a [u8],
    /// SHA256sum of the file.
    pub sha256sum: Option<&'a str>,
    /// Number of the rendering steps.
    pub render_steps: i32,
    /// Whether to use colors.
    pub use_colors: bool,
}

impl<'a> SplashConfig<'a> {
    /// Constructs a new instance.
    pub fn new(
        image_data: &'a [u8],
        sha256sum: Option<&'a str>,
        render_steps: i32,
        use_colors: bool,
    ) -> Self {
        Self {
            image_data,
            sha256sum,
            render_steps,
            use_colors,
        }
    }

    /// Verifies the SHA256 checksum and returns an error on mismatch.
    pub fn verify_checksum(&self) -> Result<(), SplashError> {
        if let Some(sha256sum) = self.sha256sum {
            let data_sha256sum = self.data_sha256sum();
            if sha256sum != data_sha256sum {
                return Err(SplashError {
                    message: format!(
                        "splash screen asset could not be verified\n({sha256sum} != {data_sha256sum})",
                    ),
                });
            }
        }

        Ok(())
    }

    /// Returns the SHA256 digest of the image data.
    fn data_sha256sum(&self) -> String {
        sha256::digest(self.image_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_sha256sum() -> Result<(), SplashError> {
        let config = SplashConfig {
            image_data: include_bytes!("../assets/splash.png"),
            sha256sum: Some("c692ae1f9bd4a03cb6fc74a71cb585a8d70c2eacda8ec95e26aa0d6a0670cffd"),
            render_steps: 0,
            use_colors: false,
        };
        assert!(config.verify_checksum().is_ok());

        let config = SplashConfig {
            image_data: include_bytes!("../assets/splash.png"),
            sha256sum: Some("test"),
            render_steps: 0,
            use_colors: false,
        };
        assert!(config.verify_checksum().is_err());

        let config = SplashConfig {
            image_data: &[],
            sha256sum: None,
            render_steps: 0,
            use_colors: false,
        };
        assert!(config.verify_checksum().is_ok());

        Ok(())
    }
}
