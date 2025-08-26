use crate::actions::AppAction;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

pub struct Counter {
    pub count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn perform(&mut self, action: AppAction) -> Option<AppAction> {
        match action {
            AppAction::KeyUp => {
                self.increment();
                Some(AppAction::Render)
            }
            AppAction::KeyDown => {
                self.decrement();
                Some(AppAction::Render)
            }
            _ => None,
        }
    }
}

pub struct CounterWidget {
    pub has_focus: bool,
}

impl StatefulWidget for CounterWidget {
    type State = Counter;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let style = match self.has_focus {
            true => Style::new().yellow(),
            false => Style::new().red(),
        };
        let paragraph = Paragraph::new(format!("Counter: {}", state.count))
            .block(Block::bordered().style(style));
        paragraph.render(area, buf);
    }
}
