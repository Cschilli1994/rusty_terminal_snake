use std::io::stdout;

use crossterm::{execute, cursor};


#[derive(Clone)]
pub struct SnakeBody {
    pub x: u16,
    pub y: u16,
}
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Snake {
    pub x: u16,
    pub y: u16,
    pub length: i16,
    pub direction: Direction,
    pub body: Vec<SnakeBody>
}


impl Snake {

    pub fn move_to(&mut self, x: u16, y: u16) {
      
        self.body.insert(0, SnakeBody { x: self.x, y: self.y });
        self.body.pop();
         
        self.x = x;
        self.y = y;
    }

    pub fn grow(&mut self, size: i16) -> i16 {
        self.length += size;
        self.length
    }
   
    pub fn render(&self) {
        let mut stdout = stdout();
        
        execute!(
            stdout,
            cursor::MoveTo(self.x, self.y)
        ).expect("Failed to move cursor!");
        println!("O");
        for body in self.body.iter() {
            execute!(
                stdout,
                cursor::MoveTo(body.x, body.y)
            ).expect("Failed to move cursor!");
            println!("o");
        }

    }


    pub fn new() -> Snake {
        Snake { 
            x: 1, 
            y: 3, 
            length: 3, 
            direction: Direction::Down, 
            body: vec![
                SnakeBody { 
                    x: 1, 
                    y: 2, 
                },
                SnakeBody { 
                    x: 1, 
                    y: 1, 
                }
            ]
        }
    }

}

