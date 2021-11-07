use vizia::*;

use crate::state::*;

pub struct Board {
    width: usize,
    height: usize,
    new_game: bool,
}

impl Board {
    pub fn new(cx: &mut Context, width: usize, height: usize, new_game: bool) -> Handle<Board> {
        Board {
            width,
            height,
            new_game,
        }
        .build(cx)
        .layout_type(LayoutType::Grid)
        .row_between(Pixels(1.0))
        .col_between(Pixels(1.0))
        .background_color(Color::rgb(50, 50, 50))
        .child_space(Pixels(1.0))
        .grid_cols(vec![Pixels(10.0); width])
        .grid_rows(vec![Pixels(10.0); height])
        .width(Pixels(200.0))
        .height(Pixels(200.0))
        .space(Stretch(1.0))
        .class("board")
    }
}

impl View for Board {
    fn body(&mut self, cx: &mut Context) {
        if self.new_game {
            let children: Vec<_> = cx.current.child_iter(&cx.tree).collect();
            for child in children {
                cx.remove(child);
            }
            let width = self.width;
            let height = self.height;
            Binding::new(cx, BoardState::cells, move |cx, field| {
                for x in 0..width {
                    for y in 0..height {
                        if let Some(&state) = field.get(cx).get(x + y * width) {
                            Cell::new(cx, x + y * width, state)
                                .col_index(x)
                                .row_index(y);
                        }
                    }
                }
            });
        }
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        /*
        if let Some(&mut new_game) = event.message.downcast::<NewGame>() {
            for child in self.cells.drain(..) {
                cx.remove(child);
            }
            cx.grid_rows(vec![Pixels(30.0); new_game.height])
                .set_grid_cols(vec![Pixels(30.0); new_game.width])
                .set_width(Pixels(31.0 * new_game.width as f32 + 1.0))
                .set_height(Pixels(31.0 * new_game.width as f32 + 1.0));
            for x in 0..new_game.width {
                for y in 0..new_game.height {
                    let index = x + y * new_game.width;
                    self.cells.push(Binding::new(cx, BoardState::cells, |cx| {
                        Cell { index }.build(cx).set_col_index(x).set_row_index(y)
                    }));
                }
            }
        }
        */
    }
}

struct Cell {
    index: usize,
}

impl Cell {
    fn new(cx: &mut Context, index: usize, state: CellState) -> Handle<Cell> {
        Cell { index }
            .build(cx)
            .class("cell")
            .width(Pixels(30.0))
            .height(Pixels(30.0))
    }
}

impl View for Cell {
    fn body(&mut self, cx: &mut Context) {}

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            return match *window_event {
                WindowEvent::MouseDown(MouseButton::Left) => {
                    cx.emit(BoardEvent::Reveal(self.index));
                }
                WindowEvent::MouseDown(MouseButton::Right) => {
                    cx.emit(BoardEvent::Flag(self.index));
                }
                _ => {}
            };
        }
    }

    /*
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
    */
}
