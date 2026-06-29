use core::time;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{
    ExecutableCommand,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    style::{Color, Style, Modifier},
    DefaultTerminal, Frame,
    text::*
};

mod tests;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    last_pressed: Option<bool>,
}

impl App {
    // runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => { 
                self.decrement_counter();
                self.last_pressed = Some(false);
            }
            KeyCode::Right => { 
                self.increment_counter(); 
                self.last_pressed = Some(true);
            }
            _ => {}
        }
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());

        let blink_style = Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::RAPID_BLINK);

        let instructions = Line::from(vec![
            " Decrement ".into(),
            if self.last_pressed == Some(false) {
                Span::styled("<Left>", blink_style)
            } else {
                "<Left>".blue().bold()
            },
            " Increment ".into(),
            if self.last_pressed == Some(true) {
                Span::styled("<Right>", blink_style)
            } else {
                "<Right>".blue().bold()
            },
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ])
        ]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
        }
}

fn main() {
    ratatui::run(|terminal| App::default().run(terminal));

}
