use bevy::prelude::*;

mod components;
mod camera;
mod ai;
mod ui;
mod combat;
mod nadis;
mod celestial;
mod dev;

use crate::components::*;
use crate::camera::{handle_camera_orbit, camera_follow_player};
use crate::ai::{evaluate_ai_behavior, execute_ai_movement};
use crate::ui::{setup_hud, update_bio_metrics_hud, update_celestial_wheel};
use crate::combat::fractals::{handle_skill_inputs, handle_casting_pipeline, update_fractals};
use crate::combat::tessellations::{execute_ai_combat, handle_hit_flashes, update_tessellation_strikes};
use crate::combat::eclipses::{handle_eclipses, update_eclipse_fields};
use crate::nadis::{charge_ojas, handle_chakra_unblocking, monitor_consciousness, update_active_channel};
use crate::celestial::generate_profile_from_birth_data;
use crate::dev::DevModePlugin;

// ==========================================
// SETUP SYSTEMS
// ==========================================

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Classic Isometric OSRS Starting Angles (Yaw 45 deg, Pitch 35 deg)
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scale: 0.015,
            ..OrthographicProjection::default_3d() 
        }),
        GameCamera {
            yaw: 45.0f32.to_radians(),
            pitch: 35.0f32.to_radians(),
            distance: 12.0,
        },
        Transform::from_xyz(8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Sun Directional Light (Enables beautiful 3D highlights and shadows)
    commands.spawn((
        DirectionalLight {
            illuminance: 3500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground Floor (Unlit turned OFF to let the directional sun shade it)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d { 
            normal: Dir3::Y, 
            half_size: Vec2::new(10.0, 10.0) 
        })),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.35, 0.2), // Healthier grass color
            unlit: false, // Turned on standard shading!
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Bake Player's Celestial Profile from birth data (e.g., October 24th, 04:15 AM)
    let player_birth = BirthData {
        month: 10,
        day: 24,
        hour: 4,
        minute: 15,
    };
    let player_celestial = generate_profile_from_birth_data(&player_birth);
    println!("SUCCESS: Baked Player Celestial Profile: {:?}", player_celestial);

    // Player Entity (Shaded capsule to bring out its curved cylindrical form)
    commands.spawn((
        Player,
        MeridianHouse::PlayerHouse,
        Health { current: 100.0, max: 100.0 },
        Ojas { current: 50.0, max: 100.0 },
        Myelin { structural_density: 100.0, max_density: 100.0, throttled: false },
        NeuralFriction { ida_friction: 0.0, pingala_friction: 0.0, decay_constant: 15.0 },
        player_birth,
        player_celestial,
        TargetPosition(Vec3::ZERO), 
        ChakraTree {
            chakras: [
                Chakra { chakra_type: ChakraType::Root, unblocked: false, cost_to_unblock: 30.0 },
                Chakra { chakra_type: ChakraType::Sacral, unblocked: false, cost_to_unblock: 40.0 },
                Chakra { chakra_type: ChakraType::SolarPlexus, unblocked: false, cost_to_unblock: 50.0 },
                Chakra { chakra_type: ChakraType::Heart, unblocked: false, cost_to_unblock: 60.0 },
                Chakra { chakra_type: ChakraType::Throat, unblocked: false, cost_to_unblock: 70.0 },
                Chakra { chakra_type: ChakraType::ThirdEye, unblocked: false, cost_to_unblock: 80.0 },
                Chakra { chakra_type: ChakraType::Crown, unblocked: false, cost_to_unblock: 90.0 },
            ]
        },
        Mesh3d(meshes.add(Capsule3d::new(0.3, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.5, 0.15), // Deep Mars Orange
            unlit: false, // Let light shade the capsule curve
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0), 
    ));

}

fn spawn_target_dummy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Dummy 1: Arrogant Mars NPC (Aries Sun / Aries Ascendant)
    let dummy1_birth = BirthData {
        month: 3,
        day: 25,
        hour: 6,
        minute: 0,
    };
    let dummy1_celestial = generate_profile_from_birth_data(&dummy1_birth);
    println!("SUCCESS: Baked Dummy 1 Celestial Profile: {:?}", dummy1_celestial);

    commands.spawn((
        TargetDummy,
        MeridianHouse::Mars,
        Personality::Arrogant,
        AIController { 
            target: None, 
            cooldown: Timer::from_seconds(1.2, TimerMode::Once),
        },
        Health { current: 500.0, max: 500.0 },
        dummy1_birth,
        dummy1_celestial,
        Mesh3d(meshes.add(Cuboid::new(0.8, 1.0, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2), // Crimson
            unlit: false, // Shaded sides
            ..default()
        })),
        Transform::from_xyz(3.0, 0.5, 3.0),
    ));

    // Dummy 2: Timid Venus NPC (Pisces Sun / Gemini Ascendant)
    let dummy2_birth = BirthData {
        month: 3,
        day: 5,
        hour: 12,
        minute: 0,
    };
    let dummy2_celestial = generate_profile_from_birth_data(&dummy2_birth);
    println!("SUCCESS: Baked Dummy 2 Celestial Profile: {:?}", dummy2_celestial);

    commands.spawn((
        TargetDummy,
        MeridianHouse::Venus,
        Personality::Timid,
        AIController { 
            target: None, 
            cooldown: Timer::from_seconds(1.2, TimerMode::Once),
        },
        Health { current: 500.0, max: 500.0 },
        dummy2_birth,
        dummy2_celestial,
        Mesh3d(meshes.add(Cuboid::new(0.8, 1.0, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.2), // Yellow Gold
            unlit: false, // Shaded sides
            ..default()
        })),
        Transform::from_xyz(-3.0, 0.5, -3.0),
    ));
}

