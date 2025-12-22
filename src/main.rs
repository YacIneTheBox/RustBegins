use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("John Johnson".to_string())));
    commands.spawn((Person, Name("Robert Robertson".to_string())));
    commands.spawn((Person, Name("Jane Johnson".to_string())));
}

fn print_names(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Person: {}", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people) // une seule fois au démarrage
        .add_systems(Update, print_names) // à chaque frame, on affiche
        .run();
}
