use crate::{
    info::{display_info, draw_welcome_message},
    Terminal,
}; // 由于main的pub use
use std::io::{self, Write};
use termion::{event::Key, input::TermRead};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialine terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        if self.should_quit {
            Terminal::clear_screen();
            Terminal::cursor_position(&Position { x: 0, y: 0 });
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        Terminal::cursor_show();
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == 1 {
                display_info(&self.terminal);
            }
            if row == height / 3 {
                draw_welcome_message(&self.terminal);
            } else {
                println!("~\r");
            }
        }
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(()) // It says “Everything is OK, and nothing has been returned”.
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_sub(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_sub(1),
            _ => (),
        }
    }

    fn die(e: std::io::Error) {
        print!("{}", termion::clear::All);
        panic!("{}", e);
    }

    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                println!("{:?} pressing ... ", key);
                return key;
            }
        }
    }
}
