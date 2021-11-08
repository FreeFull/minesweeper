use vizia::*;

use crate::state::BoardState;

mod board;
mod scorebar;
mod state;

fn main() {
    let description = WindowDescription::new().with_title("Minesweeper");
    let app = Application::new(|cx| {
        let _ = cx.add_stylesheet("style.css");
        state::BoardState::new(11, 11, 10).build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |cx| {
                        cx.emit(NewGame {
                            width: 9,
                            height: 9,
                            mines: 10,
                        });
                    },
                    |cx| {
                        Label::new(cx, "New Game");
                    },
                );
            })
            .width(Percentage(100.0))
            .class("top-bar");

            Binding::new(cx, state::BoardState::root, |cx, data| {
                println!("outer binding");
                let &BoardState {
                    width,
                    height,
                    new_game,
                    ..
                } = data.get(cx);
                board::Board::new(cx, width, height, new_game);
            });
        })
        .class("body");
    });

    app.run();
}

#[derive(Clone, Copy, Debug)]
struct NewGame {
    width: usize,
    height: usize,
    mines: usize,
}
