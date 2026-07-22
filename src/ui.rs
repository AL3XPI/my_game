use bevy::prelude::*;
use crate::components::*;

pub fn setup_hud(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Spawn Right Panel
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
            width: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.05, 0.95)),
        BorderColor::all(Color::srgba(0.0, 0.8, 0.4, 0.4)),
    )).with_children(|parent| {
        // Header
        parent.spawn((
            Text::new("NADI BIO-METRICS"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.0, 1.0, 0.6)),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));

        // Myelin Text
        parent.spawn((
            Text::new("MYELIN INTEGRITY: 100.0%"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            MyelinTextUi,
        ));

        // Myelin Bar Container
        parent.spawn((
            Node {
                width: Val::Px(240.0),
                height: Val::Px(12.0),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 1.0)),
        )).with_children(|bar_parent| {
            // Myelin Fill Bar
            bar_parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.8, 1.0)),
                MyelinFillBarUi,
            ));
        });

        // Silhouette Container
        parent.spawn((
            Node {
                width: Val::Px(140.0),
                height: Val::Px(330.0),
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|sil_parent| {
            // Stylized Body Parts (translucent dark panels)
            // Head
            sil_parent.spawn((
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(50.0), // (140 - 40) / 2 = 50
                    bottom: Val::Px(265.0),
                    border_radius: BorderRadius::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.4, 0.6, 0.15)),
            ));

            // Neck
            sil_parent.spawn((
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(25.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(62.0), // (140 - 16) / 2 = 62
                    bottom: Val::Px(245.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.4, 0.6, 0.15)),
            ));

            // Torso
            sil_parent.spawn((
                Node {
                    width: Val::Px(90.0),
                    height: Val::Px(140.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(25.0), // (140 - 90) / 2 = 25
                    bottom: Val::Px(105.0),
                    border_radius: BorderRadius::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.4, 0.6, 0.15)),
            ));

            // Pelvis/Base
            sil_parent.spawn((
                Node {
                    width: Val::Px(70.0),
                    height: Val::Px(70.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(35.0), // (140 - 70) / 2 = 35
                    bottom: Val::Px(35.0),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.4, 0.6, 0.15)),
            ));

            // Central Vril spinal column (Nadi pathway)
            sil_parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(280.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(55.0), // (140 - 30) / 2 = 55
                    bottom: Val::Px(25.0),
                    border_radius: BorderRadius::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.6)),
            )).with_children(|vril_parent| {
                // Fluid fill meter (emerald / red)
                vril_parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0), // default to 50%
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        border_radius: BorderRadius::all(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 0.8, 0.4)),
                    OjasFillMeterUi,
                ));
            });

            // 7 Chakras mapped vertically
            let chakra_heights = [25.0, 65.0, 115.0, 165.0, 215.0, 255.0, 285.0];
            for i in 0..7 {
                sil_parent.spawn((
                    Node {
                        width: Val::Px(22.0),
                        height: Val::Px(22.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(59.0), // (140 - 22) / 2 = 59
                        bottom: Val::Px(chakra_heights[i]),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(11.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.6)),
                    BorderColor::all(Color::WHITE),
                    ChakraUiMarker { index: i },
                ));
            }

            // Procedurally render Ida & Pingala curving vector paths flanking the spine
            let steps = 25;
            let start_y = 20.0;
            let end_y = 295.0;
            let amplitude = 6.0;
            let freq = 2.0 * std::f32::consts::PI * 2.0;

            for i in 0..steps {
                let t = i as f32 / (steps - 1) as f32;
                let y = start_y + (end_y - start_y) * t;
                let theta = t * freq;
                
                let ida_x = 45.0 + amplitude * theta.sin();
                let pingala_x = 95.0 - amplitude * theta.sin();

                // Spawn Ida node (Left channel)
                sil_parent.spawn((
                    Node {
                        width: Val::Px(5.0),
                        height: Val::Px(5.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(ida_x - 2.5),
                        bottom: Val::Px(y - 2.5),
                        border_radius: BorderRadius::all(Val::Px(2.5)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(30.0 / 255.0, 37.0 / 255.0, 43.0 / 255.0)),
                    IdaNode,
                ));

                // Spawn Pingala node (Right channel)
                sil_parent.spawn((
                    Node {
                        width: Val::Px(5.0),
                        height: Val::Px(5.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(pingala_x - 2.5),
                        bottom: Val::Px(y - 2.5),
                        border_radius: BorderRadius::all(Val::Px(2.5)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(30.0 / 255.0, 37.0 / 255.0, 43.0 / 255.0)),
                    PingalaNode,
                ));
            }

            // Dynamic text labels above top entry coordinates of the flanking strands
            sil_parent.spawn((
                Text::new("[Q]"),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(32.0),
                    bottom: Val::Px(302.0),
                    ..default()
                },
                IdaKeybindUi,
            ));

            sil_parent.spawn((
                Text::new("[E]"),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(92.0),
                    bottom: Val::Px(302.0),
                    ..default()
                },
                PingalaKeybindUi,
            ));
        });

        // Segmented Wheel Canvas Node
        let size = bevy::render::render_resource::Extent3d {
            width: 280,
            height: 280,
            depth_or_array_layers: 1,
        };
        let canvas_data = vec![0; 280 * 280 * 4];
        let canvas_image = Image::new_fill(
            size,
            bevy::render::render_resource::TextureDimension::D2,
            &canvas_data,
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            bevy::asset::RenderAssetUsages::MAIN_WORLD | bevy::asset::RenderAssetUsages::RENDER_WORLD,
        );
        let canvas_handle = images.add(canvas_image);

        parent.spawn((
            ImageNode::new(canvas_handle.clone()),
            Node {
                width: Val::Px(280.0),
                height: Val::Px(280.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            CelestialWheelCanvas {
                image_handle: canvas_handle,
                sun_flash: 0.0,
                sun_flash_sign: None,
                sun_flash_is_resonant: false,
                moon_flash: 0.0,
                moon_flash_sign: None,
                moon_flash_is_resonant: false,
                ascendant_flash: 0.0,
                ascendant_flash_sign: None,
                ascendant_flash_is_resonant: false,
            },
        ));

        // Spacing
        parent.spawn(Node {
            height: Val::Px(15.0),
            ..default()
        });

        // Status Text
        parent.spawn((
            Text::new("STATUS: STABLE"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
            ExhaustionStatusTextUi,
        ));
    });

    // Spawn Horizontal Action Hotbar
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            bottom: Val::Px(15.0),
            left: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
    )).with_children(|parent_container| {
        parent_container.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                align_items: AlignItems::Center,
                ..default()
            },
            Pickable::IGNORE,
        )).with_children(|hotbar| {
            let categories = [
                (SkillCategory::AscendantAligned, "[1]\nTESSELLATION"),
                (SkillCategory::SunAligned, "[2]\nFRACTAL"),
                (SkillCategory::MoonAligned, "[3]\nECLIPSE"),
            ];

            for (category, label) in categories {
                hotbar.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(65.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(6.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        position_type: PositionType::Relative,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.02, 0.02, 0.05, 0.90)),
                    BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                    HotbarCardUi { category },
                )).with_children(|card| {
                    card.spawn((
                        Text::new(label),
                        TextFont { font_size: 11.0, ..default() },
                        TextColor(Color::WHITE),
                        Node {
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                    ));

                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(0.0),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.8, 0.1, 0.1, 0.45)),
                        HotbarFrictionOverlayUi { category },
                    ));

                    card.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 0.75, 0.0, 0.0)),
                        HotbarAmberFlashOverlayUi { category },
                    ));

                    card.spawn((
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Px(4.0),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 0.9, 0.4)),
                        HotbarProgressBarUi { category },
                    ));
                });
            }
        });
    });
