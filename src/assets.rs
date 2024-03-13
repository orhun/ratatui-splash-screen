use rust_embed::RustEmbed;

/// A directory of embedded assets.
#[derive(Debug, RustEmbed)]
#[folder = "assets/"]
pub struct Assets;
