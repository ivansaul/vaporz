use crate::errors::Result;
use crate::ui::loading::LoadingLine;
use crate::utils;
use crate::{
    actions::AppAction,
    models::{FolderInfo, ProcessStatus},
    utils::scanner::scan_current_dir,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, TableState},
};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

pub struct Artifacts {
    pub rows: Arc<RwLock<Vec<FolderInfo>>>,
    pub table_state: TableState,
    pub action_tx: UnboundedSender<AppAction>,
    path_order_descending: bool,
    last_modified_order_descending: bool,
    size_order_descending: bool,
}

impl Artifacts {
    pub fn new(action_tx: UnboundedSender<AppAction>) -> Self {
        Self {
            rows: Arc::new(RwLock::new(Vec::new())),
            table_state: TableState::new(),
            action_tx: action_tx,
            path_order_descending: Default::default(),
            last_modified_order_descending: Default::default(),
            size_order_descending: Default::default(),
        }
    }
}

impl Artifacts {
    pub fn handle_key_event(&mut self, kev: KeyEvent) -> Option<AppAction> {
        match kev.code {
            KeyCode::Up => Some(AppAction::ArtifactsSelectPreviousRow),
            KeyCode::Down => Some(AppAction::ArtifactsSelectNextRow),
            KeyCode::Enter => Some(AppAction::ArtifactsRemoveRow),
            KeyCode::Char('m') => Some(AppAction::ArtifactsSortByLastMod),
            KeyCode::Char('p') => Some(AppAction::ArtifactsSortByPath),
            KeyCode::Char('s') => Some(AppAction::ArtifactsSortBySize),
            _ => None,
        }
    }

    pub fn perform(&mut self, action: AppAction) -> Result<Option<AppAction>> {
        match action {
            AppAction::ArtifactsSelectPreviousRow => {
                self.table_state.select_previous();
            }
            AppAction::ArtifactsSelectNextRow => {
                self.table_state.select_next();
            }
            AppAction::ArtifactsInsertRow(row) => {
                self.insert_row(row);
            }
            AppAction::ArtifactsRemoveRow => {
                self.remove_path()?;
            }
            AppAction::ArtifactsSortByPath => {
                self.sort_by_path();
            }
            AppAction::ArtifactsSortByLastMod => {
                self.sort_by_last_modified();
            }
            AppAction::ArtifactsSortBySize => {
                self.sort_by_size();
            }
            _ => {}
        };
        Ok(Some(AppAction::Render))
    }
}

impl Artifacts {
    fn sort_by_path(&mut self) {
        self.path_order_descending = !self.path_order_descending;
        if let Ok(mut rows) = self.rows.write() {
            if self.path_order_descending {
                rows.sort_by_key(|row| std::cmp::Reverse(row.path_string()));
            } else {
                rows.sort_by_key(|row| row.path_string());
            }
        }
    }

    fn sort_by_last_modified(&mut self) {
        self.last_modified_order_descending = !self.last_modified_order_descending;
        if let Ok(mut rows) = self.rows.write() {
            if self.last_modified_order_descending {
                rows.sort_by_key(|row| std::cmp::Reverse(row.last_modified()));
            } else {
                rows.sort_by_key(|row| row.last_modified());
            }
        }
    }

    fn sort_by_size(&mut self) {
        self.size_order_descending = !self.size_order_descending;
        if let Ok(mut rows) = self.rows.write() {
            if self.size_order_descending {
                rows.sort_by_key(|row| std::cmp::Reverse(row.size()));
            } else {
                rows.sort_by_key(|row| row.size());
            }
        }
    }

    fn insert_row(&mut self, row: FolderInfo) {
        if let Ok(mut rows) = self.rows.write() {
            rows.push(row);
        }
    }

    fn remove_path(&mut self) -> Result<()> {
        if let Some(index) = self.table_state.selected() {
            let tx = self.action_tx.clone();
            let rows = Arc::clone(&self.rows);

            let (id, path, status) = {
                let data = rows.read()?;
                let row = &data[index];
                (row.id, row.path.clone(), row.removal_status)
            };

            if status != ProcessStatus::Pending {
                return Ok(());
            }

            {
                let mut data = rows.write()?;
                if let Some(row) = data.iter_mut().find(|r| r.id == id) {
                    row.removal_status = ProcessStatus::InProgress;
                    let _ = tx.send(AppAction::Render);
                }
            }

            tokio::task::spawn_blocking(move || match utils::fs::remove_path(&path) {
                Ok(_) => {
                    Self::update_removal_status(&rows, id, ProcessStatus::Completed);
                    let _ = tx.send(AppAction::Render);
                }
                Err(err) => {
                    Self::update_removal_status(&rows, id, ProcessStatus::Failed);
                    let _ = tx.send(AppAction::Error(format!("Failed to remove path: {err}")));
                    let _ = tx.send(AppAction::Render);
                }
            });
        }

        Ok(())
    }

    fn update_removal_status(
        rows: &Arc<RwLock<Vec<FolderInfo>>>,
        id: Uuid,
        new_status: ProcessStatus,
    ) {
        if let Ok(mut data) = rows.write() {
            if let Some(row) = data.iter_mut().find(|r| r.id == id) {
                row.removal_status = new_status;
            }
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
                let _ = tx_action_clone.send(AppAction::ArtifactsInsertRow(row));
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

        let table_rows = if let Ok(rows) = state.rows.read() {
            rows.iter()
                .map(|folder| {
                    let line_path = Line::from(vec![
                        Span::styled(
                            match folder.removal_status {
                                ProcessStatus::Pending => "",
                                ProcessStatus::Completed => "[Deleted] ",
                                ProcessStatus::Failed => "[Failed] ",
                                ProcessStatus::InProgress => "[Deleting ...] ",
                            },
                            Style::default()
                                .fg(match folder.removal_status {
                                    ProcessStatus::Failed => Color::Red,
                                    _ => Color::Green,
                                })
                                .italic()
                                .bold(),
                        ),
                        Span::raw(folder.path_string()),
                    ]);
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
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

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
