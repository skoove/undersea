use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListState, StatefulWidget},
};
use undersea_lib::Shows;

pub struct ShowsWidget<'a> {
    shows: &'a Shows,
}

impl StatefulWidget for ShowsWidget<'_> {
    type State = ListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let mut items = Vec::new();

        for show in self.shows.shows() {
            let line = Line::from(show.name()).white();
            items.push(line);
        }

        let list = List::new(items)
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true)
            .highlight_style(Style::new().yellow().bold())
            .style(Style::new().white().not_bold())
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl<'a> ShowsWidget<'a> {
    pub fn new(shows: &'a Shows) -> ShowsWidget<'a> {
        Self { shows }
    }
}
