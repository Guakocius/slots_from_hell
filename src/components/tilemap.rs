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

#[derive(Clone, Debug, Default)]
pub struct DoorSides {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,

    pub skip_left: bool,
    pub skip_right: bool,
    pub skip_top: bool,
    pub skip_bottom: bool,
}

fn generate_walls(center: Vec2, size: Vec2, tile_size: f32, doors: DoorSides) -> Vec<Wall> {
    let half = size / 2.0;
    let inset = half - Vec2::splat(tile_size / 2.0);
    let mut walls = Vec::new();

    for (has_door, y) in [(doors.bottom, -inset.y), (doors.top, inset.y)] {
        if has_door {
            let seg = (size.x - tile_size) / 2.0;
            let off = (tile_size + seg) / 2.0;
            // Left segment
            walls.push(Wall::new(
                seg,
                tile_size,
                Vec3::new(center.x - off, center.y + y, 0.0),
            ));
            // Right segment
            walls.push(Wall::new(
                seg,
                tile_size,
                Vec3::new(center.x + off, center.y + y, 0.0),
            ));
        } else {
            // Full wall
            walls.push(Wall::new(
                size.x,
                tile_size,
                Vec3::new(center.x, center.y + y, 0.0),
            ));
        }
    }

    for (skip, has_door, x) in [
        (doors.skip_left, doors.left, -inset.x),
        (doors.skip_right, doors.right, inset.x),
    ] {
        if skip {
            continue;
        }
        if has_door {
            let seg = (size.y - tile_size) / 2.0;
            let off = (tile_size + seg) / 2.0;
            // Bottom segment
            walls.push(Wall::new(
                tile_size,
                seg,
                Vec3::new(center.x + x, center.y - off, 0.0),
            ));
            // Top segment
            walls.push(Wall::new(
                tile_size,
                seg,
                Vec3::new(center.x + x, center.y + off, 0.0),
            ));
        } else {
            // Full wall
            walls.push(Wall::new(
                tile_size,
                size.y,
                Vec3::new(center.x + x, center.y, 0.0),
            ));
        }
    }
    walls
}

fn setup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    map_query: Query<&TilemapChunk, With<WorldMap>>,
) {
    if !map_query.is_empty() {
        return;
    }

    const TILE_SIZE: f32 = 64.0;

    let mut walls = Vec::new();

    // Main room walls
    walls.extend(generate_walls(
        Vec2::ZERO,
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            top: true,
            bottom: true,
            left: true,
            right: true,
            ..default()
        },
    ));

    // Right room walls
    walls.extend(generate_walls(
        Vec2::new(1024.0, 0.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            skip_left: true,
            ..default()
        },
    ));

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
    walls.iter().for_each(|w| {
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
        Door::new(Vec3::new(480.0, 0.0, 0.0)),
        Door::new(Vec3::new(0.0, -480.0, 0.0)),
        Door::new(Vec3::new(-480.0, 0.0, 0.0)),
        Door::new(Vec3::new(0.0, 480.0, 0.0)),
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
