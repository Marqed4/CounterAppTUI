use lipgloss::prelude::*;

pub fn lipstick_pop() -> Style {
    Style::new()
        .bold()
        .foreground("#ff00ff")
        .padding((1, 2))
        .align(Position::Center)
        .border(Border::ascii())
        .width(40)
    }