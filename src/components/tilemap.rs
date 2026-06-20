use bevy::{
    color::palettes::tailwind::AMBER_400,
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData, TilemapChunk, TilemapChunkTileData},
};
use chacha20::ChaCha8Rng;
use rand::{RngExt, SeedableRng};

use super::player::Player;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_systems(Startup, (setup, setup_scene).chain())
            .add_systems(Update, (update_tilemap, log_tile));
    }
}

#[derive(Component, Deref, DerefMut)]
struct UpdateTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct SeededRng(ChaCha8Rng);

fn setup(mut cmds: Commands, assets: Res<AssetServer>) {
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(8);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|_| rng.random_range(0..5))
        .map(|i| {
            if i == 0 {
                None
            } else {
                Some(TileData::from_tileset_index(i - 1))
            }
        })
        .collect();

    cmds.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load_with_settings(
                "textures/map_texture.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
                },
            ),
            alpha_mode: AlphaMode2d::Opaque,
        },
        TilemapChunkTileData(tile_data),
        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    cmds.insert_resource(SeededRng(rng));
}

#[derive(Component)]
struct MovePlayer;

fn setup_scene(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    chunk: Single<&TilemapChunk>,
) {
    let mut transform = chunk.calculate_tile_transform(UVec2::new(5, 6));
    transform.translation.z = 1.0;

    cmds.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000.0, 700.0))),
        MeshMaterial2d(materials.add(Color::from(AMBER_400))),
        transform,
    ));

    cmds.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));
}

fn update_tilemap(
    time: Res<Time>,
    mut query: Query<(&mut TilemapChunkTileData, &mut UpdateTimer)>,
    mut rng: ResMut<SeededRng>,
) {
    for (mut tile_data, mut timer) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            [0..50].iter().for_each(|_| {
                let index = rng.random_range(0..tile_data.len());
                tile_data[index] = Some(TileData::from_tileset_index(rng.random_range(0..5)));
            })
        }
    }
}

fn log_tile(tilemap: Single<(&TilemapChunk, &TilemapChunkTileData)>, mut local: Local<u16>) {
    let (chunk, data) = tilemap.into_inner();
    let Some(tile_data) = data.tile_data_from_tile_pos(chunk.chunk_size, UVec2::new(3, 4)) else {
        return;
    };
    if tile_data.tileset_index != *local {
        info!(?tile_data, "tile_data changed");
        *local = tile_data.tileset_index;
    }
}
