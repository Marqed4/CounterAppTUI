use lipgloss::prelude::*;

pub fn lipstick_pop() -> Style {
    Style::new()
        .bold()
        .foreground("#ff00ff")
        .padding((2, 4))
        .align(Position::Center)
        .width(60)
    }