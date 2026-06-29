//! A module for creating the world's environments and embedding its graphics inside a
//! Tilemap

use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use super::screens::game_menu::{GameState, InGame};

/// A plugin which adds the scene's setup and the tilemap update to the `App's`
/// behavior
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::tilemap::TilemapPlugin;
///
/// App::new().add_plugins(TilemapPlugin);
/// ```
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, pause.run_if(in_state(GameState::Playing)));
    }
}

/// A module representing an already existing World map.
///
/// # Examples
///
/// ```
/// use bevy::{prelude::*, sprite_render::TilemapChunk};
/// use slots_from_hell::components::{screens::game_menu::GameState, tilemap::WorldMap};
///
/// fn world_exists(map_query: Query<&TilemapChunk, With<WorldMap>>) {
///     if map_query.is_empty() {
///         println!("World map is empty.");
///     } else {
///         println!("World map is not empty.");
///     }
/// }
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_systems(OnEnter(GameState::Playing), world_exists);
/// ```
#[derive(Component, Debug)]
pub struct WorldMap;

fn setup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    map_query: Query<&TilemapChunk, With<WorldMap>>,
) {
    if !map_query.is_empty() {
        return;
    }

    let chunk_size = UVec2::splat(16);
    let tile_display_size = UVec2::splat(64);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|i| Some(TileData::from_tileset_index(i as u16)))
        .collect();

    // Rooms
    [
        (
            Vec3::new(0.0, 0.0, -200.0),
            "textures/map_texture_floor.png",
        ),
        (
            Vec3::new(1024.0, 0.0, -200.0),
            "textures/map_texture_kitchen.png",
        ),
        (
            Vec3::new(-1024.0, 0.0, -200.0),
            "textures/map_texture_kitchen.png",
        ),
        (
            Vec3::new(0.0, 1024.0, -200.0),
            "textures/map_texture_kitchen.png",
        ),
        (
            Vec3::new(0.0, -1024.0, -200.0),
            "textures/map_texture_kitchen.png",
        ),
        (
            Vec3::new(1024.0, 1024.0, -200.0),
            "textures/map_texture_kitchen.png",
        ),
        (
            Vec3::new(1024.0, -1024.0, -200.0),
            "textures/map_texture_kitchen.png",
        ),
    ]
    .iter()
    .for_each(|(v, p)| {
        cmds.spawn((
            WorldMap,
            TilemapChunk {
                chunk_size,
                tile_display_size,
                tileset: assets
                    .load_builder()
                    .with_settings(|settings: &mut ImageLoaderSettings| {
                        settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 })
                    })
                    .load(*p),

                ..default()
            },
            TilemapChunkTileData(tile_data.clone()),
            Transform::from_translation(*v),
        ));
    });

    // Walls
    [(
        Vec3::new(384.0, 0.0, -200.0),
        "textures/map_texture_floor.png",
    )]
    .iter()
    .for_each(|(v, p)| {
        cmds.spawn((
            WorldMap,
            TilemapChunk {
                chunk_size,
                tile_display_size,
                tileset: assets
                    .load_builder()
                    .with_settings(|settings: &mut ImageLoaderSettings| {
                        settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 })
                    })
                    .load(*p),

                ..default()
            },
            TilemapChunkTileData(tile_data.clone()),
            Transform::from_translation(*v),
        ));
    })
}

fn pause(
    mut game_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
    is_in_game: Res<InGame>,
) {
    if input.pressed(KeyCode::Escape) && is_in_game.0 {
        game_state.set(GameState::Pause);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
