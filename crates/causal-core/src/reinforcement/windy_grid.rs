//! A 9 by 9 lava grid whose wind, unseen by the learner, pushes the agent as it moves.

use crate::nprandom::NpRng;
use crate::reinforcement::reward_shaping::{EpisodicEnvironment, Step};

const SIZE: usize = 9;
const HORIZON: usize = 20;
const STEP_COST: f64 = -0.1;
const LAVA_COST: f64 = -1.0;
const COIN_REWARD: f64 = 0.2;
const STILL: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// One lava column with openings at the top and bottom.
    Easy,
    /// A lava lake in the middle.
    Hard,
    /// Two corridors between lava; the upper one holds coins.
    Extreme,
    Maze,
    MazeComplex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Lava,
    Goal,
    Coin,
}

#[derive(Clone, Copy)]
enum Primitive {
    TurnLeft,
    TurnRight,
    Forward,
    Wait,
}

struct Outcome {
    position: (usize, usize),
    collected: [Option<usize>; 2],
    reward: f64,
    terminated: bool,
    truncated: bool,
}

/// A grid and the episode in progress on it.
#[derive(Clone, Debug)]
pub struct WindyGrid {
    layout: Layout,
    template: Vec<Cell>,
    cells: Vec<Cell>,
    position: (usize, usize),
    direction: usize,
    wind: usize,
    elapsed: usize,
    rng: NpRng,
}

impl WindyGrid {
    pub fn new(layout: Layout) -> Self {
        let template = cells(layout);
        Self {
            layout,
            cells: template.clone(),
            template,
            position: (1, 1),
            direction: 0,
            wind: STILL,
            elapsed: 0,
            rng: NpRng::seeded(0),
        }
    }

    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn horizon(&self) -> usize {
        HORIZON
    }

    /// The agent's direction: 0 right, 1 down, 2 left, 3 up.
    pub fn direction(&self) -> usize {
        self.direction
    }

    /// The wind for the next move: 0 right, 1 down, 2 left, 3 up, 4 none.
    pub fn wind(&self) -> usize {
        self.wind
    }

    /// Probabilities of each wind at `(x, y)`.
    pub fn wind_distribution(&self, x: usize, y: usize) -> [f64; 5] {
        wind_distribution(self.layout, x, y)
    }

    /// Cells where the agent can act: not wall, lava or goal.
    pub fn actionable(&self) -> Vec<bool> {
        self.template.iter().map(|cell| matches!(cell, Cell::Empty | Cell::Coin)).collect()
    }

    /// Values and action values of the optimal policy by value iteration, rounded to three decimals.
    pub fn optimal_values(&self) -> (Vec<f64>, Vec<f64>) {
        let fresh = &self.template;
        let states = SIZE * SIZE;
        let mut values = vec![0.0; states];
        let mut action_values = vec![0.0; states * 5];
        let mut previous = vec![0.0; states];
        loop {
            for state in 0..states {
                if !matches!(fresh[state], Cell::Empty | Cell::Coin) {
                    continue;
                }
                let position = (state / SIZE, state % SIZE);
                let winds = wind_distribution(self.layout, position.0, position.1);
                for action in 0..5 {
                    let mut weighted = 0.0;
                    for (wind, probability) in winds.iter().enumerate() {
                        let mut elapsed = 0;
                        let outcome = act(fresh, position, heading(action), wind, action, &mut elapsed);
                        weighted += probability * (outcome.reward + previous[index(outcome.position)]);
                    }
                    action_values[state * 5 + action] = weighted;
                }
                values[state] = action_values[state * 5..state * 5 + 5].iter().copied().fold(f64::NEG_INFINITY, f64::max);
            }
            let change = values.iter().zip(&previous).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
            if change < 1e-4 {
                break;
            }
            previous.copy_from_slice(&values);
        }
        (round_all(&values, 3), round_all(&action_values, 3))
    }
}

impl EpisodicEnvironment for WindyGrid {
    /// Restore the coins, place the agent on a random empty cell facing a random way and draw the wind.
    fn reset(&mut self, seed: u64) -> usize {
        self.cells.copy_from_slice(&self.template);
        let mut placement = NpRng::seeded(seed);
        self.position = loop {
            let x = placement.bounded_uint64(SIZE as u64) as usize;
            let y = placement.bounded_uint64(SIZE as u64) as usize;
            if self.cells[index((x, y))] == Cell::Empty {
                break (x, y);
            }
        };
        self.direction = placement.bounded_uint64(4) as usize;
        self.rng = NpRng::seeded(seed);
        self.wind = self.draw_wind();
        self.elapsed = 0;
        index(self.position)
    }

    /// Move up, down, left or right (0 to 3) or stay still (4).
    fn step(&mut self, action: usize) -> Step {
        if action != STILL {
            self.direction = heading(action);
        }
        let outcome = act(&self.cells, self.position, self.direction, self.wind, action, &mut self.elapsed);
        for coin in outcome.collected.into_iter().flatten() {
            self.cells[coin] = Cell::Empty;
        }
        self.position = outcome.position;
        self.wind = self.draw_wind();
        Step {
            next: index(outcome.position),
            reward: outcome.reward,
            terminated: outcome.terminated,
            truncated: outcome.truncated,
        }
    }
}

impl WindyGrid {
    fn draw_wind(&mut self) -> usize {
        let (x, y) = self.position;
        self.rng.choice_weighted(&wind_distribution(self.layout, x, y))
    }
}

fn index((x, y): (usize, usize)) -> usize {
    x * SIZE + y
}

fn heading(action: usize) -> usize {
    [3, 1, 2, 0, 0][action]
}

