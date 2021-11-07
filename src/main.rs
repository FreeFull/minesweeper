use vizia::*;

use crate::state::BoardState;

mod board;
mod scorebar;
mod state;

fn main() {
    let description = WindowDescription::new().with_title("Minesweeper");
    let app = Application::new(|cx| {
        let _ = cx.add_theme(include_str!("../style.css"));
        state::BoardState::new(11, 11, 10).build(cx);

        let bar = HStack::new(cx, |cx| {
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
            )
            .class("new_game");
        })
        .class("top_bar")
        .width(Percentage(100.0))
        .height(Pixels(30.0));
        Binding::new(cx, state::BoardState::root, |cx, data| {
            let &BoardState {
                width,
                height,
                new_game,
                ..
            } = data.get(cx);
            dbg!(width, height, new_game);
            board::Board::new(cx, width, height, new_game);
        });
    });

    app.run();
}

#[derive(Clone, Copy, Debug)]
struct NewGame {
    width: usize,
    height: usize,
    mines: usize,
}
