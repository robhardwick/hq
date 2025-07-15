use std::{cmp, ffi::OsStr, fs::read_to_string, time::Duration};

use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, WidgetRef, Wrap},
};
use walkdir::WalkDir;

const SPEED: f32 = 1000.;

pub struct Log {
    style: Style,
    lines: Vec<String>,
    elapsed: Duration,
}

impl Log {
    pub fn new(style: Style) -> Result<Self> {
        let lines: Vec<String> = WalkDir::new(".")
            .into_iter()
            .filter_map(|e| match e {
                Ok(e)
                    if e.file_type().is_file()
                        && e.path().extension() == Some(OsStr::new("rs")) =>
                {
                    Some(Ok(e))
                }
                Ok(_) => None,
                Err(e) => Some(Err(e)),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|e| read_to_string(e.path()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flat_map(|s| s.lines().map(|l| l.to_string()).collect::<Vec<_>>())
            .collect();

        Ok(Self {
            style,
            lines,
            elapsed: Duration::from_secs(0),
        })
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.elapsed = elapsed;
    }
}

impl WidgetRef for Log {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let elapsed: f32 = self.elapsed.as_millis() as f32;
        let offset = (((elapsed / SPEED % self.lines.len() as f32).floor() * area.height as f32)
            % self.lines.len() as f32) as usize;
        let line = (elapsed % SPEED / SPEED * area.height as f32) as usize;
        let length = cmp::min(offset + line, self.lines.len());

        let lines = self.lines[offset..length]
            .iter()
            .cloned()
            .map(Line::from)
            .collect::<Vec<Line>>();

        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(
                Block::bordered()
                    .title(
                        Line::from(
                            format!(
                                " [T/M] {:0>3} [O/S] {:0>3} [L/N] {:0>3} ",
                                self.elapsed.subsec_millis(),
                                offset,
                                line
                            )
                            .bold(),
                        )
                        .centered(),
                    )
                    .border_set(border::THICK)
                    .style(self.style),
            )
            .render_ref(area, buf);
    }
}
