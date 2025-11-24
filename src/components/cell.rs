use crate::core_logic::{Cell as CellType, Player};
use leptos::*;

#[component]
pub fn Cell(
    cell: CellType,
    index: usize,
    #[prop(into)] on_click: Callback<usize>,
) -> impl IntoView {
    let cell_content = match cell {
        CellType::Empty => "",
        CellType::Occupied(Player::X) => "X",
        CellType::Occupied(Player::O) => "O",
    };

    let cell_style = match cell {
        CellType::Empty => "cursor-pointer hover:bg-gray-700",
        CellType::Occupied(Player::X) => "text-blue-400 cursor-default",
        CellType::Occupied(Player::O) => "text-red-400 cursor-default",
    };

    view! {
        <div
            class=format!(
                "w-24 h-24 flex items-center justify-center text-4xl font-bold bg-gray-800 border-2 border-gray-600 rounded-lg transition-colors duration-200 {}",
                cell_style
            )
            on:click=move |_| on_click.call(index)
        >
            {cell_content}
        </div>
    }
}
