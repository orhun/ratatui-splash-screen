//! A [Ratatui] widget to turn any image to a splash screen in your terminal ✨
//!
//! ![demo](https://github.com/orhun/ratatui-splash-screen/assets/24392180/e8e7570d-1c3a-4294-9b4c-e7e9b262730b)
//!
//! [![crates.io badge]][ratatui-splash-screen-crate]
//! [![docs.rs badge]][ratatui-splash-screen-docs]
//! [![license badge]](./LICENSE-MIT)
//! [![codecov.io badge]][Code Coverage]
//! [![discord badge]][Ratatui Discord]
//!
//! See the demo of [gpg-tui] for a real world example.
//!
//! ## Features
//!
//! - Turn any image (`jpg`, `png`) into a splash screen!
//! - Verifies the file integrity via checking SHA checksum (optional)
//! - Supports grayscaling
//!
//! ## Installation
//!
//! [![deps.rs badge]][Dependency Status]
//!
//! ```shell
//! cargo add ratatui ratatui-splash-screen
//! ```
//!
//! ## Usage
//!
//! Create a [`SplashConfig`] and construct a [`SplashScreen`] widget with it.
//! Then render the widget in a loop using the [`render`] function.
//! You can check if the splash screen is done rendering by calling [`is_rendered`].
//!
//! ## Examples
//!
//! ```no_run
//! use std::error::Error;
//! use std::io::stdout;
//! use std::time::Duration;
//!
//! use ratatui::prelude::*;
//! use ratatui_splash_screen::{SplashConfig, SplashScreen, SplashError};
//!
//! static SPLASH_CONFIG: SplashConfig = SplashConfig {
//!     image_data: include_bytes!("../assets/splash.png"),
//!     sha256sum: Some("c692ae1f9bd4a03cb6fc74a71cb585a8d70c2eacda8ec95e26aa0d6a0670cffd"),
//!     render_steps: 12,
//!     use_colors: true,
//! };
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // create a terminal
//!     let backend = CrosstermBackend::new(stdout());
//!     let mut terminal = Terminal::new(backend)?;
//!
//!     // render splash screen
//!     let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;
//!     while !splash_screen.is_rendered() {
//!         terminal.draw(|frame| {
//!             frame.render_widget(&mut splash_screen, frame.size());
//!         })?;
//!         std::thread::sleep(Duration::from_millis(100));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! See the full example [here](https://github.com/orhun/ratatui-splash-screen/blob/main/examples/demo.rs).
//!
//! ## Tips
//!
//! - Use small images (such as 200x200) for a better experience.
//! - You can tweak the [`render_steps`] value for smoother rendering.
//! - Run [`sha256sum(1)`] command on your system to find out the SHA value. You can set it to `None` if you don't want to check integrity.
//!
//! [ratatui-splash-screen]: https://github.com/orhun/ratatui-splash-screen
//! [ratatui-splash-screen-crate]: https://crates.io/crates/ratatui-splash-screen
//! [ratatui-splash-screen-docs]: https://docs.rs/ratatui-splash-screen
//! [ratatui]: https://ratatui.rs
//! [`sha256sum(1)`]: https://linux.die.net/man/1/sha256sum
//! [gpg-tui]: https://github.com/orhun/gpg-tui
//!
//! [`SplashConfig`]: crate::config::SplashConfig
//! [`SplashScreen`]: crate::splash_screen::SplashScreen
//! [`is_rendered`]: crate::splash_screen::SplashScreen::is_rendered
//! [`render_steps`]: crate::config::SplashConfig#structfield.render_steps
//! [`render`]: https://docs.rs/ratatui/latest/ratatui/widgets/trait.Widget.html#tymethod.render
//!
//! [crates.io badge]:
//! https://img.shields.io/crates/v/ratatui-splash-screen?style=flat&logo=Rust&color=666&labelColor=1c1c24
//! [docs.rs badge]: https://img.shields.io/docsrs/ratatui-splash-screen?logo=rust&style=flat&color=666&labelColor=1c1c24
//! [deps.rs badge]: https://deps.rs/repo/github/orhun/ratatui-splash-screen/status.svg?style=flat&color=666&labelColor=1c1c24
//! [license badge]: https://img.shields.io/crates/l/ratatui-splash-screen?style=flat&color=666&labelColor=1c1c24&logo=github
//! [codecov.io badge]: https://img.shields.io/codecov/c/github/orhun/ratatui-splash-screen?logo=codecov&style=flat&color=666&labelColor=1c1c24&logoColor=white
//! [discord badge]: https://img.shields.io/discord/1070692720437383208?label=Ratatui+Discord&logo=discord&style=flat&color=666&labelColor=1c1c24&logoColor=white
//!
//! [Dependency Status]: https://deps.rs/repo/github/orhun/ratatui-splash-screen
//! [Code Coverage]: https://app.codecov.io/gh/orhun/ratatui-splash-screen
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj

#![warn(missing_docs, clippy::unwrap_used)]

mod config;
mod error;
mod splash_screen;

pub use config::SplashConfig;
pub use error::SplashError;
pub use splash_screen::SplashScreen;
