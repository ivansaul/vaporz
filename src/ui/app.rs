use crate::{
    actions::AppAction,
    events::AppEvent,
    tui::Tui,
    ui::{
        artifacts::{ArtifacsWidget, Artifacts},
        counter::{Counter, CounterWidget},
    },
};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, StatefulWidget, Widget},
};
use tokio::sync::mpsc::{self, UnboundedReceiver};

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub enum AppMode {
    Counter,
    #[default]
    Artifacts,
}

pub struct App {
    pub mode: AppMode,
    pub counter_1: Counter,
    pub artifacts: Artifacts,
    should_quit: bool,
    action_rx: UnboundedReceiver<AppAction>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Self {
            mode: AppMode::default(),
            counter_1: Counter { count: 0 },
            artifacts: Artifacts::new(action_tx.clone()),
            should_quit: false,
            action_rx: action_rx,
        }
    }
}

impl App {
    fn render(&mut self, f: &mut Frame) {
        f.render_stateful_widget(AppWidget, f.area(), self);
    }

    fn handle_event(&mut self, event: AppEvent) -> Option<AppAction> {
        match event {
            AppEvent::Quit => Some(AppAction::Quit),
            AppEvent::Tick => Some(AppAction::Tick),
            AppEvent::Render => Some(AppAction::Render),
            AppEvent::Key(key) => self.handle_key_event(key),
            _ => None,
        }
    }

    fn handle_key_event(&mut self, kev: KeyEvent) -> Option<AppAction> {
        match kev.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(AppAction::Quit),
            KeyCode::Char('1') => Some(AppAction::SwitchMode(AppMode::Counter)),
            KeyCode::Char('2') => Some(AppAction::SwitchMode(AppMode::Artifacts)),
            KeyCode::Up => Some(AppAction::KeyUp),
            KeyCode::Down => Some(AppAction::KeyDown),
            KeyCode::Enter => Some(AppAction::KeyEnter),
            KeyCode::Char('m') => Some(AppAction::KeyCharLowerM),
            KeyCode::Char('p') => Some(AppAction::KeyCharLowerP),
            KeyCode::Char('s') => Some(AppAction::KeyCharLowerS),
            _ => None,
        }
    }

    fn handle_action(&mut self, action: AppAction) -> Option<AppAction> {
        match action {
            AppAction::Quit => self.quit(),
            AppAction::SwitchMode(mode) => self.switch_mode(mode),
            AppAction::ArtifactNewRow(_)
            | AppAction::ArtifactUpdateRowRemoveStatus { id: _, removed: _ } => {
                self.artifacts.perform(action)
            }
            _ => match self.mode {
                AppMode::Counter => self.counter_1.perform(action),
                AppMode::Artifacts => self.artifacts.perform(action),
            },
        }
    }

    fn switch_mode(&mut self, mode: AppMode) -> Option<AppAction> {
        self.mode = mode;
        Some(AppAction::Render)
    }

    fn quit(&mut self) -> Option<AppAction> {
        self.should_quit = true;
        None
    }

    pub async fn run(&mut self, mut tui: Tui) -> Result<()> {
        tui.enter()?;

        self.artifacts.load_data();

        loop {
            tokio::select! {
                Some(evt) = tui.next() => {
                    let mut maybe_action = self.handle_event(evt);
                    while let Some(action) = maybe_action {
                        maybe_action = match action {
                            AppAction::Render => {
                                tui.draw(|f| self.render(f))?;
                                None
                            }
                            other => self.handle_action(other),
                        };
                    }
                }

                Some(action) = self.action_rx.recv() => {
                    let mut maybe_action = Some(action);
                    while let Some(action) = maybe_action {
                        maybe_action = match action {
                            AppAction::Render => {
                                tui.draw(|f| self.render(f))?;
                                None
                            }
                            other => self.handle_action(other),
                        };
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }
}

struct AppWidget;

impl StatefulWidget for AppWidget {
    type State = App;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let background = Block::default().style(Style::default().bg(Color::Rgb(0, 0, 0)));
        background.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(area);

        CounterWidget {
            has_focus: state.mode == AppMode::Counter,
        }
        .render(layout[0], buf, &mut state.counter_1);

        ArtifacsWidget {
            has_focus: state.mode == AppMode::Artifacts,
        }
        .render(layout[1], buf, &mut state.artifacts);
    }
}
