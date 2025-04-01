use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the ball (Circle)
    commands.spawn(ShapeBundle {
        shape: Shape::Circle { radius: 25.0 }.into(),
        fill: Fill::color(Color::RED), // Fill with red color
        transform: Transform::from_xyz(0.0, 200.0, 0.0), // Position above floor
        ..Default::default()
    });

    // Spawn the floor
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(Vec2::new(500.0, 20.0)), // Floor width and height
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -150.0, 0.0), // Position at bottom
        ..Default::default()
    });
}