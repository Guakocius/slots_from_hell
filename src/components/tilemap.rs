//! A module for creating the world's environments and embedding its graphics inside a
//! Tilemap.

use crate::{GameState, InGame, generate_rooms};
use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use bevy_northstar::prelude::*;

/// A plugin which adds the scene's setup and the tilemap update to the `App's`
/// behavior.
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
        app.add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, pause.run_if(in_state(GameState::Playing)));
    }
}

const TILE_SIZE: f32 = 64.0;
const CHUNK_SIZE: UVec2 = UVec2::splat(16);
const TILE_DISPLAY_SIZE: UVec2 = UVec2::splat(64);
const MAP_WORLD_SIZE: f32 = 3072.0; // -1536.0..1536.0
const MAP_OFFSET: Vec2 = Vec2::splat(MAP_WORLD_SIZE / 2.0);

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

/// [`Component`] struct for the rooms' walls.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::tilemap::Wall;
///
/// let wall = Wall::new(100.0, 64.0, Vec3::new(0.0, 0.0, 0.0));
///
/// assert_eq!(wall.height, 100.0);
/// assert_eq!(wall.width, 64.0);
/// assert_eq!(wall.pos.x, 0.0);
/// assert_eq!(wall.pos.y, 0.0);
/// ```
#[derive(Component, Debug)]
pub struct Wall {
    /// The wall's height.
    pub height: f32,
    /// The wall's width.
    pub width: f32,
    /// The wall's position on the map represented as a three-dimensional Vector.
    pub pos: Vec3,
    texture: String,
}

#[derive(Component, Debug)]
pub struct Room {
    pub name: String,
    pub pos: Vec3,
    texture: String,
}

#[derive(Resource, Debug)]
pub struct RoomDimensions(pub Vec<Vec2>);

impl Room {
    fn new(name: String, pos: Vec3, texture: String) -> Self {
        Self { name, pos, texture }
    }
}

/// A [`Component`] struct for the Door elements.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use slots_from_hell::components::tilemap::Door;
///
/// let door = Door::new(Vec3::new(0.0, 0.0, 0.0));
///
/// assert_eq!(door.height, 64.0);
/// assert_eq!(door.width, 64.0);
/// assert_eq!(door.pos.x, 0.0);
/// assert_eq!(door.pos.y, 0.0);
/// ```
#[derive(Component, Debug)]
pub struct Door {
    /// The door's height.
    pub height: f32,
    /// The door's width.
    pub width: f32,
    /// The door's position on the map represented as a three-dimensional Vector.
    pub pos: Vec3,
}

impl Door {
    /// Generates a new door with hard-coded `height` and `width` values and a custom position.
    ///
    /// # Examples
    ///
    /// ```
    /// use slots_from_hell::components::tilemap::Door;
    ///
    /// let door = Door::new(Vec3::new(0.0, 0.0, 0.0));
    ///
    /// assert_eq!(door.height, 64.0);
    /// assert_eq!(door.width, 64.0);
    /// assert_eq!(door.pos.x, 0.0);
    /// assert_eq!(door.pos.y, 0.0);
    /// ```
    pub fn new(pos: Vec3) -> Self {
        Self {
            height: 64.0,
            width: 64.0,
            pos,
        }
    }
}

impl Wall {
    /// Generates a new wall with a custom height, width, position and a hard-coded texture file
    /// path.
    ///
    /// # Examples
    ///
    /// ```
    /// use slots_from_hell::components::tilemap::Wall;
    ///
    /// let wall = Wall::new(100.0, 64.0, Vec3::new(0.0, 0.0, 0.0));
    ///
    /// assert_eq!(wall.height, 100.0);
    /// assert_eq!(wall.width, 64.0);
    /// assert_eq!(wall.pos.x, 0.0);
    /// assert_eq!(wall.pos.y, 0.0);
    /// ```
    pub fn new(height: f32, width: f32, pos: Vec3) -> Self {
        Self {
            height,
            width,
            pos,
            texture: String::from("textures/map_texture_wall_concrete.png"),
        }
    }
}

/// Struct to check which walls should have doors.
///
/// # Examples
///
/// ```
/// use slots_from_hell::components::tilemap::DoorSides;
///
/// let door_sides = DoorSides {
///     top: true,
///     bottom: false,
///     left: false,
///     right: false,
///     skip_left: false,
///     skip_right: false,
///     skip_top: false,
///     skip_bottom: false,
/// };
///
/// if door_sides.top {
///     println!("Leaving a gap on the top wall.");
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct DoorSides {
    /// The top wall should have a door.
    pub top: bool,
    /// The bottom wall should have a door.
    pub bottom: bool,
    /// The left hand side wall should have a door.
    pub left: bool,
    /// The right hand side wall should have a door.
    pub right: bool,

    /// Skip the left hand side wall.
    pub skip_left: bool,
    /// Skip the right hand side wall.
    pub skip_right: bool,
    /// Skip the top wall.
    pub skip_top: bool,
    /// Skip the bottom wall.
    pub skip_bottom: bool,
}

fn world_to_grid(pos: Vec2) -> UVec3 {
    let shifted = pos + MAP_OFFSET;
    let x = (shifted.x / TILE_SIZE).floor() as u32;
    let y = (shifted.y / TILE_SIZE).floor() as u32;
    UVec3::new(x, y, 0)
}