// update_ui system scrubbed completely (legacy text block removed)

pub fn update_bio_metrics_hud(
    player_query: Query<(&Ojas, &ChakraTree, &Myelin, &Health), With<Player>>,
    mut fill_meter_query: Query<(&mut Node, &mut BackgroundColor), (With<OjasFillMeterUi>, Without<MyelinFillBarUi>, Without<ChakraUiMarker>, Without<HotbarProgressBarUi>, Without<HotbarFrictionOverlayUi>, Without<HotbarAmberFlashOverlayUi>, Without<IdaNode>, Without<PingalaNode>)>,
    mut myelin_bar_query: Query<&mut Node, (With<MyelinFillBarUi>, Without<OjasFillMeterUi>, Without<ChakraUiMarker>, Without<HotbarProgressBarUi>, Without<HotbarFrictionOverlayUi>)>,
    mut myelin_text_query: Query<&mut Text, (With<MyelinTextUi>, Without<ExhaustionStatusTextUi>, Without<IdaKeybindUi>, Without<PingalaKeybindUi>)>,
    mut status_text_query: Query<&mut Text, (With<ExhaustionStatusTextUi>, Without<MyelinTextUi>, Without<IdaKeybindUi>, Without<PingalaKeybindUi>)>,
    mut chakra_ui_query: Query<(&ChakraUiMarker, &mut BackgroundColor), (Without<OjasFillMeterUi>, Without<MyelinFillBarUi>, Without<IdaNode>, Without<PingalaNode>, Without<HotbarAmberFlashOverlayUi>)>,
) {
    let (ojas, chakra_tree, myelin, health) = player_query.single().unwrap();
    let ojas_pct = (ojas.current / ojas.max * 100.0).clamp(0.0, 100.0);
    let is_exhausted = ojas.current <= ojas.max * 0.15;

    // 1. Update Ojas Fill Meter
    if let Ok((mut node, mut bg_color)) = fill_meter_query.single_mut() {
        node.height = Val::Percent(ojas_pct);
        if is_exhausted {
            bg_color.0 = Color::srgb(0.9, 0.2, 0.1); // Volatile Burning Red/Orange
        } else {
            bg_color.0 = Color::srgb(0.0, 0.8, 0.4); // Vibrant Emerald Green
        }
    }

    // 2. Update Myelin Fill Bar
    if let Ok(mut node) = myelin_bar_query.single_mut() {
        let myelin_pct = (myelin.structural_density / myelin.max_density * 100.0).clamp(0.0, 100.0);
        node.width = Val::Percent(myelin_pct);
    }

    // 3. Update Myelin Text
    if let Ok(mut text) = myelin_text_query.single_mut() {
        text.0 = format!("MYELIN INTEGRITY: {:.1}%", myelin.structural_density);
    }

    // 4. Update Status Text
    if let Ok(mut text) = status_text_query.single_mut() {
        if health.current <= 0.0 || myelin.structural_density <= 0.0 {
            text.0 = "STATUS: SEVERED".to_string();
        } else if is_exhausted {
            text.0 = "STATUS: EXHAUSTION DANGER".to_string();
        } else {
            text.0 = "STATUS: STABLE".to_string();
        }
    }

    // 5. Update Chakras unblocked state colors dynamically
    for (chakra_marker, mut bg_color) in chakra_ui_query.iter_mut() {
        if chakra_marker.index < chakra_tree.chakras.len() {
            let chakra = &chakra_tree.chakras[chakra_marker.index];
            if chakra.unblocked {
                // Vibrant unique colors for each unblocked chakra node (classic ascending ROYGBIV spectrum)
                bg_color.0 = match chakra.chakra_type {
                    ChakraType::Root => Color::srgb(1.0, 0.1, 0.1),       // Red
                    ChakraType::Sacral => Color::srgb(1.0, 0.5, 0.0),     // Orange
                    ChakraType::SolarPlexus => Color::srgb(1.0, 0.9, 0.0), // Yellow
                    ChakraType::Heart => Color::srgb(0.1, 0.9, 0.1),       // Green
                    ChakraType::Throat => Color::srgb(0.1, 0.6, 1.0),      // Blue
                    ChakraType::ThirdEye => Color::srgb(0.29, 0.0, 0.51),  // Indigo
                    ChakraType::Crown => Color::srgb(0.5, 0.0, 1.0),       // Deep Purple / Violet
                };
            } else {
                bg_color.0 = Color::srgba(0.2, 0.2, 0.2, 0.6); // Blocked state (dark translucent gray)
            }
        }
    }
}

