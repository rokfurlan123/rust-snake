use std::{thread, time::{Duration}};
use std::io;
use std::io::Write;
use rand::Rng;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
use crossterm::terminal::Clear;
use crossterm::cursor::MoveTo;
use crossterm::terminal::enable_raw_mode;

const WIDTH: usize = 50;
const HEIGHT: usize = 20;
const GAME_SPEED_MILLIS: u64 = 500;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {                 
    let mut grid: Vec<Vec<char>> = vec![vec![' '; HEIGHT]; WIDTH];
    let mut snake: Vec<(usize, usize)> = vec![(WIDTH / 2, HEIGHT / 2)];
    create_map(&mut grid);
    game(&mut grid, &mut snake);
}   

fn game(grid: &mut Vec<Vec<char>>, snake: &mut Vec<(usize, usize)>) {
    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();
    let mut direction = Direction::Right;

    let mut fruit_exists = false;

    'game: loop {
        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    direction = match key_event.code {
                        KeyCode::Up => Direction::Up,
                        KeyCode::Down => Direction::Down,
                        KeyCode::Left => Direction::Left,
                        KeyCode::Right => Direction::Right,
                        _ => direction,
                    };
                }
            }
        }

        if !fruit_exists {
            create_fruit(grid, snake);
            fruit_exists = true;
        }

        let steps = match direction {
            Direction::Left | Direction::Right => 2,
            _ => 1,
        };

        for _ in 0..steps {
            let (head_x, head_y) = snake[0];

            let (new_x, new_y) = match direction {
                Direction::Up    => (head_x, head_y - 1),
                Direction::Down  => (head_x, head_y + 1),
                Direction::Left  => (head_x - 1, head_y),
                Direction::Right => (head_x + 1, head_y),
            };

            if grid[new_x][new_y] == '#' || snake.contains(&(new_x, new_y)){
                //TODO
                //game over -  if we hit the wall or snake itself
                break 'game;
            }

            if grid[new_x][new_y] == '*' {
                snake.insert(0, (new_x, new_y));
                grid[new_x][new_y] = ' ';
                fruit_exists = false; 
            } else {
                snake.insert(0, (new_x, new_y));
                snake.pop(); 
            }
        }

        clear_screen(&mut stdout);
        let frame = draw_map(grid, snake);
        print!("{}", frame);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(GAME_SPEED_MILLIS));
    }
}

fn move_snake(snake: &mut Vec<(usize, usize)>, direction: Direction){
    let (head_x, head_y) = snake[0];

    let new_head: (usize, usize) = match direction {
        Direction::Up    => (head_x, head_y - 1),
        Direction::Down  => (head_x, head_y + 1),
        Direction::Left  => (head_x - 1, head_y),
        Direction::Right => (head_x + 1, head_y),
    };

    snake.insert(0, new_head);
    snake.pop();
}

fn create_fruit(grid: &mut Vec<Vec<char>>, snake: &Vec<(usize, usize)>) {
    let mut free_positions: Vec<(usize, usize)> = Vec::new();

    for x in 1..WIDTH - 1 {
        for y in 1..HEIGHT - 1 {
            if grid[x][y] == ' ' && !snake.contains(&(x, y)) {
                free_positions.push((x, y));
            }
        }
    }

    if free_positions.is_empty() {
        //TODO
        //game won
        return; 
    }

    let mut rng = rand::rng();
    let index = rng.random_range(0..free_positions.len());
    let (x, y) = free_positions[index];

    grid[x][y] = '*';
}

fn draw_map(grid: &Vec<Vec<char>>, snake: &Vec<(usize, usize)>) -> String {
    let mut frame = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if snake.contains(&(x, y)) {
                frame.push('@');
            } else {
                frame.push(grid[x][y]);
            }
        }
        frame.push('\n');
    }

    frame
}

fn create_map(grid: &mut Vec<Vec<char>>){

    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            if x == 0 || y == 0 || x == WIDTH - 1 || y == HEIGHT - 1 {
                grid[x][y] = '#';
            }
        }
    }
}

fn clear_screen(stdout: &mut io::Stdout) {
    stdout.execute(MoveTo(0, 0)).unwrap();
    stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();
}
