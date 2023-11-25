use bevy::{core::FixedTimestep, prelude::*};
use device_query::{DeviceQuery, DeviceState, MouseState};

#[derive(Component)]
struct Square {
    point: Vec2,
    origin: Vec2,
    range: f32,
}

const CELL_SIZE: f32 = 10.;
const SQUARE_COLOR: Color = Color::BLUE;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_square(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE * 6., CELL_SIZE * 6., CELL_SIZE),
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Square {
            point: Vec2::new(0., 0.),
            origin: Vec2::new(0., 0.),
            range: 1.,
        });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE * 2., CELL_SIZE * 2., CELL_SIZE),
                translation: Vec3::new(-15., -0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Square {
            point: Vec2::new(0., 0.),
            origin: Vec2::new(-15., -0.),
            range: 5.,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE * 2., CELL_SIZE * 2., CELL_SIZE),
                translation: Vec3::new(15., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Square {
            point: Vec2::new(0., 0.),
            origin: Vec2::new(15., -0.),
            range: 5.,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SQUARE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Square {
            point: Vec2::new(0., 0.),
            origin: Vec2::new(-15., -0.),
            range: 10.,
        });
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SQUARE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Square {
            point: Vec2::new(0., 0.),
            origin: Vec2::new(15., 0.),
            range: 10.,
        });
}

fn move_square(mut sprite_positions: Query<(&mut Square, &mut Transform)>) {
    for (_head, mut transform) in sprite_positions.iter_mut() {
        transform.translation.x = _head.point.x * _head.range + _head.origin.x;
        transform.translation.y = _head.point.y * _head.range + _head.origin.y;
    }
}

fn check_mouse(mut sprite: Query<&mut Square>, mut windows: ResMut<Windows>) {
    for mut head in sprite.iter_mut() {
        let device_state = DeviceState::new();
        let mouse: MouseState = device_state.get_mouse();

        let window1 = windows.primary_mut();
        let window_pos = window1.position().unwrap();

        let x: f32 = ((mouse.coords.0 - window_pos.x - 250) as f32 + head.origin.x) / 1920.;
        let y: f32 = -((mouse.coords.1 - window_pos.y - 250) as f32 + head.origin.y) / 1080.;

        head.point = Vec2::new(x, y);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            // <--
            title: "Tracking".to_string(), // <--
            width: 500.0,                  // <--
            height: 500.0,                 // <--
            ..default()                    // <--
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_square)
        .add_system(move_square)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01))
                .with_system(check_mouse),
        )
        .run();
}
