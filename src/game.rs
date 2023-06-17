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
    pub snake: Snake,
    pub food: Food,
    pub view: u16,
}


impl Game {

    pub fn new() -> Game {
        panic::set_hook(Box::new(|_| {
            Game::clean_up()
        }));
        Game { snake: Snake::new(), food: Food::new(4, 9), view: 60 }
    }

    pub fn clean_up() {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        helper::show_cursor();
        execute!(
            stdout(),
            cursor::MoveTo(1, 0),
            terminal::Clear(terminal::ClearType::All)
        ).expect("Failed cleaing terminal");
        println!("Game Over!");
    }
    
    fn render(&self) {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveTo(0, 0)
        ).expect("Failed to Move Cursor");
        
        for i in 1..self.view {
            execute!(
                stdout,
                cursor::MoveTo(i, 0)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");
        }
        for i in 1..(self.view / 2) {
            execute!(
                stdout,
                cursor::MoveTo(0, i)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");

            execute!(
                stdout,
                cursor::MoveTo(self.view, i)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");
        }
        for i in 1..self.view {
            execute!(
                stdout,
                cursor::MoveTo(i, self.view / 2)
            ).expect("Failed to Move Cursor");
            queue!(stdout, Print("#")).expect("failed to queue!");
        }
        stdout.flush().expect("Failed to Flush");
        self.food.render();
        self.snake.render();
    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode().expect("Failed to enable raw mode."); 
        println!("Starting");
        helper::hide_cursor();
        let mut direction = Direction::Down;
        loop {
            
                if poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = read().expect("Failed to read input!") {

                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                    direction = match key_event.code {
                        KeyCode::Left => Direction::Left,
                        KeyCode::Right => Direction::Right,
                        KeyCode::Up => Direction::Up,
                        KeyCode::Down => Direction::Down,
                        _ => direction

                    }
                    }
                }
                match direction {
                    Direction::Left => self.snake.move_to(self.snake.x - 1, self.snake.y),
                    Direction::Right => self.snake.move_to(self.snake.x + 1, self.snake.y),
                    Direction::Up => self.snake.move_to(self.snake.x, self.snake.y - 1),
                    Direction::Down => self.snake.move_to(self.snake.x, self.snake.y + 1),
                }

                execute!(
                    stdout(),
                    terminal::Clear(terminal::ClearType::All)
                ).expect("Failed cleaing terminal");
            self.render();
            helper::sleep(60);
        }
        Game::clean_up();
    }
}