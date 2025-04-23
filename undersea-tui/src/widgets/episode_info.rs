use ratatui::{prelude::*, widgets::Paragraph};
use undersea_lib::Episode;

pub struct EpisodeInfoWidget<'a> {
    episode: &'a Episode,
}

impl<'a> EpisodeInfoWidget<'a> {
    pub fn new(episode: &'a Episode) -> EpisodeInfoWidget<'a> {
        Self { episode }
    }
}

impl Widget for EpisodeInfoWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let date = Line::from(format!("uploaded: {}", self.episode.date())).white();
        let newline = Line::from("");
        // TODO: This HTML needs to be parsed
        let show_notes = Line::from(format!("{}", self.episode.descrpition().unwrap())).white();

        let lines = vec![date, newline, show_notes];
        Paragraph::new(lines)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .render(area, buf);
    }
}
