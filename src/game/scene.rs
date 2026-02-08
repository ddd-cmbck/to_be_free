// src/game/scene.rs
use bevy::prelude::*;

/// Sets up a minimal 3D scene:
/// - a ground base
/// - a visible cube (so you can immediately see lighting / depth)
/// - one point light (shadows on by default)
/// - one 3D camera looking at the origin
///
/// Bevy 0.18 note:
/// The official examples use `Mesh3d` + `MeshMaterial3d` instead of `PbrBundle`.
/// This is the most "current" style and keeps the spawn tuples minimal. :contentReference[oaicite:0]{index=0}
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground base (a circle rotated to lie on the XZ plane).
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(6.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // A cube at the origin, raised by half its height so it rests on the ground.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Light.
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
