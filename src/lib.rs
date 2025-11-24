pub mod components;
pub mod core_logic;

use components::board::Board;
use core_logic::{Game, Player};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (game, set_game) = create_signal(Game::new());

    let status = create_memo(move |_| {
        let g = game.get();
        if let Some(winner) = g.winner {
            match winner {
                Player::X => "Winner: X".to_string(),
                Player::O => "Winner: O".to_string(),
            }
        } else if g.is_draw() {
            "Draw!".to_string()
        } else {
            match g.current_turn {
                Player::X => "Current Turn: X".to_string(),
                Player::O => "Current Turn: O".to_string(),
            }
        }
    });

    let on_cell_click = move |index: usize| {
        set_game.update(|g| {
            let _ = g.play_turn(index);
        });
    };

    let reset_game = move |_| {
        set_game.set(Game::new());
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-950 text-white">
            <h1 class="text-5xl font-extrabold mb-8 text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-600">
                "Tic-Tac-Toe"
            </h1>

            <div class="mb-6 text-2xl font-semibold tracking-wide">
                {status}
            </div>

            <Board
                game_state=game
                on_click=on_cell_click
            />

            <button
                class="mt-8 px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white font-bold rounded-full shadow-lg transform transition hover:scale-105 focus:outline-none focus:ring-2 focus:ring-blue-400"
                on:click=reset_game
            >
                "Restart Game"
            </button>
        </div>
    }
}
