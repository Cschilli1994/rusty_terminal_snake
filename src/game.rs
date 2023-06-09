use std::io::stdout;
use std::io::Write;
use std::panic;
use std::time::Duration;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::{execute, terminal, cursor, queue, style::Print};

use crate::food::Food;
use crate::helper;
use crate::snake::Direction;
use crate::snake::Snake;




pub struct Game {
    snake: Snake,
    food: Food,
    view: (u16, u16),
    score: u64
}


impl Game {

    pub fn new() -> Game {
        let size = terminal::size().expect("Failed to get terminal size.");
        Game { snake: Snake::new(size.0 / 2, size.1 / 2), food: Food::new(size.0 - 1, size.1 - 1), view: size, score: 0 }
    }
    fn setup() {
        terminal::enable_raw_mode().expect("Failed to enable raw mode."); 
        println!("Starting");
        helper::hide_cursor();

        // In case of unexpected ending  the clean_up fn to reenable expected terminal behavior
        panic::set_hook(Box::new(|_| {
            Game::clean_up("Something went wrong!")
        }));
    }
    fn new_food(&mut self) {
        self.food = Food::new(
            self.view.0 -1,
            self.view.1 - 1
        )
    }

    fn clean_up(msg: &str) {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        helper::show_cursor();
        execute!(
            stdout(),
            cursor::MoveTo(1, 0),
            terminal::Clear(terminal::ClearType::All)
        ).expect("Failed cleaing terminal");
        println!("{}",msg);
    }
    
    fn render(&self) {
        let mut stdout = stdout();
        
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(self.view.0 / 2, 0 ),
        ).expect("Failed to Move Cursor");
       
        queue!(stdout, Print(format!( "Score : {}", self.score))).expect("failed to queue!");
        
        for i in 1..self.view.0 {
            execute!(
                stdout,
                cursor::MoveTo(i, 1)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");
        }
        for i in 1..self.view.1  {
            execute!(
                stdout,
                cursor::MoveTo(1, i)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");

            execute!(
                stdout,
                cursor::MoveTo(self.view.0 -1, i)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");
        }
        for i in 1..self.view.0  {
            execute!(
                stdout,
                cursor::MoveTo(i, self.view.1 -1)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");
        }
        stdout.flush().expect("Failed to Flush");
        self.food.render();
        self.snake.render();
     
    }

    pub fn run(&mut self) {
        Game::setup();
        let mut direction = Direction::Down;
        
        loop {
                let speed: u64 = 100 - (self.score * 5);
                if poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = read().expect("Failed to read input!") {

                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                    direction = match (key_event.code, &direction){
                        (kc, dir) if kc == KeyCode::Left && *dir != Direction::Right => Direction::Left,
                        (kc, dir) if kc == KeyCode::Right && *dir != Direction::Left => Direction::Right,
                        (kc, dir) if kc == KeyCode::Up && *dir != Direction::Down => Direction::Up,
                        (kc, dir) if kc == KeyCode::Down && *dir != Direction::Up => Direction::Down,
                        _ => direction
                    }
                    }
                }
                let can_continue = match direction {
                    Direction::Left => self.try_move_snake(self.snake.x - 1, self.snake.y),
                    Direction::Right => self.try_move_snake(self.snake.x + 1, self.snake.y),
                    Direction::Up => self.try_move_snake(self.snake.x, self.snake.y - 1),
                    Direction::Down => self.try_move_snake(self.snake.x, self.snake.y + 1),
                };
                if can_continue == false {
                    break;
                }

      
            self.render();
            helper::sleep(speed);
        }

        Game::clean_up("Game Over!");
        println!("Score: {}", self.score);
    }

    fn try_move_snake(&mut self, x: u16, y: u16) -> bool {
        if x <= 1 || x >= self.view.0 -1 || y <= 1 || y >= self.view.1 - 1 {
            return false;
        } 
        
        for snake_body in &self.snake.body {
            if snake_body.x == x && snake_body.y == y {
                return false;
            }
        };
        
        if (self.food.x == x && self.food.y == y) {
            self.score += 1;
            self.new_food();
            self.snake.grow(x, y);
            return true;
        }

        self.snake.move_to(x,y);
        return true
    }

  
}

