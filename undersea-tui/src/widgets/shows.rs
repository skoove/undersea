use ratatui::{
    style::{Style, Stylize},
    widgets::{List, ListState, StatefulWidget},
};

pub struct ShowsWidget {
    names: Vec<String>,
}

impl StatefulWidget for ShowsWidget {
    type State = ListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let list = List::new(self.names)
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true)
            .highlight_style(Style::new().yellow().bold())
            .style(Style::new().white().not_bold())
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl ShowsWidget {
    pub fn new(names: &[String]) -> Self {
        Self {
            names: names.into(),
        }
    }
}
