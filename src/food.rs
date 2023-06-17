use std::io::stdout;

use crossterm::{execute, cursor};




pub struct Food {
    pub x: u16,
    pub y: u16
}

impl Food {
    pub fn new(x: u16, y:u16) -> Food {
        Food { x, y }
    }


    pub fn render(&self) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x, self.y)
        ).expect("Failed to move cursor!");
        println!("🍎");
    }
}