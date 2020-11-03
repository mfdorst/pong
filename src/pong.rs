use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Paddle>();
        let spritesheet_handle = load_spritesheet(world);
        initialize_camera(world);
        initialize_paddles(world, spritesheet_handle);
    }
}

#[derive(Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Self {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

/// Setup the camera so that it covers the whole arena with (0, 0) in the botom left
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    // Position camera in center of arena so that (0, 0) will be in the bottom left instead of the
    // middle of the screen
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_paddles(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    let paddle_y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, paddle_y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, paddle_y, 0.0);
    let sprite_render = SpriteRender::new(spritesheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();
    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &spritesheet_storage,
    )
}