/// One move; no move sequence enters a cell twice, so coins are collected after it.
fn act(
    cells: &[Cell],
    start: (usize, usize),
    direction: usize,
    wind: usize,
    action: usize,
    elapsed: &mut usize,
) -> Outcome {
    let sequence = if action == STILL { &[Primitive::Wait][..] } else { blown(direction, wind) };
    let mut position = start;
    let mut collected = [None; 2];
    let mut direction = direction;
    let mut reward = 0.0;
    let mut terminated = false;
    let mut truncated = false;
    for &primitive in sequence {
        *elapsed += 1;
        let mut gained = 0.0;
        match primitive {
            Primitive::TurnLeft => direction = (direction + 3) % 4,
            Primitive::TurnRight => direction = (direction + 1) % 4,
            Primitive::Wait => {}
            Primitive::Forward => {
                let (dx, dy) = [(1, 0), (0, 1), (-1, 0), (0, -1)][direction];
                let ahead = ((position.0 as isize + dx) as usize, (position.1 as isize + dy) as usize);
                let cell = cells[index(ahead)];
                if cell != Cell::Wall {
                    position = ahead;
                    if cell == Cell::Coin {
                        gained = COIN_REWARD;
                        collected[usize::from(collected[0].is_some())] = Some(index(ahead));
                    }
                }
                terminated |= matches!(cell, Cell::Goal | Cell::Lava);
            }
        }
        truncated |= *elapsed >= HORIZON;
        if !terminated {
            reward += gained;
        }
        if terminated || truncated {
            break;
        }
    }
    reward += STEP_COST;
    if terminated && cells[index(position)] == Cell::Lava {
        reward += LAVA_COST;
    }
    Outcome { position, collected, reward, terminated, truncated }
}

fn blown(direction: usize, wind: usize) -> &'static [Primitive] {
    use Primitive::*;
    if wind == STILL {
        &[Forward]
    } else if direction == wind {
        &[Forward, Forward]
    } else if (direction + 2) % 4 == wind {
        &[Wait]
    } else if (direction + 4 - wind) % 4 == 1 {
        &[Forward, TurnLeft, Forward, TurnRight]
    } else {
        &[Forward, TurnRight, Forward, TurnLeft]
    }
}

fn cells(layout: Layout) -> Vec<Cell> {
    let mut cells = vec![Cell::Empty; SIZE * SIZE];
    for i in 0..SIZE {
        for edge in [(i, 0), (i, SIZE - 1), (0, i), (SIZE - 1, i)] {
            cells[index(edge)] = Cell::Wall;
        }
    }
    cells[index((SIZE - 2, SIZE - 2))] = Cell::Goal;
    let lake = || (2..SIZE - 2).flat_map(|x| (3..SIZE - 3).map(move |y| (x, y)));
    let (lava, coins): (Vec<(usize, usize)>, Vec<(usize, usize)>) = match layout {
        Layout::Easy => ((2..SIZE - 2).map(|y| (SIZE / 2, y)).collect(), vec![]),
        Layout::Hard => (lake().collect(), vec![]),
        Layout::Extreme => (
            lake().chain((2..SIZE - 2).flat_map(|x| [(x, 1), (x, SIZE - 3)])).collect(),
            (2..SIZE - 2).map(|x| (x, 2)).collect(),
        ),
        Layout::Maze => (maze(MAZE), vec![(SIZE - 2, SIZE - 3), (SIZE - 2, 1)]),
        Layout::MazeComplex => (maze(MAZE_COMPLEX), vec![(SIZE - 2, SIZE - 3), (3, 2), (3, 5)]),
    };
    for position in lava {
        cells[index(position)] = Cell::Lava;
    }
    for position in coins {
        cells[index(position)] = Cell::Coin;
    }
    cells
}

const MAZE: [u8; 49] = [
    0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0,
    0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
];

const MAZE_COMPLEX: [u8; 49] = [
    0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn maze(map: [u8; 49]) -> Vec<(usize, usize)> {
    (0..49).filter(|&i| map[i] == 1).map(|i| (i % 7 + 1, i / 7 + 1)).collect()
}

fn wind_distribution(layout: Layout, x: usize, y: usize) -> [f64; 5] {
    const CALM: [f64; 5] = [0.15, 0.15, 0.15, 0.15, 0.4];
    match layout {
        Layout::Easy | Layout::Hard => [0.0, 0.8, 0.0, 0.0, 0.2],
        Layout::Extreme => [0.0, 0.7, 0.0, 0.0, 0.3],
        Layout::Maze if x == 1 => [0.0, 0.5, 0.0, 0.0, 0.5],
        Layout::Maze if y == 7 => [0.5, 0.0, 0.0, 0.0, 0.5],
        Layout::Maze => CALM,
        Layout::MazeComplex => {
            if (x == 1 && y <= 3) || [(2, 3), (3, 3)].contains(&(x, y)) {
                [0.0, 0.2, 0.0, 0.0, 0.8]
            } else if x == 1 {
                [0.0, 0.5, 0.0, 0.0, 0.5]
            } else if x == 7 || (x, y) == (4, 5) {
                [0.0, 0.0, 0.8, 0.0, 0.2]
            } else if (4..7).contains(&y) && x < 7 {
                [0.0, 0.8, 0.1, 0.0, 0.1]
            } else if y == 7 || [(2, 2), (3, 2)].contains(&(x, y)) {
                [0.8, 0.0, 0.0, 0.0, 0.2]
            } else {
                CALM
            }
        }
    }
}

fn round_all(values: &[f64], decimals: i32) -> Vec<f64> {
    values.iter().map(|value| round(*value, decimals)).collect()
}

/// numpy.round: scale, round half to even, unscale.
pub(crate) fn round(value: f64, decimals: i32) -> f64 {
    let scale = 10f64.powi(decimals);
    (value * scale).round_ties_even() / scale
}
