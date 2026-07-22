use bevy::prelude::*;
use crate::components::*;
use crate::nadis::apply_exhaustion_damage_if_applicable;

fn get_procedural_name(channel: NadiChannel, category: SkillCategory) -> String {
    let prefix = match (category, channel) {
        (SkillCategory::SunAligned, NadiChannel::Ida) => "Creeping Shroud",
        (SkillCategory::SunAligned, NadiChannel::Pingala) => "Oxidized Iron",
        (SkillCategory::MoonAligned, NadiChannel::Ida) => "Vortex Shadow",
        (SkillCategory::MoonAligned, NadiChannel::Pingala) => "Tectonic",
        (SkillCategory::AscendantAligned, NadiChannel::Ida) => "Graceful Zephyr",
        (SkillCategory::AscendantAligned, NadiChannel::Pingala) => "Gale Force",
        (_, NadiChannel::Sushumna) => "Unified",
    };
    let suffix = match category {
        SkillCategory::SunAligned => "Fractal",
        SkillCategory::MoonAligned => "Eclipse",
        SkillCategory::AscendantAligned => "Tessellation",
    };
    format!("{} {}", prefix, suffix)
}

pub fn handle_skill_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut active_cast: ResMut<ActiveCastState>,
    active_channel: Res<ActiveChannelState>,
    player_query: Query<(&Ojas, &NeuralFriction, &CelestialProfile), With<Player>>,
    dev_state: Res<DevModeState>,
    focus_query: Query<&ActiveInputFocus>,
) {
    if !focus_query.is_empty() {
        return;
    }

    if active_cast.casting_skill.is_some() {
        return;
    }

    let channel = active_channel.current;
    if channel != NadiChannel::Ida && channel != NadiChannel::Pingala {
        return;
    }

    let Ok((ojas, friction, player_profile)) = player_query.single() else {
        return;
    };

    let friction_val = match channel {
        NadiChannel::Ida => friction.ida_friction,
        NadiChannel::Pingala => friction.pingala_friction,
        _ => 0.0,
    };
    let is_overheated = friction_val > 75.0;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        if active_cast.lockout_tessellation > 0.0 {
            println!("Skill category locked out!");
            return;
        }
        let native_sign = dev_state.tessellation_override;
        let player_sign = player_profile.ascendant_sign;
        let is_resonant = native_sign == player_sign;
        let mut base_cost = 15.0;
        if is_overheated {
            base_cost *= 2.0;
            active_cast.flash_amber_tessellation = 0.5;
            println!("WARNING: Ida/Pingala gate overheated! Tessellation base cost doubled to {:.1}", base_cost);
        }
        let mut final_cost = base_cost * (1.0 + (friction_val / ojas.max));
        if is_resonant {
            final_cost *= 0.85;
        }
        if ojas.current < final_cost {
            println!("Not enough Ojas! Requires {:.1}", final_cost);
            return;
        }
        active_cast.casting_skill = Some(SkillCategory::AscendantAligned);
        active_cast.timer = 0.0;
        active_cast.active_channel = Some(channel);
        println!("Starting cast of Tessellation under channel {:?}", channel);
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        if active_cast.lockout_fractal > 0.0 {
            println!("Skill category locked out!");
            return;
        }
        let native_sign = dev_state.fractal_override;
        let player_sign = player_profile.sun_sign;
        let is_resonant = native_sign == player_sign;
        let mut base_cost = 15.0;
        if is_overheated {
            base_cost *= 2.0;
            active_cast.flash_amber_fractal = 0.5;
            println!("WARNING: Ida/Pingala gate overheated! Fractal base cost doubled to {:.1}", base_cost);
        }
        let mut final_cost = base_cost * (1.0 + (friction_val / ojas.max));
        if is_resonant {
            final_cost *= 0.85;
        }
        if ojas.current < final_cost {
            println!("Not enough Ojas! Requires {:.1}", final_cost);
            return;
        }
        active_cast.casting_skill = Some(SkillCategory::SunAligned);
        active_cast.timer = 0.0;
        active_cast.active_channel = Some(channel);
        println!("Starting cast of Fractal under channel {:?}", channel);
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        if active_cast.lockout_eclipse > 0.0 {
            println!("Skill category locked out!");
            return;
        }
        let native_sign = dev_state.eclipse_override;
        let player_sign = player_profile.moon_sign;
        let is_resonant = native_sign == player_sign;
        let mut base_cost = 30.0;
        if is_overheated {
            base_cost *= 2.0;
            active_cast.flash_amber_eclipse = 0.5;
            println!("WARNING: Ida/Pingala gate overheated! Eclipse base cost doubled to {:.1}", base_cost);
        }
        let mut final_cost = base_cost * (1.0 + (friction_val / ojas.max));
        if is_resonant {
            final_cost *= 0.85;
        }
        if ojas.current < final_cost {
            println!("Not enough Ojas! Requires {:.1}", final_cost);
            return;
        }
        active_cast.casting_skill = Some(SkillCategory::MoonAligned);
        active_cast.timer = 0.0;
        active_cast.active_channel = Some(channel);
        println!("Starting cast of Eclipse under channel {:?}", channel);
    }
}

