use crate::utils::humanize::format_size;
use ratatui::{prelude::*, widgets::*};

pub struct Metrics {
    pub releasable_space: u64,
    pub saved_space: u64,
}

impl Widget for Metrics {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .border_style(Style::new().fg(Color::DarkGray))
            .border_type(BorderType::Rounded)
            .title(" metrics ".white().bold());

        let rows = vec![
            Row::new(vec![
                Cell::from("releasable space:"),
                Cell::from(
                    Line::from(format_size(self.releasable_space))
                        .fg(Color::Green)
                        .alignment(Alignment::Right),
                ),
            ]),
            Row::new(vec![
                Cell::from("saved space:"),
                Cell::from(
                    Line::from(format_size(self.saved_space))
                        .fg(Color::Blue)
                        .alignment(Alignment::Right),
                ),
            ]),
        ];

        let table = Table::new(rows, [Constraint::Min(20), Constraint::Length(15)]).block(block);

        Widget::render(table, area, buf);
    }
}
