use crate::Color;
use crate::Rectangle;

pub const ALIVE_COLOR: Color = Color::WHITE;
pub const DEAD_COLOR: Color = Color::BLACK;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    ALIVE(Color),
    DEAD(Color),
}

#[derive(Copy, Clone)]
pub struct Cell {
    rec: Rectangle,
    state: State,
}

impl Cell {
    pub fn new(x:f32, y:f32, length:f32) -> Cell {
        Self {
            rec: Rectangle {
                 x,
                 y,
                 width: length,
                 height: length,
            },
            state: State::DEAD(DEAD_COLOR),
        }
    }

    pub fn rectangle(&self) -> Rectangle {
        self.rec
    }

    pub fn color(&self) -> Color {
        match self.state {
            State::ALIVE(i) => i,
            State::DEAD(j) => j,
        }
    }

    pub fn set_alive(&mut self) {
        self.state = State::ALIVE(ALIVE_COLOR);
    }

    pub fn set_dead(&mut self) {
        self.state = State::DEAD(DEAD_COLOR);
    }

    pub fn is_alive(&self) -> bool {
        self.state == State::ALIVE(ALIVE_COLOR)
    }
}
