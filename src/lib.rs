//! # Slots from Hell
//!
//! A 2D horror game for the Juniper Game Jam using the Bevy engine.

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod components {
    pub mod entity;
    pub mod player;
    pub mod tilemap;

    pub mod screens {
        pub mod game_menu;
    }
}

pub use components::{entity::*, player::*, screens::game_menu::*, tilemap::*};
