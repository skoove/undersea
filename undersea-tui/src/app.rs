use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    widgets::{Block, Paragraph},
};
use style::Stylize;
use symbols::border;
use undersea_lib::Shows;

pub struct App {
    shows: Shows,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            shows: Shows::default(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" undersea ").bold().light_blue();
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK)
            .border_style(Style::new().blue());

        Paragraph::new("huhsdfi9uhsdu8i9fhb oiasduhf ou8isdhofgui8 sd")
            .block(block)
            .render(area, buf);
    }
}
