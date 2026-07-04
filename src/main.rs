use bevy::prelude::*;

// ==========================================
// 1. COMPONENTS, ENUMS & RESOURCES
// ==========================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ojas {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct TargetPosition(pub Vec3);

#[derive(Component)]
pub struct OjasTextUi;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct TargetDummy;

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub original_color: Color,
}

#[derive(Component)]
pub struct EclipseDebuff {
    pub timer: Timer,
    pub original_color: Color,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum MeridianHouse {
    Mars,
    Venus,
    Jupiter,
    PlayerHouse, // Your faction
}

#[derive(Component)]
pub enum Personality {
    Arrogant, // Will always attack rivals
    Timid,    // Will always run from rivals
}

#[derive(Component)]
pub struct AIController {
    pub target: Option<Vec3>, // Where the AI currently wants to walk
}

// Dynamic camera orbit controls component
#[derive(Component)]
pub struct GameCamera {
    pub yaw: f32,      // Orbit angle around the vertical Y axis
    pub pitch: f32,    // Vertical angle (tilt) looking down
    pub distance: f32, // Distance from the player target
}

// Projectile Component for our Fractals
#[derive(Component)]
pub struct FractalProjectile {
    pub direction: Vec3,
    pub speed: f32,
    pub damage: f32,
    pub lifespan: Timer, // To despawn it if it misses and flies off the map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChakraType {
    Root,         // Stability (Increases base Health)
    Sacral,       // Flow (Increases movement speed)
    SolarPlexus,  // Will (Increases maximum Ojas)
    Heart,        // Connection (Unlocks passive aura regeneration)
    Throat,       // Expression (Increases Fractal projectile damage)
    ThirdEye,     // Vision (Improves visual overlay range)
    Crown,        // Ascension (Prepares user for Kundalini awakening)
}

#[derive(Clone, Copy)]
pub struct Chakra {
    pub chakra_type: ChakraType,
    pub unblocked: bool,
    pub cost_to_unblock: f32,
}

#[derive(Component)]
pub struct ChakraTree {
    pub chakras: [Chakra; 7],
}

// ==========================================
// 2. SETUP SYSTEMS
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

    // Player Entity (Shaded capsule to bring out its curved cylindrical form)
    commands.spawn((
        Player,
        MeridianHouse::PlayerHouse,
        Ojas { current: 50.0, max: 100.0 },
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

    // Retro UI
    commands.spawn((
        OjasTextUi,
        Text::new("Ojas: 50.0 / 100.0\n[Space] Breathe\n[Right Click] Move\n[1] Tessellation\n[2] Mars Fractal\n[3] Eclipse\n[4] Unblock Next Chakra\n[Arrows] Adjust Camera Tilt/Angle"),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0), left: Val::Px(10.0), ..default()
        },
    ));
}

fn spawn_target_dummy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Dummy 1: Arrogant Mars NPC (Fully shaded cube)
    commands.spawn((
        TargetDummy,
        MeridianHouse::Mars,
        Personality::Arrogant,
        AIController { target: None },
        Health { current: 500.0, max: 500.0 },
        Mesh3d(meshes.add(Cuboid::new(0.8, 1.0, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2), // Crimson
            unlit: false, // Shaded sides
            ..default()
        })),
        Transform::from_xyz(3.0, 0.5, 3.0),
    ));

    // Dummy 2: Timid Venus NPC (Fully shaded cube)
    commands.spawn((
        TargetDummy,
        MeridianHouse::Venus,
        Personality::Timid,
        AIController { target: None },
        Health { current: 500.0, max: 500.0 },
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
// 3. LOGIC & COMBAT SYSTEMS
// ==========================================

fn handle_mouse_clicks(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<&mut TargetPosition, With<Player>>,
) {
    if mouse_button.just_pressed(MouseButton::Right) {
        let Ok((camera, camera_transform)) = camera_query.single() else { return; };
        let Ok(window) = window_query.single() else { return; };

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) {
                    let point = ray.get_point(distance);
                    if let Ok(mut target) = player_query.single_mut() {
                        target.0 = Vec3::new(point.x, 0.5, point.z); 
                    }
                }
            }
        }
    }
}

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

