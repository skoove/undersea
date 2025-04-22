use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    widgets::{Block, BorderType, ListState},
};
use style::Stylize;
use undersea_lib::Shows;

use crate::widgets::{episodes::EpisodesWidget, shows::ShowsWidget};

pub struct App {
    shows: Shows,
    selected_episode: Option<usize>,
    selection_state: SelectionState,
    show_list_state: ListState,
    episode_list_state: ListState,
    exit: bool,
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

        let show_list_state = ListState::default().with_selected(Some(0));

        let episode_list_state = ListState::default();

        App {
            shows,
            exit: false,
            selected_episode: None,
            selection_state: SelectionState::Shows,
            show_list_state,
            episode_list_state,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
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

        let shows_widget = ShowsWidget::new(&self.shows);
        frame.render_widget(&block, sidebar);
        frame.render_stateful_widget(
            shows_widget,
            block.inner(sidebar),
            &mut self.show_list_state,
        );

        // Main: episodes list
        let (main, footer) = if self.selected_episode.is_some() {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(5)])
                .split(main);
            (layout[0], layout[1])
        } else {
            (main, main)
        };

        if let Some(selected_episode_id) = self.selected_episode {
            let block_title = self
                .shows
                .get_show_by_index(self.show_list_state.selected().unwrap())
                .unwrap()
                .episode_by_index(selected_episode_id)
                .unwrap()
                .title();
            let block = Block::bordered()
                .title(Line::from(block_title).blue().bold())
                .border_style(Style::new().blue());

            frame.render_widget(block, footer);
        };

        let block_title = self
            .show_list_state
            .selected()
            .and_then(|index| self.shows.get_show_by_index(index))
            .map_or(" ... ", |show| show.name());

        let block = match self.selection_state {
            SelectionState::Shows => Block::bordered()
                .style(border_style)
                .title(Line::from(block_title).blue()),
            SelectionState::Episodes => Block::bordered()
                .style(border_style)
                .border_type(BorderType::Thick)
                .title(Line::from(format!(" {block_title} ")).blue().bold()),
        };

        // render block around the episodes widget
        frame.render_widget(&block, main);

        if let Some(show) = self
            .show_list_state
            .selected()
            .and_then(|index| self.shows.get_show_by_index(index))
        {
            let episodes = show.episodes();
            let episodes_widget = EpisodesWidget::new(&episodes, self.selected_episode);
            frame.render_stateful_widget(
                episodes_widget,
                block.inner(main),
                &mut self.episode_list_state,
            );
        } else {
            let no_episode_found = Line::from("no episodes!").bold().red();
            frame.render_widget(no_episode_found, block.inner(main));
        }
    }

    fn select_hovered_episode(&mut self) {
        self.selected_episode = self.episode_list_state.selected();
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
                self.episode_list_state.select(None);
            }
            KeyCode::Char('l') => {
                self.selection_state = SelectionState::Episodes;
                self.episode_list_state.select(Some(0));
            }
            _ => {}
        }

        if self.selection_state == SelectionState::Shows {
            match key_event.code {
                KeyCode::Char('j') => self.show_list_state.select_next(),
                KeyCode::Char('k') => self.show_list_state.select_previous(),
                _ => {}
            }
        }

        if self.selection_state == SelectionState::Episodes {
            match key_event.code {
                KeyCode::Char('j') => self.episode_list_state.select_next(),
                KeyCode::Char('k') => self.episode_list_state.select_previous(),
                KeyCode::Enter => self.select_hovered_episode(),
                _ => {}
            }
        }
    }
}
