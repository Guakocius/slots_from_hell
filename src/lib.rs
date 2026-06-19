pub mod components {
    pub mod entity;
    pub mod player;

    pub mod screens {
        pub mod game_menu;
    }
}

pub use components::{entity::*, player::*, screens::game_menu::*};
