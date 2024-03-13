use rust_embed::RustEmbed;

/// A directory of embedded assets.
#[derive(Debug, RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR"]
#[include = "*.png"]
#[include = "*.jpg"]
pub struct Assets;
