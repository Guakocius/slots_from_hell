//! # Slots from Hell
//!
//! A 2D horror game for the Juniper Game Jam using the Bevy engine.
//!
//! ## Examples
//!
//! ```
//! use bevy::prelude::*;
//! use slots_from_hell::components::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((
//!         DefaultPlugins,
//!         screens::game_menu::menu::menu_plugin,
//!         tilemap::TilemapPlugin,
//!         ));
//! }
//! ```

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
/// All game components packed into a public module.
pub mod components {
    pub mod entity;
    pub mod player;
    pub mod tilemap;

    /// Public module for all screens and menus.
    pub mod screens {
        pub mod fps_overlay;
        pub mod game_menu;
    }
}

pub use components::{
    entity::*,
    player::*,
    screens::{fps_overlay::*, game_menu::*},
    tilemap::*,
};
