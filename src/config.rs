use crate::SplashError;

/// Splash screen configuration.
#[derive(Clone, Copy, Debug)]
pub struct SplashConfig<'a> {
    /// Path of the image file.
    pub image_path: &'a str,
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
        image_path: &'a str,
        sha256sum: Option<&'a str>,
        render_steps: i32,
        use_colors: bool,
    ) -> Self {
        Self {
            image_path,
            sha256sum,
            render_steps,
            use_colors,
        }
    }

    /// Returns the value of sha256sum as hex decoded.
    pub fn decode_sha256sum(&self) -> Result<Option<Vec<u8>>, SplashError> {
        if let Some(s) = self.sha256sum {
            let sha256sum = (0..s.len())
                .step_by(2)
                .map(|i| {
                    u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| SplashError {
                        message: format!("failed to decode hex: {e}"),
                    })
                })
                .collect::<Result<Vec<u8>, SplashError>>()?;
            Ok(Some(sha256sum))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_sha256sum() -> Result<(), SplashError> {
        let sha256sum = "c692ae1f9bd4a03cb6fc74a71cb585a8d70c2eacda8ec95e26aa0d6a0670cffd";

        let config = SplashConfig {
            image_path: "",
            sha256sum: Some(sha256sum),
            render_steps: 0,
            use_colors: false,
        };

        assert_eq!(
            hex_literal::hex!("c692ae1f9bd4a03cb6fc74a71cb585a8d70c2eacda8ec95e26aa0d6a0670cffd")
                .to_vec(),
            config.decode_sha256sum()?.unwrap()
        );

        Ok(())
    }
}
