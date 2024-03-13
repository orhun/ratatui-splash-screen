## `ratatui-splash-screen`

<!-- cargo-rdme start -->

A [Ratatui] widget to turn any image to a splash screen in your terminal ✨

![demo](https://github.com/orhun/ratatui-splash-screen/assets/24392180/e8e7570d-1c3a-4294-9b4c-e7e9b262730b)

See the demo of [gpg-tui] for a real world example.

#### Features

- Turn any image (`jpg`, `png`) into a splash screen!
- Embeds images in your binary (via [rust-embed])
- Verifies the file integrity via checking SHA checksum (optional)
- Supports grayscale

#### Installation

```shell
cargo add ratatui ratatui-splash-screen
```

#### Usage

Create a `SplashConfig` and construct a `SplashScreen` widget with it.
Then render the widget in a loop using the `render_widget` function.
You can check if the splash screen is done rendering by calling `is_rendered`.

#### Examples

```rust
use std::error::Error;
use std::io::stdout;
use std::time::Duration;

use ratatui::prelude::*;
use ratatui_splash_screen::{SplashConfig, SplashScreen, SplashError};

static SPLASH_CONFIG: SplashConfig = SplashConfig {
    image_path: "assets/splash.png",
    sha256sum: Some("c692ae1f9bd4a03cb6fc74a71cb585a8d70c2eacda8ec95e26aa0d6a0670cffd"),
    render_steps: 12,
    use_colors: true,
};

fn main() -> Result<(), Box<dyn Error>> {
    // create a terminal
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // render splash screen
    let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;
    while !splash_screen.is_rendered() {
        terminal.draw(|frame| {
            frame.render_widget(&mut splash_screen, frame.size());
        })?;
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
```

See the full example [here](https://github.com/orhun/ratatui-splash-screen/blob/main/examples/demo.rs).

#### Tips

- Use small images (such as 200x200) for a better experience.
- You can tweak the `render_steps` value for smoother rendering.
- Run [`sha256sum(1)`] command on your system to find out the SHA value. You can set it to `None` if you don't want to check integrity.

[ratatui-splash-screen]: https://github.com/orhun/ratatui-splash-screen
[ratatui]: https://ratatui.rs
[rust-embed]: https://github.com/pyrossh/rust-embed
[`sha256sum(1)`]: https://linux.die.net/man/1/sha256sum
[gpg-tui]: https://github.com/orhun/gpg-tui

<!-- cargo-rdme end -->

## Contributing

See the [contribution guidelines](CONTRIBUTING.md).

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat&logo=GitHub)](./LICENSE-APACHE)

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2024, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
