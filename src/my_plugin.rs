use bevy::{math::vec3, prelude::*, window::exit_on_primary_closed};

const MOVE_SPEED: f32 = 200.0;

#[derive(Component)]
struct Character;

#[derive(Bundle)]
struct CharacterBundle {
    sprite: Sprite,
    character: Character,
}

fn init_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(CharacterBundle {
        sprite: Sprite::from_image(asset_server.load("mono.png")),
        character: Character,
    });

}

fn update_character(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Character>>) {
    let mut direction = vec3(0.0, 0.0, 0.0);
    if !keys.any_pressed(vec![KeyCode::ArrowUp, KeyCode::ArrowDown, KeyCode::ArrowLeft, KeyCode::ArrowRight]) {
        return;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        direction[1] = 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction[1] = -1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction[0] = -1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction[0] = 1.0;
    }
    let translate = direction.normalize() * MOVE_SPEED * time.delta_secs();
    for mut transform in &mut query {
        transform.translation += translate;
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(30.0));
        // add things to your app here
        app.add_systems(Startup, init_resources);
        app.add_systems(FixedUpdate, update_character);
        app.add_systems(Update, exit_on_primary_closed);
    }
}