pub fn update_nadi_hud(
    player_query: Query<(&Ojas, &NeuralFriction, &CelestialProfile), With<Player>>,
    active_channel: Res<ActiveChannelState>,
    time: Res<Time>,
    mut ida_query: Query<&mut BackgroundColor, (With<IdaNode>, Without<PingalaNode>, Without<OjasFillMeterUi>, Without<MyelinFillBarUi>, Without<ChakraUiMarker>, Without<HotbarAmberFlashOverlayUi>)>,
    mut pingala_query: Query<&mut BackgroundColor, (With<PingalaNode>, Without<IdaNode>, Without<OjasFillMeterUi>, Without<MyelinFillBarUi>, Without<ChakraUiMarker>, Without<HotbarAmberFlashOverlayUi>)>,
    active_cast: Res<ActiveCastState>,
    dev_state: Res<DevModeState>,
    mut hotbar_card_query: Query<(&HotbarCardUi, &mut BorderColor)>,
    mut progress_bar_query: Query<(&HotbarProgressBarUi, &mut Node), (Without<OjasFillMeterUi>, Without<MyelinFillBarUi>, Without<HotbarFrictionOverlayUi>)>,
    mut flash_overlay_query: Query<(&HotbarAmberFlashOverlayUi, &mut BackgroundColor), (Without<OjasFillMeterUi>, Without<ChakraUiMarker>, Without<IdaNode>, Without<PingalaNode>)>,
    mut friction_overlay_query: Query<(&HotbarFrictionOverlayUi, &mut Node), (Without<OjasFillMeterUi>, Without<MyelinFillBarUi>, Without<HotbarProgressBarUi>)>,
    binds: Res<MovementBinds>,
    mut ida_label_query: Query<&mut Text, (With<IdaKeybindUi>, Without<PingalaKeybindUi>, Without<MyelinTextUi>, Without<ExhaustionStatusTextUi>)>,
    mut pingala_label_query: Query<&mut Text, (With<PingalaKeybindUi>, Without<IdaKeybindUi>, Without<MyelinTextUi>, Without<ExhaustionStatusTextUi>)>,
) {
    let (_ojas, friction, _player_profile) = player_query.single().unwrap();
    let dt = time.delta_secs();
    let lerp_speed = 8.0;

    let inactive_color = Color::srgb(30.0 / 255.0, 37.0 / 255.0, 43.0 / 255.0);
    let ida_active_color = Color::srgb(0.0, 225.0 / 255.0, 1.0);
    let pingala_active_color = Color::srgb(1.0, 68.0 / 255.0, 0.0);

    let ida_base_color = if active_channel.current == NadiChannel::Ida {
        ida_active_color
    } else {
        inactive_color
    };

    let pingala_base_color = if active_channel.current == NadiChannel::Pingala {
        pingala_active_color
    } else {
        inactive_color
    };

    let ida_pulse_factor = (active_cast.ida_pulse / 0.25).clamp(0.0, 1.0);
    let pingala_pulse_factor = (active_cast.pingala_pulse / 0.25).clamp(0.0, 1.0);

    let ida_target = Color::srgb(
        ida_base_color.to_srgba().red + (1.0 - ida_base_color.to_srgba().red) * ida_pulse_factor,
        ida_base_color.to_srgba().green + (1.0 - ida_base_color.to_srgba().green) * ida_pulse_factor,
        ida_base_color.to_srgba().blue + (1.0 - ida_base_color.to_srgba().blue) * ida_pulse_factor,
    ).to_srgba();

    let pingala_target = Color::srgb(
        pingala_base_color.to_srgba().red + (1.0 - pingala_base_color.to_srgba().red) * pingala_pulse_factor,
        pingala_base_color.to_srgba().green + (1.0 - pingala_base_color.to_srgba().green) * pingala_pulse_factor,
        pingala_base_color.to_srgba().blue + (1.0 - pingala_base_color.to_srgba().blue) * pingala_pulse_factor,
    ).to_srgba();

    for mut bg_color in ida_query.iter_mut() {
        let current = bg_color.0.to_srgba();
        let r = current.red + (ida_target.red - current.red) * lerp_speed * dt;
        let g = current.green + (ida_target.green - current.green) * lerp_speed * dt;
        let b = current.blue + (ida_target.blue - current.blue) * lerp_speed * dt;
        let a = current.alpha + (ida_target.alpha - current.alpha) * lerp_speed * dt;
        bg_color.0 = Color::srgba(r, g, b, a);
    }

    for mut bg_color in pingala_query.iter_mut() {
        let current = bg_color.0.to_srgba();
        let r = current.red + (pingala_target.red - current.red) * lerp_speed * dt;
        let g = current.green + (pingala_target.green - current.green) * lerp_speed * dt;
        let b = current.blue + (pingala_target.blue - current.blue) * lerp_speed * dt;
        let a = current.alpha + (pingala_target.alpha - current.alpha) * lerp_speed * dt;
        bg_color.0 = Color::srgba(r, g, b, a);
    }

    // 7. Update Action Hotbar Cards visual states
    for (card_ui, mut border_color) in hotbar_card_query.iter_mut() {
        let sign = match card_ui.category {
            SkillCategory::AscendantAligned => dev_state.tessellation_override,
            SkillCategory::SunAligned => dev_state.fractal_override,
            SkillCategory::MoonAligned => dev_state.eclipse_override,
        };
        let flash_val = match card_ui.category {
            SkillCategory::AscendantAligned => active_cast.flash_amber_tessellation,
            SkillCategory::SunAligned => active_cast.flash_amber_fractal,
            SkillCategory::MoonAligned => active_cast.flash_amber_eclipse,
        };
        if flash_val > 0.0 {
            *border_color = BorderColor::all(Color::srgb(1.0, 0.75, 0.0)); // Flash solid amber warning sign
        } else {
            *border_color = BorderColor::all(get_element_color_prelude(sign));
        }
    }

    // 8. Update Progress Bars
    for (pb_ui, mut pb_node) in progress_bar_query.iter_mut() {
        let progress = if active_cast.casting_skill == Some(pb_ui.category) {
            let prep_time = match pb_ui.category {
                SkillCategory::AscendantAligned => 0.1,
                SkillCategory::SunAligned => 0.6,
                SkillCategory::MoonAligned => 1.8,
            };
            (active_cast.timer / prep_time).clamp(0.0, 1.0)
        } else {
            0.0
        };
        pb_node.width = Val::Percent(progress * 100.0);
    }

    // 9. Update Lockout Flash Overlays
    for (flash_ui, mut bg_color) in flash_overlay_query.iter_mut() {
        let flash_val = match flash_ui.category {
            SkillCategory::AscendantAligned => active_cast.flash_amber_tessellation,
            SkillCategory::SunAligned => active_cast.flash_amber_fractal,
            SkillCategory::MoonAligned => active_cast.flash_amber_eclipse,
        };
        let intensity = (flash_val / 0.5).clamp(0.0, 1.0);
        bg_color.0 = Color::srgba(1.0, 0.75, 0.0, intensity * 0.5);
    }

    // 10. Update Friction Overlays
    for (overlay_ui, mut overlay_node) in friction_overlay_query.iter_mut() {
        let ratio = match overlay_ui.category {
            SkillCategory::AscendantAligned => active_cast.lockout_tessellation / 3.0,
            SkillCategory::SunAligned => active_cast.lockout_fractal / 6.0,
            SkillCategory::MoonAligned => active_cast.lockout_eclipse / 12.0,
        };
        overlay_node.height = Val::Percent(ratio.clamp(0.0, 1.0) * 100.0);
    }

    // 11. Update dynamic Nadi keybind labels above flanking strands
    let (left_text, right_text) = if binds.up == KeyCode::KeyW {
        ("[Q]".to_string(), "[E]".to_string())
    } else {
        ("[O]".to_string(), "[[ ]".to_string())
    };

    if let Ok(mut text) = ida_label_query.single_mut() {
        text.0 = left_text;
    }
    if let Ok(mut text) = pingala_label_query.single_mut() {
        text.0 = right_text;
    }
}

