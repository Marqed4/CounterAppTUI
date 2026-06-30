#[cfg(test)]
use super::*;
use crate::graphics::decimal_numbers::new_u8_to_ascii_string;

#[test]
fn handle_key_event() {
    let mut app = App::default();
    app.handle_key_event(KeyCode::Right.into());
    assert_eq!(app.counter, 1);

    app.handle_key_event(KeyCode::Left.into());
    assert_eq!(app.counter, 0);

    let mut app = App::default();
    app.handle_key_event(KeyCode::Char('q').into());
    assert!(app.exit);
}

#[test]
fn printing() {
    // Run wit "cargo test -- --nocapture"
    print!("{}", new_u8_to_ascii_string(0));
    print!("{}", new_u8_to_ascii_string(60));
}