fn handle_camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut GameCamera>,
) {
    if let Ok(mut camera) = camera_query.single_mut() {
        let rotation_speed = 2.0; // Radians per second
        
        // Yaw (Horizontal orbit)
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera.yaw -= rotation_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera.yaw += rotation_speed * time.delta_secs();
        }
        
        // Pitch (Vertical tilt angle)
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // Keep the camera from flipping completely upside down (Limit to 85 degrees)
            camera.pitch = (camera.pitch + rotation_speed * time.delta_secs()).min(85.0f32.to_radians());
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            // Keep the camera from clipping flat onto the ground (Limit to 5 degrees)
            camera.pitch = (camera.pitch - rotation_speed * time.delta_secs()).max(5.0f32.to_radians());
        }
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<(&mut Transform, &GameCamera), With<Camera>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok((mut camera_transform, game_cam)) = camera_query.single_mut() {
            // Convert Yaw and Pitch spherical coordinates to standard 3D Cartesian coordinates
            let pitch_cos = game_cam.pitch.cos();
            
            // Calculate orbital offset vector around player
            let offset = Vec3::new(
                game_cam.yaw.sin() * pitch_cos,
                game_cam.pitch.sin(),
                game_cam.yaw.cos() * pitch_cos,
            ) * game_cam.distance;
            
            let target_camera_pos = player_transform.translation + offset;
            
            // Smoothly pan camera center with Lerp
            let lerp_factor = 8.0 * time.delta_secs();
            camera_transform.translation = camera_transform.translation.lerp(target_camera_pos, lerp_factor);
            
            // FIX: Using look_at (mutating in-place) instead of looking_at (builder returning copy)
            // This corrects the landscape tilting and centers the camera securely onto the player player position.
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

fn evaluate_ai_behavior(
    mut ai_query: Query<(&mut AIController, &Transform, &Personality, &MeridianHouse)>,
    player_query: Query<(&Transform, &MeridianHouse), With<Player>>,
) {
    if let Ok((player_transform, player_house)) = player_query.single() {
        for (mut ai, ai_transform, personality, ai_house) in ai_query.iter_mut() {
            
            // If they are from different houses, they are rivals
            if ai_house != player_house {
                let distance_to_player = ai_transform.translation.distance(player_transform.translation);
                
                // If the player is within 4 units, trigger a reaction
                if distance_to_player < 4.0 {
                    match personality {
                        Personality::Arrogant => {
                            // Arrogant AI sets their target to the player's current location
                            ai.target = Some(player_transform.translation);
                        }
                        Personality::Timid => {
                            // Timid AI runs in the exact opposite direction
                            let run_dir = (ai_transform.translation - player_transform.translation).normalize_or_zero();
                            ai.target = Some(ai_transform.translation + run_dir * 3.0);
                        }
                    }
                } else {
                    // If the player is far away, the AI stops moving
                    ai.target = None;
                }
            }
        }
    }
}

fn execute_ai_movement(
    time: Res<Time>,
    mut ai_query: Query<(&mut Transform, &AIController)>,
) {
    for (mut transform, ai) in ai_query.iter_mut() {
        if let Some(target_pos) = ai.target {
            let distance = transform.translation.distance(target_pos);
            if distance > 0.05 {
                let direction = (target_pos - transform.translation).normalize_or_zero();
                let move_step = 3.0 * time.delta_secs(); // AI moves slightly slower than the player
                
                if move_step > distance { 
                    transform.translation = target_pos; 
                } else { 
                    transform.translation += direction * move_step; 
                }
            }
        }
    }
}

fn charge_ojas(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Ojas, &ChakraTree), With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if let Ok((mut ojas, chakra_tree)) = query.single_mut() {
            // HEART CHAKRA BUFF: Double breathing charge speed if unblocked!
            let mut rate_multiplier = 1.0;
            if chakra_tree.chakras[3].unblocked {
                rate_multiplier = 2.0;
            }

            ojas.current = (ojas.current + 30.0 * rate_multiplier * time.delta_secs()).min(ojas.max);
        }
    }
}

fn handle_chakra_unblocking(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Ojas, &mut ChakraTree), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        if let Ok((mut ojas, mut chakra_tree)) = player_query.single_mut() {
            // Find the first blocked chakra in the sequence
            if let Some(next_chakra) = chakra_tree.chakras.iter_mut().find(|c| !c.unblocked) {
                if ojas.current >= next_chakra.cost_to_unblock {
                    ojas.current -= next_chakra.cost_to_unblock;
                    next_chakra.unblocked = true;
                    println!("SUCCESS: Unblocked {:?} Chakra!", next_chakra.chakra_type);

                    // Apply immediate permanent upgrades based on unblocked chakra
                    match next_chakra.chakra_type {
                        ChakraType::Root => {
                            // ROOT CHAKRA BUFF: Increases max Ojas capacity
                            ojas.max += 25.0;
                            println!("Root Chakra active: Maximum Ojas capacity expanded by 25!");
                        }
                        ChakraType::Sacral => {
                            println!("Sacral Chakra active: Movement speed permanently boosted to 8.0!");
                        }
                        ChakraType::SolarPlexus => {
                            println!("Solar Plexus Chakra active: Ojas pools fully refreshed!");
                            ojas.current = ojas.max;
                        }
                        ChakraType::Heart => {
                            println!("Heart Chakra active: Spooky breathing charges Vril pools 2x faster!");
                        }
                        _ => {
                            println!("Aperture Synced Node Activated: High visual metrics enhanced.");
                        }
                    }
                } else {
                    println!("Not enough Ojas! Requires {:.1} Ojas to unblock next Node.", next_chakra.cost_to_unblock);
                }
            } else {
                println!("All 7 Earthly Chakras unblocked! Ready for cosmic constellation binding.");
            }
        }
    }
}

