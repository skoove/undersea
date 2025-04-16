use std::ops::Index;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    widgets::{Block, BorderType, List, ListState},
};
use style::Stylize;
use undersea_lib::Shows;

use crate::widgets::episodes::EpisodesWidget;

pub struct App {
    shows: Shows,
    selected_show: ListState,
    exit: bool,
}

impl App {
    pub async fn new() -> Self {
        let mut shows = Shows::default();
        shows.add_multiple(super::TESTING_URLS).await.unwrap();
        let mut selected_show = ListState::default();
        selected_show.select(Some(0));
        App {
            shows,
            exit: false,
            selected_show,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn selected_show_title(&self) -> &str {
        if let Some(selected_show_index) = self.selected_show.selected() {
            self.shows.names().index(selected_show_index)
        } else {
            "..."
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Horizontal,
            Constraint::from_percentages([25, 75]),
        )
        .split(frame.area());
        let sidebar = layout[0];
        let main = layout[1];

        let block = Block::bordered()
            .border_type(BorderType::Plain)
            .border_style(Style::new().blue());

        let mut lines = Vec::new();

        for show in self.shows.names() {
            lines.push(Line::from(show).left_aligned());
        }

        let list = List::new(lines)
            .block(block.clone().title(Line::from(" shows ").blue().bold()))
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true)
            .highlight_style(Style::new().yellow().bold());

        frame.render_stateful_widget(list, sidebar, &mut self.selected_show);

        let episode_titles = self
            .shows
            .get_show(self.selected_show.selected().unwrap())
            .episode_titles();

        let block = block
            .clone()
            .title(Line::from(format!(" {} ", self.selected_show_title())).bold());

        frame.render_widget(&block, layout[1]);
        frame.render_widget(EpisodesWidget::new(&episode_titles), block.inner(main));
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('k') => self.selected_show.select_previous(),
            KeyCode::Char('j') => self.selected_show.select_next(),
            _ => {}
        }
    }
}
