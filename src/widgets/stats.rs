use std::{collections::VecDeque, time::Duration};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Cell, Row, Sparkline, Table, WidgetRef},
};
use systemstat::{ByteSize, Platform, System};

const INTERVAL: u128 = 4000;
const B_TO_MB: u64 = 1048576;
const MEM_LEN: usize = 40;

pub struct Stats {
    style: Style,
    elapsed: Duration,
    sys: System,
    mem: VecDeque<u64>,
}

impl Stats {
    pub fn new(style: Style) -> Self {
        let sys = System::new();
        let elapsed = Duration::from_secs(0);
        let mem = VecDeque::with_capacity(MEM_LEN);
        Self {
            style,
            elapsed,
            sys,
            mem,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.elapsed = elapsed;
        let value = self
            .sys
            .memory()
            .map(|m| m.free.as_u64())
            .unwrap_or_default();
        if self.mem.back() != Some(&value) {
            self.mem.push_back(value);
            if self.mem.len() > MEM_LEN {
                self.mem.pop_front();
            }
        }
    }
}

impl WidgetRef for Stats {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let uptime = self.sys.uptime().unwrap_or_default().as_secs_f32();
        let (mem_free, mem_total) = self
            .sys
            .memory()
            .map(|m| (format(m.free), format(m.total)))
            .unwrap_or_default();
        let (swp_free, swp_total) = self
            .sys
            .swap()
            .map(|m| (format(m.free), format(m.total)))
            .unwrap_or_default();
        let (load1, load2, load3) = self
            .sys
            .load_average()
            .map(|l| (l.one, l.five, l.fifteen))
            .unwrap_or_default();

        let mounts = self
            .sys
            .mounts()
            .unwrap_or_default()
            .into_iter()
            .filter(|m| m.free.as_u64() > 0 && m.total.as_u64() > 0)
            .collect::<Vec<_>>();
        let mounts_len = mounts.len() as f32;
        let (mount_free, mount_total) = mounts
            .into_iter()
            .nth(
                (((self.elapsed.as_millis() % INTERVAL) as f32 / INTERVAL as f32) * mounts_len)
                    as usize,
            )
            .map(|m| (format(m.free), format(m.total)))
            .unwrap_or_default();

        let table = Table::new(
            [
                Row::new([Cell::from("[U/T]"), Cell::from(format!("{uptime:.2}"))]),
                Row::new([
                    Cell::from("[M/T]"),
                    Cell::from(format!("{mem_free} / {mem_total}")),
                ]),
                Row::new([
                    Cell::from("[S/W]"),
                    Cell::from(format!("{swp_free} / {swp_total}")),
                ]),
                Row::new([
                    Cell::from("[L/A]"),
                    Cell::from(format!("{load1:.2} / {load2:.2} / {load3:.2}")),
                ]),
                Row::new([
                    Cell::from("[D/T]"),
                    Cell::from(format!("{mount_free} / {mount_total}")),
                ]),
            ],
            [Constraint::Length(5), Constraint::Fill(1)],
        )
        .block(
            Block::bordered()
                .title(Line::from(format!(" [ST/T] {:.2} ", uptime.fract()).bold()).centered())
                .border_set(border::THICK)
                .style(self.style),
        );

        let load_chart = BarChart::default()
            .data(
                BarGroup::default().bars(&[
                    Bar::default()
                        .value((load1 * 100.) as u64)
                        .label(Line::from("L1"))
                        .text_value(format!("{load1:.2}")),
                    Bar::default()
                        .value((load2 * 100.) as u64)
                        .label(Line::from("L2"))
                        .text_value(format!("{load2:.2}")),
                    Bar::default()
                        .value((load3 * 100.) as u64)
                        .label(Line::from("L3"))
                        .text_value(format!("{load3:.2}")),
                ]),
            )
            .block(
                Block::bordered()
                    .title(Line::from(format!(" [L/A] {load1:.2} ").bold()).centered())
                    .border_set(border::THICK)
                    .style(self.style),
            )
            .bar_width(10)
            .bar_gap(4);

        let min = self.mem.iter().min().unwrap_or(&0);
        let mem_chart = Sparkline::default()
            .data(self.mem.iter().map(|v| v - min))
            .style(self.style)
            .block(
                Block::bordered()
                    .title(
                        Line::from(
                            format!(
                                " [M/U] {:.2} ",
                                self.mem.back().cloned().unwrap_or_default()
                            )
                            .bold(),
                        )
                        .centered(),
                    )
                    .border_set(border::THICK)
                    .style(self.style),
            );

        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Length(6),
        ]);
        let [stats, load, mem] = vertical.areas(area);

        table.render_ref(stats, buf);
        load_chart.render_ref(load, buf);
        mem_chart.render_ref(mem, buf);
    }
}

fn format(value: ByteSize) -> String {
    format!("{:.2}", value.as_u64() / B_TO_MB)
}