fn cast_eclipse(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Ojas), With<Player>>,
    mut dummy_query: Query<(Entity, &Transform, &MeshMaterial3d<StandardMaterial>), (With<TargetDummy>, Without<EclipseDebuff>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        if let Ok((player_transform, mut ojas)) = player_query.single_mut() {
            let cost = 25.0;
            if ojas.current < cost {
                println!("Not enough Ojas for an Eclipse!");
                return;
            }
            
            ojas.current -= cost;
            println!("Eclipse Cast! Sensory Deprivation Field expanding...");

            // Apply to all dummies in a 5.0 unit radius
            for (dummy_entity, dummy_transform, mat_handle) in dummy_query.iter_mut() {
                if player_transform.translation.distance(dummy_transform.translation) < 5.0 {
                    if let Some(mat) = materials.get_mut(mat_handle) {
                        commands.entity(dummy_entity).insert(EclipseDebuff {
                            timer: Timer::from_seconds(4.0, TimerMode::Once),
                            original_color: mat.base_color,
                        });
                        
                        // Turn them void-purple standard color
                        mat.base_color = Color::srgb(0.15, 0.0, 0.3); 
                        println!("Dummy trapped in Eclipse!");
                    }
                }
            }
        }
    }
}

fn cast_tessellation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Ojas), With<Player>>,
    mut dummy_query: Query<(Entity, &Transform, &mut Health, &MeshMaterial3d<StandardMaterial>), With<TargetDummy>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        for (player_transform, mut ojas) in player_query.iter_mut() {
            let cost = 15.0;
            if ojas.current < cost { println!("Not enough Ojas!"); continue; }

            for (dummy_entity, dummy_transform, mut dummy_health, mat_handle) in dummy_query.iter_mut() {
                if player_transform.translation.distance(dummy_transform.translation) < 1.5 {
                    ojas.current -= cost;
                    dummy_health.current -= 50.0;
                    println!("Tessellation Connected! Dummy HP: {:.1}", dummy_health.current);
                    
                    if let Some(mat) = materials.get_mut(mat_handle) {
                        commands.entity(dummy_entity).insert(HitFlash {
                            timer: Timer::from_seconds(0.05, TimerMode::Once),
                            original_color: mat.base_color,
                        });
                        mat.base_color = Color::WHITE; 
                    }
                }
            }
        }
    }
}

fn cast_fractal(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut player_query: Query<(&Transform, &mut Ojas, &ChakraTree), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        let Ok((camera, camera_transform)) = camera_query.single() else { return; };
        let Ok(window) = window_query.single() else { return; };

        // Raycast against ground plane under dynamic camera rotation
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) {
                    let point = ray.get_point(distance);
                    let target_pos = Vec3::new(point.x, 0.5, point.z);

                    for (player_transform, mut ojas, chakra_tree) in player_query.iter_mut() {
                        let cost = 30.0;
                        if ojas.current < cost { 
                            println!("Not enough Ojas for a Fractal!"); 
                            continue; 
                        }

                        ojas.current -= cost;
                        println!("Mars Fractal Fired!");

                        let direction = (target_pos - player_transform.translation).normalize_or_zero();

                        // THROAT CHAKRA BUFF: Unblocked node scales default projectile damage to 150.0!
                        let mut dmg_value = 100.0;
                        if chakra_tree.chakras[4].unblocked {
                            dmg_value = 150.0;
                        }

                        // Spawn the glowing 3D projectile
                        commands.spawn((
                            FractalProjectile {
                                direction,
                                speed: 12.0,
                                damage: dmg_value,
                                lifespan: Timer::from_seconds(3.0, TimerMode::Once),
                            },
                            Mesh3d(meshes.add(Sphere::new(0.2))),
                            MeshMaterial3d(materials.add(StandardMaterial {
                                base_color: Color::srgb(1.0, 0.1, 0.1), // Mars red
                                unlit: true, // Projectiles glow, ignore shadows
                                ..default()
                            })),
                            Transform::from_translation(player_transform.translation + direction * 0.5),
                        ));
                    }
                }
            }
        }
    }
}

