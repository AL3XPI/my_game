use bevy::prelude::*;
use crate::components::*;

pub fn apply_exhaustion_damage_if_applicable(_ojas: &mut Ojas, _myelin: &mut Myelin, _action_name: &str) {
    // Myelin integrity is safely protected at 100% during baseline usage.
}

pub fn charge_ojas(
    time: Res<Time>,
    mut query: Query<(&mut Ojas, &ChakraTree, &Myelin, &mut NeuralFriction), With<Player>>,
    active_cast: Res<ActiveCastState>,
) {
    let (mut ojas, chakra_tree, myelin, mut friction) = query.single_mut().unwrap();
    
    if myelin.structural_density > 0.0 {
        let base_regen = 15.0;
        let mut rate_multiplier = 1.0;
        if chakra_tree.chakras[3].unblocked {
            rate_multiplier = 2.0;
        }

        let regeneration_rate = base_regen * (myelin.structural_density / myelin.max_density) * rate_multiplier;
        ojas.current = (ojas.current + regeneration_rate * time.delta_secs()).min(ojas.max);
    }
    
    let actively_molding_channel = active_cast.active_channel;
    
    if actively_molding_channel != Some(NadiChannel::Ida) {
        friction.ida_friction = (friction.ida_friction - friction.decay_constant * time.delta_secs()).max(0.0);
    }
    if actively_molding_channel != Some(NadiChannel::Pingala) {
        friction.pingala_friction = (friction.pingala_friction - friction.decay_constant * time.delta_secs()).max(0.0);
    }
}

pub fn handle_chakra_unblocking(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Ojas, &mut ChakraTree, &mut Myelin), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        if let Ok((mut ojas, mut chakra_tree, mut myelin)) = player_query.single_mut() {
            apply_exhaustion_damage_if_applicable(&mut ojas, &mut myelin, "Chakra Unblock");

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

pub fn monitor_consciousness(
    player_query: Query<(&Health, &Myelin), With<Player>>,
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    let (health, myelin) = player_query.single().unwrap();
    if health.current <= 0.0 || myelin.structural_density <= 0.0 {
        let t = timer.get_or_insert_with(|| Timer::from_seconds(1.0, TimerMode::Repeating));
        t.tick(time.delta());
        if t.just_finished() || t.elapsed_secs() == 0.0 {
            println!("[EVENT LOOP] System Shock: Consciousness Severed");
        }
    } else {
        *timer = None;
    }
}

pub fn update_active_channel(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut active_channel: ResMut<ActiveChannelState>,
    binds: Res<MovementBinds>,
    focus_query: Query<&ActiveInputFocus>,
) {
    if !focus_query.is_empty() {
        return;
    }
    let (left_key, right_key) = if binds.up == KeyCode::KeyW {
        (KeyCode::KeyQ, KeyCode::KeyE)
    } else {
        (KeyCode::KeyO, KeyCode::BracketLeft)
    };

    if keyboard_input.just_pressed(left_key) {
        active_channel.current = NadiChannel::Ida;
        println!("Active energetic channel switched to Ida (Left Gate)");
    } else if keyboard_input.just_pressed(right_key) {
        active_channel.current = NadiChannel::Pingala;
        println!("Active energetic channel switched to Pingala (Right Gate)");
    }
}
