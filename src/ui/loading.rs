use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

pub struct LoadingLine;

impl LoadingLine {
    pub fn colored_dots() -> Line<'static> {
        Line::from(vec![
            Span::styled("‧", Style::default().fg(Color::LightGreen)).bold(),
            Span::styled("‧", Style::default().fg(Color::Green)).bold(),
            Span::styled("‧", Style::default().fg(Color::Yellow)).bold(),
            Span::styled("‧", Style::default().fg(Color::LightRed)).bold(),
            Span::styled("‧", Style::default().fg(Color::Red)).bold(),
        ])
    }
}
