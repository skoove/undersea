use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::{List, ListState};
use undersea_lib::Episode;

pub struct EpisodesWidget<'a> {
    episodes: Vec<&'a Episode>,
}

impl StatefulWidget for EpisodesWidget<'_> {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState)
    where
        Self: Sized,
    {
        let mut items = Vec::new();
        for episode in self.episodes {
            let date = episode.date().format("%Y-%m-%d %H:%M");
            let date = date.to_string();

            let distance = area.width.saturating_sub(
                (3 + episode.title().chars().count() + date.chars().count())
                    .try_into()
                    .unwrap_or(0),
            );

            let seperator = " ".repeat(distance as usize);

            let spans = [
                episode.title().to_string().white(),
                Span::from(seperator).red(),
                date.gray(),
            ];

            let line = Line::default().spans(spans);
            items.push(line);
        }

        let list = List::new(items)
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true)
            .highlight_style(Style::new().yellow().bold())
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl<'a> EpisodesWidget<'a> {
    pub fn new(episodes: &'a [&Episode]) -> Self {
        Self {
            episodes: episodes.to_vec(),
        }
    }
}
