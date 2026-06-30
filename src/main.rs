use std::{io, vec};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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
use ansi_to_tui::IntoText;
use crate::{graphics::decimal_numbers::new_u8_to_ascii_string, charm_styles::styles::lipstick_pop};

mod graphics;
mod charm_styles;
#[cfg(test)]
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
        let title = Line::from(" Counter App ".bold());

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

        let ascii_string = new_u8_to_ascii_string(self.counter);
        // let counter_graphic = Text::from(
        //     ascii_string.lines().map(|l| Line::from(l.to_string())).collect::<Vec<_>>()
        // );

        let style_lipstick_pop = lipstick_pop().render(&ascii_string);
        let counter_graphic  = style_lipstick_pop.into_text().unwrap();

        let counter_text = Text::from(vec![
            Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ]),
            Line::from(""),
            Line::from(""),
        ]);

        let combined = Text::from(
            counter_text.lines.into_iter()
                .chain(counter_graphic.lines.into_iter())
                .collect::<Vec<_>>()
        );

        Paragraph::new(combined)
            .centered()
            .block(block)
            .centered()
            .render(area, buf);
            }
        }

fn main() {
    ratatui::run(|terminal| App::default().run(terminal));

}
