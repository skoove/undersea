use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::text::Text;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw)?;

        if let Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) = event::read()?
        {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let colors = ["red", "green", "yellow", "blue", "magenta", "cyan", "grey"];

    let area = frame.area();
    let mut y = area.y;

    for color in colors {
        let text = Text::styled("hewwowowowo", color.parse::<Color>().unwrap());
        let widget_area = Rect::new(area.x, y, area.width, 1);
        frame.render_widget(text, widget_area);
        y += 1
    }
}
