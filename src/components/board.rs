use crate::components::cell::Cell;
use crate::core_logic::{Cell as CellType, Game};
use leptos::*;

#[component]
pub fn Board(
    #[prop(into)] game_state: Signal<Game>,
    #[prop(into)] on_click: Callback<usize>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 gap-4 p-4 bg-gray-900 rounded-xl shadow-2xl">
            {move || {
                game_state.get().board.iter().enumerate().map(|(i, &cell)| {
                    view! {
                        <Cell
                            cell=cell
                            index=i
                            on_click=on_click
                        />
                    }
                }).collect_view()
            }}
        </div>
    }
}
