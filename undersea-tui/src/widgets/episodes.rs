use ratatui::prelude::*;
use ratatui::widgets::{List, ListState};

pub struct EpisodesWidget {
    titles: Vec<String>,
}

impl StatefulWidget for EpisodesWidget {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState)
    where
        Self: Sized,
    {
        let list = List::new(self.titles)
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true)
            .highlight_style(Style::new().yellow().bold())
            .style(Style::new().white().not_bold())
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl EpisodesWidget {
    pub fn new(titles: &[String]) -> Self {
        Self {
            titles: titles.into(),
        }
    }
}
