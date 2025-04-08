use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name {
    name: String,
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name{name: "Thais Otsuka".to_string()}));
    commands.spawn((Person, Name{name: "Juan Canales".to_string()}));
}

fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.name);
    }
}

fn mutate_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.name == "Juan Canales" {
            name.name = "Juan Canales Otsuka".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (mutate_people, greet_people).chain());
    }
}