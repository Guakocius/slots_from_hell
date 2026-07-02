//! A module for creating the world's environments and embedding its graphics inside a
//! Tilemap

use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
    ui_render::NodeType::Rect,
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

#[derive(Component, Debug)]
pub struct Wall {
    height: f32,
    width: f32,
    pos: Vec3,
    texture: String,
}

#[derive(Component, Debug)]
pub struct Door {
    height: f32,
    width: f32,
    pos: Vec3,
}

impl Door {
    fn new(pos: Vec3) -> Self {
        Self {
            height: 64.0,
            width: 64.0,
            pos,
        }
    }
}

impl Wall {
    fn new(height: f32, width: f32, pos: Vec3) -> Self {
        Self {
            height,
            width,
            pos,
            texture: String::from("textures/map_texture_wall_concrete.png"),
        }
    }
}

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
            "textures/map_texture_pianists_room.png",
        ),
        (
            Vec3::new(0.0, 1024.0, -200.0),
            "textures/map_texture_wooden1.png",
        ),
        (
            Vec3::new(0.0, -1024.0, -200.0),
            "textures/map_texture_wooden2.png",
        ),
        (
            Vec3::new(1024.0, 1024.0, -200.0),
            "textures/map_texture_wooden3.png",
        ),
        (
            Vec3::new(1024.0, -1024.0, -200.0),
            "textures/map_texture_bricks.png",
        ),
        (
            Vec3::new(-1024.0, -1024.0, -200.0),
            "textures/map_texture_oldbricks.png",
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
    [
        // Main room
        Wall::new(64.0, 448.0, Vec3::new(512.0, -256.0, 0.0)),
        Wall::new(64.0, 448.0, Vec3::new(512.0, 256.0, 0.0)),
        Wall::new(448.0, 64.0, Vec3::new(256.0, -512.0, 0.0)),
        Wall::new(448.0, 64.0, Vec3::new(-256.0, -512.0, 0.0)),
        Wall::new(64.0, 448.0, Vec3::new(-512.0, -256.0, 0.0)),
        Wall::new(64.0, 448.0, Vec3::new(-512.0, 256.0, 0.0)),
        Wall::new(448.0, 64.0, Vec3::new(-256.0, 512.0, 0.0)),
        Wall::new(448.0, 64.0, Vec3::new(256.0, 512.0, 0.0)),
        //
        Wall::new(448.0, 64.0, Vec3::new(704.0, -512.0, 0.0)),
        Wall::new(448.0, 64.0, Vec3::new(704.0, 512.0, 0.0)),
    ]
    .iter()
    .for_each(|w| {
        cmds.spawn((
            Sprite {
                image: assets.load(&w.texture),
                custom_size: Some(Vec2::new(w.height, w.width)),
                ..default()
            },
            Transform::from_translation(w.pos),
        ));
    });

    // Doors
    [
        Door::new(Vec3::new(512.0, 0.0, 0.0)),
        Door::new(Vec3::new(0.0, -512.0, 0.0)),
        Door::new(Vec3::new(-512.0, 0.0, 0.0)),
        Door::new(Vec3::new(0.0, 512.0, 0.0)),
    ]
    .iter()
    .for_each(|d| {
        cmds.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(d.width, d.height)),
                ..default()
            },
            Transform::from_translation(d.pos),
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
