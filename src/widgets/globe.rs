use std::cmp;
use std::f32::consts::TAU;
use std::time::Duration;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, WidgetRef},
};

const INTERVAL: u128 = 10000;

pub struct Globe {
    globe: globe::Globe,
    elapsed: Duration,
    style: Style,
}

impl Globe {
    pub fn new(style: Style) -> Self {
        let globe = globe::GlobeConfig::default()
            .use_template(globe::GlobeTemplate::Earth)
            .with_camera(globe::CameraConfig::new(1.5, 0.5, 0.5))
            .build();
        let elapsed = Duration::from_secs(0);

        Self {
            globe,
            elapsed,
            style,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.elapsed = elapsed;
        self.globe.angle = (elapsed.as_millis() % INTERVAL) as f32 / INTERVAL as f32 * TAU;
    }
}

impl WidgetRef for Globe {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let size = cmp::min(area.width, area.height);
        let rect = Rect::new(area.left(), area.top(), size, size);

        let mut canvas = globe::Canvas::new(size * 2, size, Some((1, 1)));
        self.globe.render_on(&mut canvas);
        for (i, line) in canvas.matrix[0..size as usize].iter().enumerate() {
            buf.set_string(
                rect.left(),
                rect.top() + i as u16,
                line.iter().collect::<String>(),
                self.style,
            );
        }

        Block::bordered()
            .title(
                Line::from(
                    format!(
                        " [T/M] {:0>3} [R/T] {:.2} ",
                        self.elapsed.subsec_millis(),
                        self.globe.angle
                    )
                    .bold(),
                )
                .centered(),
            )
            .border_set(border::THICK)
            .style(self.style)
            .render_ref(area, buf);
    }
}
