use bevy::prelude::*;

fn spawn_ball(mut commands: Commands) {
    println!("Spawning ball...");
    commands.spawn_empty();
}

fn spawn_camera(mut commands: Commands) {
    commands
      .spawn((Camera2d,Transform::from_xyz(0., 10., 0.)));
}
  
fn main() {
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_ball, spawn_camera))
    .run();
}