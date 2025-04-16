use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    widgets::{Block, BorderType, ListState},
};
use std::ops::Index;
use style::Stylize;
use undersea_lib::Shows;

use crate::widgets::{episodes::EpisodesWidget, shows::ShowsWidget};

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

    fn selected_show_title(&self) -> String {
        if let Some(selected_show_index) = self.selected_show.selected() {
            self.shows.names().index(selected_show_index).to_string()
        } else {
            "...".to_string()
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

        let block_template = Block::bordered()
            .border_type(BorderType::Plain)
            .border_style(Style::new().blue());

        let block = block_template
            .clone()
            .title(Line::from(" shows ").bold().blue());

        let names = self.shows.names();
        let shows_widget = ShowsWidget::new(&names);
        frame.render_widget(&block, sidebar);
        frame.render_stateful_widget(shows_widget, block.inner(sidebar), &mut self.selected_show);

        let episode_titles = self
            .shows
            .get_show(self.selected_show.selected().unwrap())
            .episode_titles();

        let block = block_template
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
