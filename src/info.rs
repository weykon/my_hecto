use crate::Terminal;

pub fn display_info(terminal: &Terminal) {
    let the_first_line_for_information = format!(
        "widht: {}, height: {}",
        terminal.size().width,
        terminal.size().height
    );
    println!("{}\r", &the_first_line_for_information);
}