pub fn handle_casting_pipeline(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    binds: Res<MovementBinds>,
    dev_state: Res<DevModeState>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut player_query: Query<(Entity, &Transform, &mut Ojas, &ChakraTree, &mut Myelin, &mut NeuralFriction, &Health, &CelestialProfile), (With<Player>, Without<TargetDummy>)>,
    mut dummy_query: Query<(Entity, &Transform, &mut Health, &MeshMaterial3d<StandardMaterial>, Option<&EclipseDebuff>), (With<TargetDummy>, Without<Player>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut active_cast: ResMut<ActiveCastState>,
    focus_query: Query<&ActiveInputFocus>,
    mut last_health: Local<f32>,
) {
    let dt = time.delta_secs();

    active_cast.ida_pulse = (active_cast.ida_pulse - dt).max(0.0);
    active_cast.pingala_pulse = (active_cast.pingala_pulse - dt).max(0.0);

    active_cast.lockout_tessellation = (active_cast.lockout_tessellation - dt).max(0.0);
    active_cast.lockout_fractal = (active_cast.lockout_fractal - dt).max(0.0);
    active_cast.lockout_eclipse = (active_cast.lockout_eclipse - dt).max(0.0);

    active_cast.flash_amber_tessellation = (active_cast.flash_amber_tessellation - dt).max(0.0);
    active_cast.flash_amber_fractal = (active_cast.flash_amber_fractal - dt).max(0.0);
    active_cast.flash_amber_eclipse = (active_cast.flash_amber_eclipse - dt).max(0.0);

    if let Some(casting) = active_cast.casting_skill {
        let Ok((_player_entity, player_transform, mut ojas, chakra_tree, mut myelin, mut friction, player_health, player_profile)) = player_query.single_mut() else {
            return;
        };

        if *last_health == 0.0 {
            *last_health = player_health.current;
        }

        let mut interrupted = false;

        if player_health.current < *last_health {
            interrupted = true;
            println!("Cast Interrupted! Player took damage.");
        }

        if (casting == SkillCategory::SunAligned || casting == SkillCategory::MoonAligned) && focus_query.is_empty() {
            let is_moving = keyboard_input.pressed(binds.up) || 
                            keyboard_input.pressed(binds.left) || 
                            keyboard_input.pressed(binds.down) || 
                            keyboard_input.pressed(binds.right);
            if is_moving {
                interrupted = true;
                println!("Cast Interrupted! Player moved during stationary cast.");
            }
        }

        if interrupted {
            active_cast.casting_skill = None;
            active_cast.timer = 0.0;
            match casting {
                SkillCategory::AscendantAligned => {
                    active_cast.lockout_tessellation = 2.0;
                    active_cast.flash_amber_tessellation = 0.5;
                }
                SkillCategory::SunAligned => {
                    active_cast.lockout_fractal = 2.0;
                    active_cast.flash_amber_fractal = 0.5;
                }
                SkillCategory::MoonAligned => {
                    active_cast.lockout_eclipse = 2.0;
                    active_cast.flash_amber_eclipse = 0.5;
                }
            }
        } else {
            active_cast.timer += dt;

            let prep_time = match casting {
                SkillCategory::AscendantAligned => 0.1,
                SkillCategory::SunAligned => 0.6,
                SkillCategory::MoonAligned => 1.8,
            };

            if active_cast.timer >= prep_time {
                let channel = active_cast.active_channel.unwrap_or(NadiChannel::Sushumna);
                let name = get_procedural_name(channel, casting);
                println!("SUCCESSFUL CAST: {}", name);

                let native_sign = match casting {
                    SkillCategory::AscendantAligned => dev_state.tessellation_override,
                    SkillCategory::SunAligned => dev_state.fractal_override,
                    SkillCategory::MoonAligned => dev_state.eclipse_override,
                };

                let player_triad_sign = match casting {
                    SkillCategory::SunAligned => player_profile.sun_sign,
                    SkillCategory::MoonAligned => player_profile.moon_sign,
                    SkillCategory::AscendantAligned => player_profile.ascendant_sign,
                };
                let is_resonant = native_sign == player_triad_sign;

                let is_overheated = match channel {
                    NadiChannel::Ida => friction.ida_friction > 75.0,
                    NadiChannel::Pingala => friction.pingala_friction > 75.0,
                    _ => false,
                };

                let base_cost = match casting {
                    SkillCategory::AscendantAligned => 15.0,
                    SkillCategory::SunAligned => 15.0,
                    SkillCategory::MoonAligned => 30.0,
                };
                let mut final_cost = base_cost;
                if is_overheated {
                    final_cost *= 2.0;
                }

                let friction_val = match channel {
                    NadiChannel::Ida => friction.ida_friction,
                    NadiChannel::Pingala => friction.pingala_friction,
                    _ => 0.0,
                };

                final_cost = final_cost * (1.0 + (friction_val / ojas.max));
                if is_resonant {
                    final_cost *= 0.85;
                }

                if ojas.current >= final_cost {
                    apply_exhaustion_damage_if_applicable(&mut ojas, &mut myelin, &name);

                    let mut final_cost_checked = match casting {
                        SkillCategory::AscendantAligned => 15.0,
                        SkillCategory::SunAligned => 15.0,
                        SkillCategory::MoonAligned => 30.0,
                    };
                    if is_overheated {
                        final_cost_checked *= 2.0;
                    }
                    final_cost_checked = final_cost_checked * (1.0 + (friction_val / ojas.max));
                    if is_resonant {
                        final_cost_checked *= 0.85;
                    }

                    if ojas.current >= final_cost_checked {
                        ojas.current -= final_cost_checked;

                        match channel {
                            NadiChannel::Ida => {
                                friction.ida_friction += 20.0;
                                active_cast.ida_pulse = 0.25;
                            }
                            NadiChannel::Pingala => {
                                friction.pingala_friction += 20.0;
                                active_cast.pingala_pulse = 0.25;
                            }
                            _ => {}
                        }

                        match casting {
                            SkillCategory::AscendantAligned => {
                                let mut hit_any = false;
                                for (dummy_entity, dummy_transform, mut dummy_health, mat_handle, _) in dummy_query.iter_mut() {
                                    if player_transform.translation.distance(dummy_transform.translation) < 1.5 {
                                        if !hit_any {
                                            hit_any = true;
                                            let mut strike_cmd = commands.spawn((
                                                TessellationStrike {
                                                    native_sign,
                                                    category: casting,
                                                    lifespan: Timer::from_seconds(0.2, TimerMode::Once),
                                                },
                                                Transform::from_translation(player_transform.translation),
                                            ));
                                            if is_resonant {
                                                strike_cmd.insert(ResonantGlow);
                                            }
                                        }
                                        dummy_health.current -= 50.0;
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
                            SkillCategory::SunAligned => {
                                let (camera, camera_transform) = camera_query.single().unwrap();
                                let window = window_query.single().unwrap();
                                let mut direction = Vec3::ZERO;
                                
                                if let Some(cursor_pos) = window.cursor_position() {
                                    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                                        if let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) {
                                            let point = ray.get_point(distance);
                                            let target_pos = Vec3::new(point.x, 0.5, point.z);
                                            direction = (target_pos - player_transform.translation).normalize_or_zero();
                                        }
                                    }
                                }
                                if direction == Vec3::ZERO {
                                    direction = player_transform.forward().as_vec3();
                                    direction.y = 0.0;
                                    direction = direction.normalize_or_zero();
                                }

                                let mut dmg_value = 100.0;
                                if chakra_tree.chakras[4].unblocked {
                                    dmg_value = 150.0;
                                }

                                let mut proj_cmd = commands.spawn((
                                    FractalProjectile {
                                        direction,
                                        speed: 12.0,
                                        damage: dmg_value,
                                        lifespan: Timer::from_seconds(3.0, TimerMode::Once),
                                        native_sign,
                                        category: casting,
                                    },
                                    Mesh3d(meshes.add(Sphere::new(0.2))),
                                    MeshMaterial3d(materials.add(StandardMaterial {
                                        base_color: Color::srgb(1.0, 0.1, 0.1),
                                        unlit: true,
                                        ..default()
                                    })),
                                    Transform::from_translation(player_transform.translation + direction * 0.5),
                                ));

                                if is_resonant {
                                    proj_cmd.insert(ResonantGlow);
                                }
                            }
                            SkillCategory::MoonAligned => {
                                let mut field_cmd = commands.spawn((
                                    EclipseField {
                                        native_sign,
                                        category: casting,
                                        lifespan: Timer::from_seconds(4.0, TimerMode::Once),
                                    },
                                    Transform::from_translation(player_transform.translation),
                                ));
                                if is_resonant {
                                    field_cmd.insert(ResonantGlow);
                                }

                                for (dummy_entity, dummy_transform, _, mat_handle, debuff) in dummy_query.iter_mut() {
                                    if debuff.is_none() && player_transform.translation.distance(dummy_transform.translation) < 5.0 {
                                        if let Some(mat) = materials.get_mut(mat_handle) {
                                            commands.entity(dummy_entity).insert(EclipseDebuff {
                                                timer: Timer::from_seconds(4.0, TimerMode::Once),
                                                original_color: mat.base_color,
                                            });
                                            mat.base_color = Color::srgb(0.15, 0.0, 0.3);
                                        }
                                    }
                                }
                            }
                        }

                        match casting {
                            SkillCategory::AscendantAligned => {
                                active_cast.lockout_tessellation = 3.0;
                            }
                            SkillCategory::SunAligned => {
                                active_cast.lockout_fractal = 6.0;
                            }
                            SkillCategory::MoonAligned => {
                                active_cast.lockout_eclipse = 12.0;
                            }
                        }
                    }
                } else {
                    println!("Not enough Ojas to finish cast!");
                }

                active_cast.casting_skill = None;
                active_cast.active_channel = None;
                active_cast.timer = 0.0;
            }
        }
    }

    if let Ok((_, _, _, _, _, _, player_health, _)) = player_query.single() {
        *last_health = player_health.current;
    }
}

pub fn update_fractals(
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
