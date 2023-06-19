use std::io::stdout;

use crossterm::{execute, cursor};

use rand::{thread_rng, Rng};


pub struct Food {
    pub x: u16,
    pub y: u16
}

impl Food {
    pub fn new(max_x: u16, max_y:u16) -> Food {
        let mut rng = thread_rng();
        let x = rng.gen_range(2..max_x);
        let y = rng.gen_range(2..max_y);
        Food { x, y }
    }


    pub fn render(&self) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x, self.y)
        ).expect("Failed to move cursor!");
        println!("x");
    }
}