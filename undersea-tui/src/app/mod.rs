use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    widgets::{Block, BorderType, ListState},
};
use std::ops::Index;
use style::Stylize;
use undersea_lib::{Episode, Shows};

use crate::widgets::{episodes::EpisodesWidget, shows::ShowsWidget};

pub struct App {
    shows: Shows,
    selection_state: SelectionState,
    selected_show: ListState,
    highlighted_episode: ListState,
    exit: bool,
    selected_episode: Option<usize>,
}

#[derive(PartialEq, Eq)]
enum SelectionState {
    Shows,
    Episodes,
}

impl App {
    pub async fn new() -> Self {
        let mut shows = Shows::default();
        shows.add_multiple(super::TESTING_URLS).await.unwrap();

        let mut selected_show = ListState::default();
        selected_show.select(Some(0));

        let highlighted_episode = ListState::default();

        App {
            shows,
            exit: false,
            selection_state: SelectionState::Shows,
            selected_show,
            highlighted_episode,
            selected_episode: None,
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
        let border_style = Style::new().blue();

        // Sidebar: Show selection
        let block = match self.selection_state {
            SelectionState::Shows => Block::bordered()
                .style(border_style)
                .border_type(BorderType::Thick)
                .title(Line::from(" shows ").blue().bold()),
            SelectionState::Episodes => Block::bordered()
                .style(border_style)
                .title(Line::from(" shows ").blue()),
        };

        let names = self.shows.names();
        let shows_widget = ShowsWidget::new(&names);
        frame.render_widget(&block, sidebar);
        frame.render_stateful_widget(shows_widget, block.inner(sidebar), &mut self.selected_show);

        // Main: episodes list
        let block = match self.selection_state {
            SelectionState::Shows => Block::bordered()
                .style(border_style)
                .title(Line::from(self.selected_show_title()).blue()),
            SelectionState::Episodes => Block::bordered()
                .style(border_style)
                .border_type(BorderType::Thick)
                .title(
                    Line::from(format!(" {} ", self.selected_show_title()))
                        .blue()
                        .bold(),
                ),
        };

        let episode_titles = self
            .shows
            .get_show(self.selected_show.selected().unwrap())
            .episode_titles();

        let episodes_widget = EpisodesWidget::new(&episode_titles);

        frame.render_widget(&block, layout[1]);
        frame.render_stateful_widget(
            episodes_widget,
            block.inner(main),
            &mut self.highlighted_episode,
        );
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
            KeyCode::Char('h') => {
                self.selection_state = SelectionState::Shows;
                self.highlighted_episode.select(None);
            }
            KeyCode::Char('l') => {
                self.selection_state = SelectionState::Episodes;
                self.highlighted_episode.select(Some(0));
            }
            _ => {}
        }

        if self.selection_state == SelectionState::Shows {
            match key_event.code {
                KeyCode::Char('j') => self.selected_show.select_next(),
                KeyCode::Char('k') => self.selected_show.select_previous(),
                _ => {}
            }
        }

        if self.selection_state == SelectionState::Episodes {
            match key_event.code {
                KeyCode::Char('j') => self.highlighted_episode.select_next(),
                KeyCode::Char('k') => self.highlighted_episode.select_previous(),
                _ => {}
            }
        }
    }
}
