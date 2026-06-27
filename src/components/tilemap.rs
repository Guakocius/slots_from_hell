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

#[derive(Component, Debug)]
pub struct WorldMap;

#[derive(Component, Deref, DerefMut)]
struct UpdateTimer(Timer);

fn setup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    map_query: Query<&TilemapChunk, With<WorldMap>>,
) {
    if !map_query.is_empty() {
        return;
    }

    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(8);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|i| Some(TileData::from_tileset_index(i as u16)))
        .collect();

    cmds.spawn((
        WorldMap,
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load_with_settings(
                "textures/map_texture_floor.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
                },
            ),
            ..default()
        },
        TilemapChunkTileData(tile_data),
        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Transform::from_translation(Vec3::new(0.0, 0.0, -200.0)),
    ));
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
