use arrayvec::ArrayVec;
use rand::prelude::*;
use tuix::*;

use crate::NewGame;

#[derive(Lens)]
pub struct BoardState {
    pub cells: Vec<CellState>,
    pub width: usize,
    pub height: usize,
    pub is_new_game: bool,
    pub total_mines: usize,
}

impl BoardState {
    pub fn new(width: usize, height: usize, mut mines: usize) -> BoardState {
        mines = std::cmp::min(mines, width * height - 1);
        BoardState {
            cells: vec![CellState::default(); width * height],
            width,
            height,
            is_new_game: true,
            total_mines: mines,
        }
    }

    fn generate(&mut self, clicked: usize) {
        for i in 1..std::cmp::min(self.total_mines + 1, self.cells.len()) {
            self.cells[i].mine = true;
        }
        self.cells[1..].shuffle(&mut thread_rng());
        // Make sure the clicked cell isn't a mine.
        self.cells.swap(0, clicked);
        for i in 0..self.cells.len() {
            self.cells[i].neighbours = self
                .neighbours(i)
                .into_iter()
                .map(|index| self.cells[index].mine as u8)
                .sum();
        }
    }

    fn neighbours(&self, index: usize) -> ArrayVec<usize, 8> {
        fn try_index(state: &BoardState, x: isize, y: isize) -> Option<usize> {
            if x < 0 || x >= state.width as isize || y < 0 || y >= state.height as isize {
                None
            } else {
                Some(x as usize + y as usize * state.width)
            }
        }

        let x = index % self.width;
        let y = index / self.width;
        let mut vec = ArrayVec::new();
        for xoff in [-1, 0, 1] {
            for yoff in [-1, 0, 1] {
                if (xoff == 0) && (yoff == 0) {
                    continue;
                }
                try_index(self, x as isize + xoff, y as isize + yoff).map(|index| vec.push(index));
            }
        }
        vec
    }

    fn reveal(&mut self, index: usize) {
        fn make_visible(state: &mut BoardState, index: usize) {
            let mut to_reveal = vec![index];
            while let Some(current) = to_reveal.pop() {
                if !state.cells[current].flagged {
                    state.cells[current].visible = true;
                    if state.cells[current].neighbours == 0 {
                        for new_index in state.neighbours(current) {
                            if state.cells[new_index].visible == false {
                                to_reveal.push(new_index);
                            }
                        }
                    }
                }
            }
        }
        if self.is_new_game {
            self.generate(index);
            self.is_new_game = false;
        }
        if !self.cells[index].visible {
            make_visible(self, index);
        } else {
            let neighbours = self.neighbours(index);
            if self.cells[index].neighbours
                == neighbours
                    .iter()
                    .map(|&i| self.cells[i].flagged as u8)
                    .sum()
            {
                for &i in neighbours.iter() {
                    make_visible(self, i);
                }
            }
        }
    }

    fn flag(&mut self, index: usize) {
        let cell = &mut self.cells[index];
        if !cell.visible && !self.is_new_game {
            cell.flagged = !cell.flagged;
        }
    }
}

impl Model for BoardState {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(&mut NewGame {
            width,
            height,
            mines,
        }) = event.message.downcast()
        {
            *self = BoardState::new(width, height, mines);
            entity.update(state);
        }

        if let Some(board_event) = event.message.downcast() {
            match *board_event {
                BoardEvent::Reveal(index) => {
                    self.reveal(index);
                    entity.update(state);
                }
                BoardEvent::Flag(index) => {
                    self.flag(index);
                    entity.update(state);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct CellState {
    pub visible: bool,
    pub flagged: bool,
    pub mine: bool,
    pub neighbours: u8,
}

pub enum BoardEvent {
    Reveal(usize),
    Flag(usize),
}
