use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Playing
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
    next_cell: Option<SnakeCell>
}

impl Snake {
    fn new(start_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(start_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
            next_cell: None
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    reward_cell: usize,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, start_idx: usize) -> World {
        let size = width * width;
        let snake = Snake::new(start_idx, 2);

        World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            status: None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    fn gen_reward_cell(world_size: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        loop {
            reward_cell = rnd(world_size);
            if !snake_body.contains(&SnakeCell(reward_cell)) { break; }
        }

        reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Playing);
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Playing) => {
                let temp = self.snake.body.clone();

                match self.snake.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.snake.next_cell = None;
                    },
                    None => {
                        self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                    }
                }

                let len = self.snake.body.len();

                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.reward_cell == self.snake_head_idx() {
                    if self.snake_length() < self.size {
                        self.snake.body.push(SnakeCell(self.snake.body[1].0));
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = self.size;
                    }
                }
            },
            _ => {}
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
            Direction::Left => {
                let threshold = row * self.width - 1;
                if snake_idx - 1 == threshold {
                    SnakeCell(threshold + self.width)
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let threshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    SnakeCell(threshold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 { return; }

        self.snake.direction = direction;
        self.snake.next_cell = Some(next_cell);
    }
}