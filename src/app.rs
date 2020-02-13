
use {
    crate::{
        game_runner::GameRunner,
        message_runner,
        io::W,
        task_sync::*,
    },
};

pub enum AppState {
    Level,  // there might be a level id or something later
    Message(String, bool),
    Quit,
}

pub fn run(
    w: &mut W,
    dam: &mut Dam,
) {
    let mut state = Ok(AppState::Level);
    loop {
        state = match state {
            Ok(AppState::Level) => {
                let mut game_runner = GameRunner::new();
                game_runner.run(w, dam)
            }
            Ok(AppState::Message(s, good)) => {
                message_runner::run(w, s, good, dam)
            }
            Ok(AppState::Quit) => { return; }
            Err(e) => {
                println!("damn: {:?}", e);
                return; // we just quit
            }
        }
    }
}

