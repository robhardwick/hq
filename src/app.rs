use std::{
    io,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
};

use crate::widgets::{Globe, Log, Stats};

const STYLE: Style = Style::new().fg(ratatui::style::Color::Green);

pub struct App {
    clock: SystemTime,
    horizontal: Layout,
    left: Layout,
    right: Layout,
    globe: Globe,
    stats: Stats,
    log1: Log,
    log2: Log,
}

impl App {
    pub fn new() -> Result<Self> {
        let clock = SystemTime::now();
        let horizontal = Layout::horizontal([Constraint::Length(40), Constraint::Fill(1)]);
        let left = Layout::vertical([Constraint::Length(20), Constraint::Fill(1)]);
        let right = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]);
        let globe = Globe::new(STYLE);
        let stats = Stats::new(STYLE);
        let log1 = Log::new(STYLE)?;
        let log2 = Log::new(STYLE)?;

        Ok(Self {
            clock,
            horizontal,
            left,
            right,
            globe,
            stats,
            log1,
            log2,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.handle_events()? {
            let elapsed = self.clock.elapsed()?;
            self.globe.update(elapsed);
            self.stats.update(elapsed);
            self.log1.update(elapsed);
            self.log2.update(elapsed + Duration::from_millis(1273));

            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [left, right] = self.horizontal.areas(frame.area());
        let [left_top, left_bottom] = self.left.areas(left);
        let [right_top, right_bottom] = self.right.areas(right);

        frame.render_widget(&self.globe, left_top);
        frame.render_widget(&self.stats, left_bottom);
        frame.render_widget(&self.log1, right_top);
        frame.render_widget(&self.log2, right_bottom);
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(80))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => return Ok(false),
                _ => {}
            }
        }
        Ok(true)
    }
}
