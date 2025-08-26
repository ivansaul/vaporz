use crate::{
    actions::AppAction,
    models::{FolderInfo, Removed},
    utils::scanner::scan_current_dir,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, TableState},
};
use std::{thread::sleep, time::Duration};
use tokio::sync::mpsc::{self, UnboundedSender};

pub struct Artifacts {
    pub rows: Vec<FolderInfo>,
    pub table_state: TableState,
    pub action_tx: UnboundedSender<AppAction>,
    path_order_descending: bool,
    last_modified_order_descending: bool,
    size_order_descending: bool,
}

impl Artifacts {
    pub fn new(action_tx: UnboundedSender<AppAction>) -> Self {
        Self {
            rows: Vec::new(),
            table_state: TableState::new(),
            action_tx: action_tx,
            path_order_descending: Default::default(),
            last_modified_order_descending: Default::default(),
            size_order_descending: Default::default(),
        }
    }
}

impl Artifacts {
    pub fn perform(&mut self, action: AppAction) -> Option<AppAction> {
        match action {
            AppAction::KeyUp => {
                self.table_state.select_previous();
                Some(AppAction::Render)
            }
            AppAction::KeyDown => {
                self.table_state.select_next();
                Some(AppAction::Render)
            }
            AppAction::ArtifactNewRow(row) => {
                self.rows.push(row);
                Some(AppAction::Render)
            }
            AppAction::ArtifactUpdateRowRemoveStatus { id, removed } => {
                if let Some(row) = self.rows.iter_mut().find(|row| row.id == id) {
                    row.removed = removed;
                }
                Some(AppAction::Render)
            }
            AppAction::KeyEnter => {
                self.remove_path();
                Some(AppAction::Render)
            }
            AppAction::KeyCharLowerP => {
                self.sort_by_path();
                Some(AppAction::Render)
            }
            AppAction::KeyCharLowerM => {
                self.sort_by_last_modified();
                Some(AppAction::Render)
            }
            AppAction::KeyCharLowerS => {
                self.sort_by_size();
                Some(AppAction::Render)
            }
            _ => None,
        }
    }

    fn remove_path(&mut self) {
        if let Some(index) = self.table_state.selected() {
            let tx = self.action_tx.clone();
            let id = self.rows[index].id;
            tokio::task::spawn_blocking(move || {
                let _ = tx.send(AppAction::ArtifactUpdateRowRemoveStatus {
                    id: id,
                    removed: Removed::Progress,
                });
                sleep(Duration::from_secs(2));
                let _ = tx.send(AppAction::ArtifactUpdateRowRemoveStatus {
                    id: id,
                    removed: Removed::True,
                });
            });
        }
    }

    fn sort_by_path(&mut self) {
        self.path_order_descending = !self.path_order_descending;
        if self.path_order_descending {
            self.rows
                .sort_by_key(|row| std::cmp::Reverse(row.path_string()));
        } else {
            self.rows.sort_by_key(|row| row.path_string());
        }
    }

    fn sort_by_last_modified(&mut self) {
        self.last_modified_order_descending = !self.last_modified_order_descending;
        if self.last_modified_order_descending {
            self.rows
                .sort_by_key(|row| std::cmp::Reverse(row.last_modified()));
        } else {
            self.rows.sort_by_key(|row| row.last_modified());
        }
    }

    fn sort_by_size(&mut self) {
        self.size_order_descending = !self.size_order_descending;
        if self.size_order_descending {
            self.rows.sort_by_key(|row| std::cmp::Reverse(row.size()));
        } else {
            self.rows.sort_by_key(|row| row.size());
        }
    }

    pub fn load_data(&self) {
        let tx_action_clone = self.action_tx.clone();
        tokio::spawn(async move {
            let (tx_info, mut rx_info) = mpsc::unbounded_channel::<FolderInfo>();
            tokio::task::spawn_blocking(move || {
                scan_current_dir(tx_info);
            });
            while let Some(row) = rx_info.recv().await {
                let _ = tx_action_clone.send(AppAction::ArtifactNewRow(row));
            }
        });
    }
}

pub struct ArtifacsWidget {
    pub has_focus: bool,
}

impl StatefulWidget for ArtifacsWidget {
    type State = Artifacts;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block_border_color = match self.has_focus {
            true => Color::LightRed,
            false => Color::DarkGray,
        };

        let block = Block::bordered()
            .border_style(Style::new().fg(block_border_color))
            .border_type(BorderType::Rounded)
            .title_bottom(" ↑ Select ↓ ".blue().bold())
            .title_bottom(" Delete ↵ ".red().bold());

        let table_header = Row::new(vec![
            Line::from(vec![
                Span::styled("p", Style::default().fg(Color::Red)),
                Span::raw("ath"),
            ])
            .alignment(Alignment::Left),
            Line::from(vec![
                Span::styled("m", Style::default().fg(Color::Red)),
                Span::raw("odified"),
            ])
            .alignment(Alignment::Right),
            Line::from(vec![
                Span::styled("s", Style::default().fg(Color::Red)),
                Span::raw("ize"),
            ])
            .alignment(Alignment::Right),
        ])
        .style(Style::default().bold());

        let table_rows = state.rows.iter().map(|folder| {
            let line_path = match folder.removed {
                Removed::False => format!("{}", folder.path_string()),
                Removed::True => format!("Deleted {}", folder.path_string()),
                Removed::Progress => format!("Deleting ... {}", folder.path_string()),
            };
            let line_size = match folder.human_size() {
                Some(size) => Line::from(size)
                    .alignment(Alignment::Right)
                    .fg(Color::LightGreen),
                None => LoadingLine::default().alignment(Alignment::Right),
            };
            let line_mod = match folder.human_last_modified() {
                Some(elapsed) => Line::from(elapsed)
                    .alignment(Alignment::Right)
                    .fg(Color::LightGreen),
                None => LoadingLine::default().alignment(Alignment::Right),
            };
            Row::new(vec![
                Cell::from(line_path),
                Cell::from(line_mod),
                Cell::from(line_size),
            ])
        });

        let table_widths = [
            Constraint::Min(0),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let table = Table::new(table_rows, table_widths)
            .header(table_header)
            .block(block)
            .row_highlight_style(
                Style::default()
                    .bg(Color::Rgb(255, 123, 123))
                    .fg(Color::White)
                    .bold(),
            );

        StatefulWidget::render(table, area, buf, &mut state.table_state);
    }
}

pub struct LoadingLine;

impl LoadingLine {
    pub fn default() -> Line<'static> {
        Line::from(vec![
            Span::styled("‧", Style::default().fg(Color::LightGreen)).bold(),
            Span::styled("‧", Style::default().fg(Color::Green)).bold(),
            Span::styled("‧", Style::default().fg(Color::Yellow)).bold(),
            Span::styled("‧", Style::default().fg(Color::LightRed)).bold(),
            Span::styled("‧", Style::default().fg(Color::Red)).bold(),
        ])
    }
}
