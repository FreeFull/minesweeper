use tuix::widgets::*;
use tuix::*;

mod board;
mod scorebar;
mod state;

fn main() {
    let description = WindowDescription::new().with_title("Minesweeper");
    let app = Application::new(description, |state, window| {
        let _ = state.add_stylesheet("style.css");
        let model = state::BoardState::new(11, 11, 10)
            .build(state, window)
            .set_layout_type(state, LayoutType::Column)
            .set_element(state, "body");
        let bar = Row::new().build(state, model, |builder| {
            builder.class("top_bar").set_height(Pixels(30.0))
        });
        Button::with_label("New Game")
            .on_press(move |_, state, entity| {
                NewGame {
                    width: 9,
                    height: 9,
                    mines: 10,
                }
                .emit(state);
            })
            .build(state, bar, |builder| builder.class("new_game"));
        board::Board::new().build(state, model, |builder| builder);
        NewGame {
            width: 11,
            height: 11,
            mines: 10,
        }
        .emit(state);
    });

    app.run();
}

#[derive(Clone, Copy)]
struct NewGame {
    width: usize,
    height: usize,
    mines: usize,
}

impl NewGame {
    fn emit(self, state: &mut State) {
        state.insert_event(Event::new(self).propagate(Propagation::All));
    }
}
