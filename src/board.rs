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
        .height(Stretch(1.0))
        .width(Stretch(1.0))
    }
}

impl View for Board {
    fn body(&mut self, cx: &mut Context) {
        if self.new_game {
            for child in cx.tree.child_iter(cx.current).collect::<Vec<_>>() {
                cx.remove(child)
            }
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
        })
        .layout_type(LayoutType::Grid)
        .row_between(Pixels(1.0))
        .col_between(Pixels(1.0))
        .background_color(Color::rgb(50, 50, 50))
        .grid_cols(vec![Pixels(20.0); width])
        .grid_rows(vec![Pixels(20.0); height])
        .width(Pixels(21.0 * width as f32 + 1.0))
        .height(Pixels(21.0 * height as f32 + 1.0))
        .child_space(Pixels(1.0))
        .class("board");
    }
}

struct Cell {
    index: usize,
}

impl Cell {
    fn new(cx: &mut Context, index: usize, state: CellState) -> Handle<Cell> {
        let mut cell = Cell { index }.build(cx).class("cell").text("");
        if state.flagged {
            cell = cell.text("🚩");
        } else if state.visible {
            cell = cell.class("visible");
            if state.mine {
                cell = cell.class("mine").text("💣");
            } else if state.neighbours != 0 {
                cell = cell.text(&state.neighbours.to_string());
            }
        }
        cell
    }
}

impl View for Cell {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match *window_event {
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
}
