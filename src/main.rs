use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(detect_wall_collisions)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Immovable;

#[derive(Component)]
struct Velocity(Vec3);

fn setup(mut commands: Commands, mut _meshes: ResMut<Assets<Mesh>>, mut _materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    })
    .insert(Velocity(Vec3::ZERO))
    .insert(Player);

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::rgb(0.0, 0.25, 0.0),
            custom_size: Some(Vec2::new(50.0, 500.0)),
            ..default()
        },
        ..default()
    })
    .insert(Immovable);

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-375.0, 0.0, 10.0)),
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        ..default()
    });
}

fn player_movement(mut query: Query<(&mut Velocity, &mut Transform), With<Player>>, input: Res<Input<KeyCode>>, time: Res<Time>) {
    let (mut velocity, mut transform) = query.single_mut();

    if input.pressed(KeyCode::W) {velocity.0.y += 1.0}
    if input.pressed(KeyCode::S) {velocity.0.y -= 1.0}
    if input.pressed(KeyCode::A) {velocity.0.x -= 1.0}
    if input.pressed(KeyCode::D) {velocity.0.x += 1.0}

    velocity.0 *= 0.8;
    transform.translation += velocity.0 * time.delta_seconds() * 100.0;

    // stopping the velocity values from getting too small
    if velocity.0.x.abs() < 0.1 {velocity.0.x = 0.0}
    if velocity.0.y.abs() < 0.1 {velocity.0.y = 0.0}

}

fn detect_wall_collisions(mut players: Query<(&Player, &mut Transform, &Sprite, &mut Velocity), Without<Immovable>>, immovables: Query<(&Transform, &Sprite), With<Immovable>>) {
    
    let (_, mut player_transform, player_sprite, mut player_velocity) = players.single_mut();

    for (immovable_transform, immovable_sprite) in immovables.iter() {

        let player_height = player_sprite.custom_size.unwrap().y;
        let player_width = player_sprite.custom_size.unwrap().x;
        let player_x = player_transform.translation.x - (player_width / 2.0);
        let player_y = player_transform.translation.y + (player_height / 2.0);
        let velocity_norm = player_velocity.0.normalize();

        let immovable_height = immovable_sprite.custom_size.unwrap().y;
        let immovable_width = immovable_sprite.custom_size.unwrap().x;
        let immovable_x = immovable_transform.translation.x - (immovable_width / 2.0);
        let immovable_y = immovable_transform.translation.y + (immovable_height / 2.0);

        // check for collision between player and immovable (this is simply using AABB collision detection)
        if player_x + player_width > immovable_x && player_x < immovable_x + immovable_width && player_y - player_height < immovable_y && player_y > immovable_y - immovable_height {
            // player moving right to wall
            if velocity_norm.x > 0.0 && velocity_norm.y == 0.0 {
                player_transform.translation.x = immovable_x - immovable_width + (player_width / 2.0);
                player_velocity.0.x = 0.0;
            }
            // player moving left to wall
            if velocity_norm.x < 0.0 && velocity_norm.y == 0.0 {
                player_transform.translation.x = immovable_x + immovable_width + (player_width / 2.0);
                player_velocity.0.x = 0.0;
            }
            // player moving down to wall
            if velocity_norm.y < 0.0 && velocity_norm.x == 0.0 {
                player_transform.translation.y = immovable_y + (player_height / 2.0);
                player_velocity.0.y = 0.0;
            }
            // player moving up to wall
            if velocity_norm.y > 0.0 && velocity_norm.x == 0.0 {
                player_transform.translation.y = immovable_y - immovable_height - (player_height / 2.0);
                player_velocity.0.y = 0.0;
            }

        }


    }
}
