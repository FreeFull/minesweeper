use tuix::*;

use crate::{state::*, NewGame};

pub struct Board {
    cells: Vec<Entity>,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: Vec::new() }
    }
}

impl Widget for Board {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut tuix::State, entity: tuix::Entity) -> Self::Ret {
        entity
            .set_element(state, "board")
            .set_layout_type(state, LayoutType::Grid)
            .set_row_between(state, Pixels(1.0))
            .set_col_between(state, Pixels(1.0))
            .set_background_color(state, Color::rgb(50, 50, 50))
            .set_child_space(state, Pixels(1.0))
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(&mut new_game) = event.message.downcast::<NewGame>() {
            for child in self.cells.drain(..) {
                state.remove(child);
            }
            entity
                .set_grid_rows(state, vec![Pixels(30.0); new_game.height])
                .set_grid_cols(state, vec![Pixels(30.0); new_game.width])
                .set_width(state, Pixels(31.0 * new_game.width as f32 + 1.0))
                .set_height(state, Pixels(31.0 * new_game.width as f32 + 1.0));
            for x in 0..new_game.width {
                for y in 0..new_game.height {
                    let index = x + y * new_game.width;
                    self.cells.push(
                        Cell { index }
                            .bind(BoardState::cells, move |cell| cell[index])
                            .build(state, entity, |builder| {
                                builder.set_col_index(x).set_row_index(y)
                            }),
                    );
                }
            }
        }
    }
}

struct Cell {
    index: usize,
}

impl Widget for Cell {
    type Ret = Entity;
    type Data = CellState;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "cell")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match *window_event {
                WindowEvent::MouseDown(MouseButton::Left) => {
                    entity.emit(state, BoardEvent::Reveal(self.index));
                }
                WindowEvent::MouseDown(MouseButton::Right) => {
                    entity.emit(state, BoardEvent::Flag(self.index));
                }
                _ => {}
            }
        }
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        // todo
        if data.visible {
            entity.class(state, "visible");
            if data.mine {
                entity.class(state, "mine");
                entity.set_text(state, "💣");
            } else {
                if data.neighbours != 0 {
                    entity.set_text(state, &data.neighbours.to_string());
                }
            }
        } else if data.flagged {
            entity.set_text(state, "🚩");
        } else {
            entity.set_text(state, "");
        }
    }
}