// ==========================================
// MAIN COMBAT & INPUT LOOPS (STAY IN MAIN)
// ==========================================

// Left and Right mouse click movement handling scrubbed completely

fn move_player_to_target(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &TargetPosition, &ChakraTree), With<Player>>,
) {
    for (mut transform, target, chakra_tree) in query.iter_mut() {
        let distance = transform.translation.distance(target.0);
        if distance > 0.05 {
            let direction = (target.0 - transform.translation).normalize();
            
            // SACRAL CHAKRA BUFF: Increases movement speed from 5.0 to 8.0 if unlocked!
            let mut move_speed = 5.0;
            if chakra_tree.chakras[1].unblocked {
                move_speed = 8.0;
            }

            let move_step = move_speed * time.delta_secs();
            
            if move_step > distance { transform.translation = target.0; } 
            else { transform.translation += direction * move_step; }
        }
    }
}

fn keyboard_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    binds: Res<MovementBinds>,
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut TargetPosition, &ChakraTree), (With<Player>, Without<Camera3d>)>,
) {
    let camera_transform = camera_query.single().unwrap();
    let (mut transform, mut target, chakra_tree) = player_query.single_mut().unwrap();

    let mut forward = camera_transform.forward().as_vec3();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let mut right = camera_transform.right().as_vec3();
    right.y = 0.0;
    right = right.normalize_or_zero();

    let mut move_dir = Vec3::ZERO;
    let mut input_active = false;

    if keyboard_input.pressed(binds.up) {
        move_dir += forward;
        input_active = true;
    }
    if keyboard_input.pressed(binds.down) {
        move_dir -= forward;
        input_active = true;
    }
    if keyboard_input.pressed(binds.left) {
        move_dir -= right;
        input_active = true;
    }
    if keyboard_input.pressed(binds.right) {
        move_dir += right;
        input_active = true;
    }

    if input_active && move_dir != Vec3::ZERO {
        move_dir = move_dir.normalize();

        let mut move_speed = 5.0;
        if chakra_tree.chakras[1].unblocked {
            move_speed = 8.0;
        }

        let move_step = move_dir * move_speed * time.delta_secs();
        transform.translation += move_step;
        target.0 = transform.translation;
    }
}

fn player_mouse_aim(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let (camera, camera_transform) = camera_query.single().unwrap();
    let window = window_query.single().unwrap();
    let mut player_transform = player_query.single_mut().unwrap();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
            if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) {
                let point = ray.get_point(distance);
                let target = Vec3::new(point.x, player_transform.translation.y, point.z);
                if player_transform.translation.distance(target) > 0.01 {
                    player_transform.look_at(target, Vec3::Y);
                }
            }
        }
    }
}

// ==========================================
// MAIN RUNNER
// ==========================================

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan");
    }
    
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DevModePlugin)
        .init_resource::<MovementBinds>()
        .init_resource::<ActiveChannelState>()
        .init_resource::<ActiveCastState>()
        .add_systems(Startup, (setup, setup_hud, spawn_target_dummy))
        .add_systems(
            Update,
            (
                keyboard_movement,
                player_mouse_aim,
                move_player_to_target, 
                charge_ojas, 
                update_active_channel,
                handle_skill_inputs,
                handle_casting_pipeline,
                update_bio_metrics_hud,
                update_nadi_hud,
                update_celestial_wheel,
                update_fractals,
            ),
        )
        .add_systems(
            Update,
            (
                handle_hit_flashes,
                handle_eclipses,
                update_tessellation_strikes,
                update_eclipse_fields,
                evaluate_ai_behavior,
                execute_ai_movement,
                execute_ai_combat,
                monitor_consciousness,
                handle_camera_orbit,
                camera_follow_player,
                handle_chakra_unblocking,
            ),
        )
        .run();
}