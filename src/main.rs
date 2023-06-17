use std::io::Write;
use std::panic;
use std::{thread, io::stdout};
use std::time::Duration;
use crossterm::event::{
    Event,
    KeyCode,
    read,
    poll, KeyEvent, self,
};
use crossterm::style::Print;
use crossterm::{
    cursor,
    terminal,
    execute, queue,
};

struct Snake {
    x: u16,
    y: u16,
    length: i16,
    direction: Direction,
    body: Vec<SnakeBody>
}

#[derive(Clone)]
struct SnakeBody {
    x: u16,
    y: u16,
}



impl Snake {

    fn move_to(&mut self, x: u16, y: u16) {
      
        self.body.insert(0, SnakeBody { x: self.x, y: self.y });
        self.body.pop();
         
        self.x = x;
        self.y = y;
    }

    fn grow(&mut self, size: i16) -> i16 {
        self.length += size;
        self.length
    }
   
    fn render(&self) {
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


    fn new() -> Snake {
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

struct Game {
    snake: Snake,
    food: Food,
    view: u16,
}

struct Food {
    x: u16,
    y: u16
}

impl Food {
    fn render(&self) {
        execute!(
            stdout(),
            cursor::MoveTo(self.x, self.y)
        ).expect("Failed to move cursor!");
        println!("🍎");
    }
}

impl Game {
    fn clean_up() {
        terminal::disable_raw_mode().expect("Failed to disable raw mode.");
        show_cursor();
        execute!(
            stdout(),
            cursor::MoveTo(1, 0),
            terminal::Clear(terminal::ClearType::All)
        ).expect("Failed cleaing terminal");
        println!("Game Over!");
    }
    fn new_food(&mut self) {
        self.food = new_food(10, 10);
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
}

fn new_game() -> Game {
    panic::set_hook(Box::new(|panic_info| {
        Game::clean_up()
    }));
    Game { snake: Snake::new(), food: new_food(4, 9), view: 60 }
}


fn new_food(x: u16, y:u16) -> Food {
    Food { x, y }
}
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    terminal::enable_raw_mode().expect("Failed to enable raw mode."); 
    println!("Starting");
    hide_cursor();
    let mut game = new_game();
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
                Direction::Left => game.snake.move_to(game.snake.x - 1, game.snake.y),
                Direction::Right => game.snake.move_to(game.snake.x + 1, game.snake.y),
                Direction::Up => game.snake.move_to(game.snake.x, game.snake.y - 1),
                Direction::Down => game.snake.move_to(game.snake.x, game.snake.y + 1),
            }

            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::All)
            ).expect("Failed cleaing terminal");
        game.render();
        sleep(60);
    }
    Game::clean_up()
   
}

fn hide_cursor() {
    print!("\x1b[?25l")
}
fn show_cursor() {
    print!("\x1b[?25h")
}


fn sleep(seconds: u64) {
    thread::sleep(Duration::from_millis(seconds));
}
