use ratatui::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::{List, ListState};
use undersea_lib::Episode;

pub struct EpisodesWidget<'a> {
    episodes: Vec<&'a Episode>,
    selected_episode: Option<usize>,
}

impl<'a> EpisodesWidget<'a> {
    pub fn new(episodes: &'a [&Episode], selected_episode: Option<usize>) -> Self {
        Self {
            episodes: episodes.to_vec(),
            selected_episode,
        }
    }
}

impl StatefulWidget for EpisodesWidget<'_> {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState)
    where
        Self: Sized,
    {
        let mut items = Vec::new();
        for (id, episode) in self.episodes.iter().enumerate() {
            let date = episode.date().format("%Y-%m-%d %H:%M");
            let date = date.to_string();

            // Make the time of upload be allinged to the left
            // (ep length + date length + 3) - total width
            // the plus 3 is to account for the ' > ' that is insertde before
            // the highlighted episode
            let distance = area.width.saturating_sub(
                (3 + episode.title().chars().count() + date.chars().count())
                    .try_into()
                    .unwrap_or(0),
            );

            let seperator = " ".repeat(distance as usize);

            let title_style = if Some(id) == self.selected_episode {
                Style::new().green().bold()
            } else {
                Style::new().white().not_bold()
            };

            let spans = [
                Span::from(episode.title()).style(title_style),
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
