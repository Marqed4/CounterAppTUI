use lipgloss::{border, prelude::*};

const EGGPLANT: &str = "#311432";
const DARK_EGGLANT: &str = "#200020";
const BURN_THISTLE: &str = "#322934";

pub fn lipstick_pop() -> Style {
    Style::new()
        .bold()
        .foreground(EGGPLANT)
        .padding((1, 2))
        .align(Position::Center)
        .border(Border::rounded())
        .background(DARK_EGGLANT)
        .width(40)
        .border_background(BURN_THISTLE)
        .border_style(border::Border::double())
    }