fn get_element_color(sign: ZodiacSign) -> [u8; 4] {
    match sign {
        ZodiacSign::Aries | ZodiacSign::Leo | ZodiacSign::Sagittarius => [255, 85, 0, 255],     // Fire (Orange)
        ZodiacSign::Taurus | ZodiacSign::Virgo | ZodiacSign::Capricorn => [0, 255, 102, 255],    // Earth (Green)
        ZodiacSign::Gemini | ZodiacSign::Libra | ZodiacSign::Aquarius => [255, 255, 255, 255],   // Air (White)
        ZodiacSign::Cancer | ZodiacSign::Scorpio | ZodiacSign::Pisces => [0, 136, 255, 255],     // Water (Blue)
    }
}

fn get_element_color_prelude(sign: ZodiacSign) -> Color {
    match sign {
        ZodiacSign::Aries | ZodiacSign::Leo | ZodiacSign::Sagittarius => Color::srgb(1.0, 0.33, 0.0), // Fire
        ZodiacSign::Taurus | ZodiacSign::Virgo | ZodiacSign::Capricorn => Color::srgb(0.0, 1.0, 0.4), // Earth
        ZodiacSign::Gemini | ZodiacSign::Libra | ZodiacSign::Aquarius => Color::srgb(1.0, 1.0, 1.0), // Air
        ZodiacSign::Cancer | ZodiacSign::Scorpio | ZodiacSign::Pisces => Color::srgb(0.0, 0.53, 1.0), // Water
    }
}