fn update_fractals(
    time: Res<Time>,
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &mut FractalProjectile, &mut Transform)>,
        Query<(Entity, &Transform, &mut Health, &MeshMaterial3d<StandardMaterial>), With<TargetDummy>>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dummies: Vec<(Entity, Transform)> = set.p1().iter().map(|(e, t, _, _)| (e, *t)).collect();
    let mut projectiles_to_despawn = vec![];
    let mut dummies_hit: Vec<(Entity, f32)> = vec![];

    for (proj_entity, mut proj, mut proj_transform) in set.p0().iter_mut() {
        proj_transform.translation += proj.direction * proj.speed * time.delta_secs();
        
        proj.lifespan.tick(time.delta());
        if proj.lifespan.just_finished() {
            projectiles_to_despawn.push(proj_entity);
            continue;
        }

        let mut hit = false;
        for (dummy_entity, dummy_transform) in &dummies {
            if proj_transform.translation.distance(dummy_transform.translation) < 0.8 {
                dummies_hit.push((*dummy_entity, proj.damage));
                hit = true;
                break;
            }
        }

        if hit {
            projectiles_to_despawn.push(proj_entity);
        }
    }
    
    for (dummy_entity, damage) in dummies_hit {
        if let Ok((_entity, _transform, mut health, mat_handle)) = set.p1().get_mut(dummy_entity) {
            health.current -= damage;
            println!("Mars Fractal Connected! Massive Damage! Dummy HP: {:.1}", health.current);

            if let Some(mat) = materials.get_mut(mat_handle) {
                commands.entity(dummy_entity).insert(HitFlash {
                    timer: Timer::from_seconds(0.05, TimerMode::Once),
                    original_color: mat.base_color,
                });
                mat.base_color = Color::WHITE;
            }
        }
    }

    for proj_entity in projectiles_to_despawn {
        commands.entity(proj_entity).despawn();
    }
}

fn handle_hit_flashes(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HitFlash, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut flash, mat_handle) in query.iter_mut() {
        flash.timer.tick(time.delta());
        if flash.timer.just_finished() {
            if let Some(mat) = materials.get_mut(mat_handle) {
                mat.base_color = flash.original_color;
            }
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

fn handle_eclipses(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut EclipseDebuff, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut debuff, mat_handle) in query.iter_mut() {
        debuff.timer.tick(time.delta());
        
        if debuff.timer.just_finished() {
            if let Some(mat) = materials.get_mut(mat_handle) {
                mat.base_color = debuff.original_color;
            }
            commands.entity(entity).remove::<EclipseDebuff>();
            println!("Dummy broke free from the Eclipse!");
        }
    }
}

fn update_ui(
    player_query: Query<(&Ojas, &ChakraTree), With<Player>>,
    mut ui_query: Query<&mut Text, With<OjasTextUi>>,
) {
    if let Ok((ojas, chakra_tree)) = player_query.single() {
        if let Ok(mut text) = ui_query.single_mut() {
            // Count unblocked chakras to output in UI
            let unblocked_count = chakra_tree.chakras.iter().filter(|c| c.unblocked).count();
            
            text.0 = format!(
                "Ojas: {:.1} / {:.1}\nChakras Opened: {}/7\n[Space] Breathe\n[Right Click] Move\n[1] Tessellation\n[2] Mars Fractal\n[3] Eclipse\n[4] Unblock Next Chakra\n[Arrows] Adjust Camera Tilt/Angle",
                ojas.current, ojas.max, unblocked_count
            );
        }
    }
}

// ==========================================
// 4. MAIN RUNNER
// ==========================================

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_target_dummy))
        .add_systems(
            Update,
            (
                handle_mouse_clicks, 
                move_player_to_target, 
                charge_ojas, 
                update_ui,
                cast_tessellation,
                cast_fractal,
                cast_eclipse,
                update_fractals,
                handle_hit_flashes,
                handle_eclipses,
                evaluate_ai_behavior,
                execute_ai_movement,
                // Dynamically modify tracking values and re-orient vectors dynamically
                handle_camera_controls,
                camera_follow_player,
                handle_chakra_unblocking,
            ),
        )
        .run();
}