use ratatui::prelude::*;
use ratatui::widgets::List;

pub struct EpisodesWidget {
    titles: Vec<String>,
}

impl Widget for EpisodesWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut lines = Vec::new();

        for title in self.titles {
            lines.push(Line::from(title).left_aligned());
        }

        Widget::render(List::new(lines), area, buf);
    }
}

impl EpisodesWidget {
    pub fn new(titles: &[String]) -> Self {
        Self {
            titles: titles.into(),
        }
    }
}
