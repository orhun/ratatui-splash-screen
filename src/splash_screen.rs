use image::{DynamicImage, RgbImage};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Points};
use ratatui::widgets::Widget;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::{SplashConfig, SplashError};

/// RGB values.
type ColorTuple = (u8, u8, u8);

/// X and Y coordinates.
type Point = (f64, f64);

/// Splash screen.
#[derive(Debug, Clone)]
pub struct SplashScreen {
    /// Image that will be used for constructing the color data.
    image: DynamicImage,
    /// Color data that consists of RGB values and coordinates.
    data: HashMap<ColorTuple, Vec<Point>>,
    /// Current render step of the splash screen.
    step: i32,
    /// Number of the rendering steps.
    steps: i32,
    /// Whether to use colors.
    use_colors: bool,
}

impl SplashScreen {
    /// Constructs a new instance.
    pub fn new(config: SplashConfig) -> Result<Self, SplashError> {
        config.verify_checksum()?;
        Ok(Self {
            image: image::load_from_memory(config.image_data).map_err(|e| SplashError {
                message: e.to_string(),
            })?,
            data: HashMap::new(),
            step: config.render_steps,
            steps: config.render_steps,
            use_colors: config.use_colors,
        })
    }

    /// Returns true if the render of the splash screen is complete.
    pub fn is_rendered(&self) -> bool {
        self.step == 0
    }

    /// Returns the color data based on the rendering step.
    ///
    /// At the last render step, the image is at the darkest and has the max blurriness.
    /// On the middle, the image gets brighter and less blurry.
    /// From the start, the image is returned without any additional effects.
    ///
    /// Image is returned as grayscale if `colored` argument is `false`.
    fn get_color_data(&mut self) -> HashMap<ColorTuple, Vec<Point>> {
        if !self.is_rendered() {
            self.step -= 1;
        }
        match self.step.cmp(&(self.steps / 2)) {
            Ordering::Greater => {
                if !self.use_colors {
                    self.image = self.image.grayscale()
                }
                let value = self.step - (self.steps / 2);
                self.group_image_colors(
                    self.image
                        .brighten(value * -20)
                        .blur((value * 2) as f32)
                        .to_rgb8(),
                )
            }
            Ordering::Equal => self.group_image_colors(if self.use_colors {
                self.image.to_rgb8()
            } else {
                self.image.grayscale().to_rgb8()
            }),
            Ordering::Less => self.data.clone(),
        }
    }

    /// Groups the colors based on their RGB values and coordinates.
    fn group_image_colors(&mut self, image: RgbImage) -> HashMap<ColorTuple, Vec<Point>> {
        let mut data = HashMap::<ColorTuple, Vec<Point>>::new();
        for (x, y, color) in image.enumerate_pixels() {
            let x = f64::from(x);
            let y = f64::from(image.height().checked_sub(y + 1).unwrap_or_default());
            let color = (color[0], color[1], color[2]);
            if let Some(points) = data.get(&color) {
                let mut points = points.clone();
                points.push((x, y));
                data.insert(color, points);
            } else {
                data.insert(color, vec![(x, y)]);
            }
        }
        self.data = data.clone();
        data
    }
}

impl Widget for &mut SplashScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let data = self.get_color_data();
        Canvas::default()
            .x_bounds([0.0, (self.image.to_rgb8().width() - 1) as f64])
            .y_bounds([0.0, (self.image.to_rgb8().height() - 1) as f64])
            .paint(|p| {
                for rgb in data.keys() {
                    if let Some(coords) = data.get(rgb) {
                        p.draw(&Points {
                            coords,
                            color: Color::Rgb(rgb.0, rgb.1, rgb.2),
                        })
                    }
                }
            })
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    fn test_get_color_data() {
        let image = RgbImage::from_fn(2, 2, |x, y| {
            if (x + y) % 2 == 0 {
                Rgb([0, 0, 0])
            } else {
                Rgb([128, 64, 32])
            }
        });
        let mut splash_screen = SplashScreen {
            image: DynamicImage::ImageRgb8(image),
            data: HashMap::new(),
            step: 4,
            steps: 4,
            use_colors: false,
        };

        assert_eq!(
            [
                ((28, 28, 28), vec![(1.0, 1.0), (0.0, 0.0)]),
                ((27, 27, 27), vec![(0.0, 1.0), (1.0, 0.0,)]),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<ColorTuple, Vec<Point>>>(),
            splash_screen.get_color_data()
        );

        assert_eq!(
            [
                ((75, 75, 75), vec![(1.0, 1.0), (0.0, 0.0)]),
                ((0, 0, 0), vec![(0.0, 1.0), (1.0, 0.0,)]),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<ColorTuple, Vec<Point>>>(),
            splash_screen.get_color_data()
        );
    }
}
