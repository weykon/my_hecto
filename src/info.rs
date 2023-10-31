use crate::Terminal;
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn display_info(terminal: &Terminal) {
    let the_first_line_for_information = format!(
        "widht: {}, height: {}",
        terminal.size().width,
        terminal.size().height
    );
    println!("{}\r", &the_first_line_for_information);
}
pub fn draw_welcome_message(terminal: &Terminal) {
    let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
    let width = terminal.size().width as usize;
    let len = welcome_message.len();
    let padding = width.saturating_sub(len) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    welcome_message = format!("~{}{}", spaces, welcome_message);
    welcome_message.truncate(width);
    println!("{}\r", welcome_message);
}
