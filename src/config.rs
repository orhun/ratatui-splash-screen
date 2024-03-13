/// Splash screen configuration.
#[derive(Clone, Copy, Debug)]
pub struct SplashConfig<'a> {
    /// Path of the image file.
    pub image_path: &'a str,
    /// SHA256sum of the file.
    pub sha256sum: Option<[u8; 32]>,
    /// Number of the rendering steps.
    pub render_steps: i32,
    /// Whether to use colors.
    pub use_colors: bool,
}

impl<'a> SplashConfig<'a> {
    /// Constructs a new instance.
    pub fn new(
        image_path: &'a str,
        sha256sum: Option<[u8; 32]>,
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
}