fn get_sign_by_index(index: usize) -> ZodiacSign {
    match index {
        0 => ZodiacSign::Aries,
        1 => ZodiacSign::Taurus,
        2 => ZodiacSign::Gemini,
        3 => ZodiacSign::Cancer,
        4 => ZodiacSign::Leo,
        5 => ZodiacSign::Virgo,
        6 => ZodiacSign::Libra,
        7 => ZodiacSign::Scorpio,
        8 => ZodiacSign::Sagittarius,
        9 => ZodiacSign::Capricorn,
        10 => ZodiacSign::Aquarius,
        _ => ZodiacSign::Pisces,
    }
}

pub fn update_celestial_wheel(
    time: Res<Time>,
    mut images: ResMut<Assets<Image>>,
    player_query: Query<&CelestialProfile, With<Player>>,
    new_projectiles: Query<&FractalProjectile, Added<FractalProjectile>>,
    new_strikes: Query<&TessellationStrike, Added<TessellationStrike>>,
    new_fields: Query<&EclipseField, Added<EclipseField>>,
    mut canvas_query: Query<&mut CelestialWheelCanvas>,
) {
    let Ok(player_profile) = player_query.single() else { return; };
    let Ok(mut canvas) = canvas_query.single_mut() else { return; };

    let dt = time.delta_secs();

    // 1. Tick down flash timers
    canvas.sun_flash = (canvas.sun_flash - dt).max(0.0);
    canvas.moon_flash = (canvas.moon_flash - dt).max(0.0);
    canvas.ascendant_flash = (canvas.ascendant_flash - dt).max(0.0);

    // 2. Read new casts to trigger flashes
    for proj in new_projectiles.iter() {
        let is_resonant = proj.native_sign == player_profile.sun_sign;
        canvas.sun_flash = 0.3;
        canvas.sun_flash_sign = Some(proj.native_sign);
        canvas.sun_flash_is_resonant = is_resonant;
    }

    for field in new_fields.iter() {
        let is_resonant = field.native_sign == player_profile.moon_sign;
        canvas.moon_flash = 0.3;
        canvas.moon_flash_sign = Some(field.native_sign);
        canvas.moon_flash_is_resonant = is_resonant;
    }

    for strike in new_strikes.iter() {
        let is_resonant = strike.native_sign == player_profile.ascendant_sign;
        canvas.ascendant_flash = 0.3;
        canvas.ascendant_flash_sign = Some(strike.native_sign);
        canvas.ascendant_flash_is_resonant = is_resonant;
    }

    // 3. Render pixel data
    if let Some(image) = images.get_mut(&canvas.image_handle) {
        if let Some(ref mut data) = image.data {
            let bg_gray = [30, 37, 43, 255]; // #1e252b

            for y in 0..280 {
                for x in 0..280 {
                    let idx = (y * 280 + x) * 4;
                    let dx = x as f32 - 140.0;
                    let dy = y as f32 - 140.0;
                    let d = (dx * dx + dy * dy).sqrt();

                    // Determine which ring the pixel is in
                    let (ring_category, active_sign, flash_val, flash_sign, flash_is_resonant) = {
                        if d >= 70.0 && d <= 80.0 {
                            (Some(SkillCategory::SunAligned), player_profile.sun_sign, canvas.sun_flash, canvas.sun_flash_sign, canvas.sun_flash_is_resonant)
                        } else if d >= 50.0 && d <= 60.0 {
                            (Some(SkillCategory::MoonAligned), player_profile.moon_sign, canvas.moon_flash, canvas.moon_flash_sign, canvas.moon_flash_is_resonant)
                        } else if d >= 30.0 && d <= 40.0 {
                            (Some(SkillCategory::AscendantAligned), player_profile.ascendant_sign, canvas.ascendant_flash, canvas.ascendant_flash_sign, canvas.ascendant_flash_is_resonant)
                        } else {
                            (None, ZodiacSign::Aries, 0.0, None, false)
                        }
                    };

                    if let Some(_cat) = ring_category {
                        // Clockwise angle mapping with Aries at the top (top vertical axis is 0 deg)
                        let mut angle = dx.atan2(-dy).to_degrees();
                        if angle < 0.0 {
                            angle += 360.0;
                        }

                        // 2-degree visual gap separation check (1 degree on each 30-degree boundary)
                        let offset = angle % 30.0;
                        if offset < 1.0 || offset > 29.0 {
                            data[idx] = 0;
                            data[idx + 1] = 0;
                            data[idx + 2] = 0;
                            data[idx + 3] = 0;
                            continue;
                        }

                        let sign_index = (angle / 30.0) as usize;
                        let sign = get_sign_by_index(sign_index);
                        let index = sign as usize;
                        let is_active_sign = index == active_sign as usize;

                        // Compute final color
                        let base_color = if is_active_sign {
                            get_element_color(sign)
                        } else {
                            bg_gray
                        };

                        let mut final_color = base_color;

                        // Apply dynamic flashes
                        if flash_val > 0.0 && flash_sign.map(|s| s as usize) == Some(index) {
                            let intensity = (flash_val / 0.3).clamp(0.0, 1.0);
                            if flash_is_resonant {
                                // Bright flare: blend base color with white
                                let r = (base_color[0] as f32 + (255.0 - base_color[0] as f32) * intensity) as u8;
                                let g = (base_color[1] as f32 + (255.0 - base_color[1] as f32) * intensity) as u8;
                                let b = (base_color[2] as f32 + (255.0 - base_color[2] as f32) * intensity) as u8;
                                final_color = [r, g, b, 255];
                            } else {
                                // Dim flash for non-resonant foreign castings using 30% of the skill's own element color
                                let skill_element_color = get_element_color(flash_sign.unwrap());
                                let target_r = (skill_element_color[0] as f32 * 0.3) as u8;
                                let target_g = (skill_element_color[1] as f32 * 0.3) as u8;
                                let target_b = (skill_element_color[2] as f32 * 0.3) as u8;

                                let r = (base_color[0] as f32 + (target_r as f32 - base_color[0] as f32) * intensity) as u8;
                                let g = (base_color[1] as f32 + (target_g as f32 - base_color[1] as f32) * intensity) as u8;
                                let b = (base_color[2] as f32 + (target_b as f32 - base_color[2] as f32) * intensity) as u8;
                                final_color = [r, g, b, 255];
                            }
                        }

                        data[idx] = final_color[0];
                        data[idx + 1] = final_color[1];
                        data[idx + 2] = final_color[2];
                        data[idx + 3] = final_color[3];
                    } else {
                        data[idx] = 0;
                        data[idx + 1] = 0;
                        data[idx + 2] = 0;
                        data[idx + 3] = 0;
                    }
                }
            }
        }
    }
}