fn generate_walls(
    center: Vec2,
    size: Vec2,
    tile_size: f32,
    doors: DoorSides,
    grid: &mut CardinalGrid,
) -> Vec<Wall> {
    let half = size / 2.0;
    let inset = half - Vec2::splat(tile_size / 2.0);
    let mut walls = Vec::new();

    for (skip, has_door, y) in [
        (doors.skip_bottom, doors.bottom, -inset.y),
        (doors.skip_top, doors.top, inset.y),
    ] {
        if skip {
            continue;
        }
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
    walls.iter().for_each(|w| {
        let grid_pos = world_to_grid(w.pos.truncate());
        if grid_pos.x < 48 && grid_pos.y < 48 {
            grid.set_nav(grid_pos, Nav::Impassable);
        }
    });
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

    let grid_settings = GridSettingsBuilder::new_2d(48, 48) // colums,rows = 3072/64px = 48 Tiles
        .chunk_size(16)
        .build();

    let mut grid = CardinalGrid::new(&grid_settings);

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
        &mut grid,
    ));

    // Right room walls
    walls.extend(generate_walls(
        Vec2::new(1024.0, 0.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            top: true,
            bottom: true,
            skip_left: true,
            ..default()
        },
        &mut grid,
    ));

    // Left room walls
    walls.extend(generate_walls(
        Vec2::new(-1024.0, 0.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            bottom: true,
            skip_right: true,
            ..default()
        },
        &mut grid,
    ));

    // Top room walls
    walls.extend(generate_walls(
        Vec2::new(0.0, 1024.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            right: true,
            skip_bottom: true,
            ..default()
        },
        &mut grid,
    ));

    // Bottom room walls
    walls.extend(generate_walls(
        Vec2::new(0.0, -1024.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            left: true,
            right: true,
            skip_top: true,
            ..default()
        },
        &mut grid,
    ));

    // Top right room walls
    walls.extend(generate_walls(
        Vec2::new(1024.0, 1024.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            skip_left: true,
            skip_bottom: true,
            ..default()
        },
        &mut grid,
    ));

    // Bottom left room walls
    walls.extend(generate_walls(
        Vec2::new(-1024.0, -1024.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            skip_right: true,
            skip_top: true,
            ..default()
        },
        &mut grid,
    ));

    // Bottom right room wall
    walls.extend(generate_walls(
        Vec2::new(1024.0, -1024.0),
        Vec2::splat(1024.0),
        TILE_SIZE,
        DoorSides {
            skip_top: true,
            skip_left: true,
            ..default()
        },
        &mut grid,
    ));
    grid.build();

    cmds.spawn(grid);

    let tile_data: Vec<Option<TileData>> = (0..CHUNK_SIZE.element_product())
        .map(|i| Some(TileData::from_tileset_index(i as u16)))
        .collect();

    generate_rooms!(
        (
            "Surveillance Room",
            Vec3::new(0.0, 0.0, -200.0),
            "textures/map_texture_floor.png"
        ),
        (
            "Kitchen",
            Vec3::new(1024.0, 0.0, -200.0),
            "textures/map_texture_kitchen.png"
        ),
        (
            "Pianists Room",
            Vec3::new(-1024.0, 0.0, -200.0),
            "textures/map_texture_pianists_room.png"
        ),
        (
            "Bedroom",
            Vec3::new(0.0, 1024.0, -200.0),
            "textures/map_texture_wooden1.png"
        ),
        (
            "Living Room",
            Vec3::new(0.0, -1024.0, -200.0),
            "textures/map_texture_wooden2.png"
        ),
        (
            "Home Office",
            Vec3::new(1024.0, 1024.0, -200.0),
            "textures/map_texture_wooden3.png"
        ),
        (
            "Dining Room",
            Vec3::new(1024.0, -1024.0, -200.0),
            "textures/map_texture_bricks.png"
        ),
        (
            "Bathroom",
            Vec3::new(-1024.0, -1024.0, -200.0),
            "textures/map_texture_oldbricks.png"
        )
    )
    .iter()
    .for_each(|r| {
        cmds.spawn((
            WorldMap,
            TilemapChunk {
                chunk_size: CHUNK_SIZE,
                tile_display_size: TILE_DISPLAY_SIZE,
                tileset: assets
                    .load_builder()
                    .with_settings(|settings: &mut ImageLoaderSettings| {
                        settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 })
                    })
                    .load(r.texture.clone()),

                ..default()
            },
            TilemapChunkTileData(tile_data.clone()),
            Transform::from_translation(r.pos),
            Room::new(r.name.clone(), r.pos, r.texture.clone()),
        ));
    });

    // Walls
    walls.iter().for_each(|w| {
        cmds.spawn((
            Wall::new(w.height, w.width, w.pos),
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
        // Main room doors
        Door::new(Vec3::new(480.0, 0.0, 0.0)),
        Door::new(Vec3::new(0.0, -480.0, 0.0)),
        Door::new(Vec3::new(-480.0, 0.0, 0.0)),
        Door::new(Vec3::new(0.0, 480.0, 0.0)),
        // Bottom doors
        Door::new(Vec3::new(-1024.0, -480.0, 0.0)),
        Door::new(Vec3::new(-480.0, -1024.0, 0.0)),
        Door::new(Vec3::new(480.0, -1024.0, 0.0)),
        Door::new(Vec3::new(1024.0, -480.0, 0.0)),
        // Top doors
        Door::new(Vec3::new(480.0, 1024.0, 0.0)),
        Door::new(Vec3::new(1024.0, 480.0, 0.0)),
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
