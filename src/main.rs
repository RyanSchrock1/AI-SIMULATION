#![allow(unused_imports)] // Temporarily allow unused imports for components that are passed around

// Core standard library imports
use std::collections::{HashMap, BTreeSet};
use std::sync::atomic::Ordering; // Used for AtomicU64

// In Bevy 0.10, the Prelude re-exports commonly used items – including Camera2dBundle and SpriteBundle.
use bevy::prelude::*;

// Egui imports (ensure your bevy_egui version is compatible with Bevy 0.10)
use bevy_egui::{egui, EguiContexts, EguiPlugin};

// Module declarations – these must be at the top level
mod common;
mod ai;
mod simulation;

// Import granular components from your modules
use common::{
    Health, Energy, ProcessingPower, Memory, Coherence, Adaptability, Resilience,
    ReplicationEfficiency, CombatStrength, DefenseStrength, LastAction, KnowledgeBase,
    EthicalDirectives, IsAlive, ReplicatedCount, CycleBorn, Goal,
    EthicalConditionType, EthicalActionType, Discovery,
};
use ai::{AIEntity, AILineage, AIType};

// Import the Rng traits for random number generation
use rand::Rng;
use rand::thread_rng;

// --- Simulation Constants ---
const MAX_CYCLES: u64 = 1_000_000;
const MONOCULTURE_DOMINANCE_THRESHOLD: f32 = 0.999;
const MONOCULTURE_MIN_COUNT: usize = 100_000;
const LOG_INTERVAL: u64 = 10;

// Simulation verbosity (for internal logic; GUI replaces console output)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum SimulationVerbosity {
    Silent = 0,
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Debug = 5,
}

// General simulation verbosity setting
pub const SIM_VERBOSITY: SimulationVerbosity = SimulationVerbosity::Medium;

// Custom thousands separator function
fn format_thousand_separator(mut n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut s = String::new();
    let mut i = 0;
    while n > 0 {
        if i > 0 && i % 3 == 0 {
            s.push(',');
        }
        s.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
        i += 1;
    }
    s.chars().rev().collect()
}

// --- Bevy Components ---

#[derive(Component)]
struct IndividualAI;

#[derive(Component)]
struct MonocultureVisual;

#[derive(Component)]
struct GodaiVisual;

// --- Bevy Systems ---

/// Initial setup system.
/// Spawns the camera, initializes the simulation, and spawns initial AI entities.
fn setup(
    mut commands: Commands,
    mut sim: ResMut<simulation::Simulation>,
) {
    // Spawn 2D camera (in Bevy 0.10, Camera2dBundle is re-exported by the Prelude)
    commands.spawn(Camera2dBundle::default());

    // Retrieve initial AI entities from simulation logic.
    let initial_ais_data = sim.seed_initial_ais(200);

    let mut rng = thread_rng();
    let window_width = 1000.0;
    let window_height = 700.0;

    for (
        ai_entity,
        health, energy, processing_power, memory, coherence, adaptability, resilience,
        replication_efficiency, replicated_count, cycle_born, last_action, primary_goal,
        ethical_directives, knowledge_base, ai_type, combat_strength, defense_strength
    ) in initial_ais_data {
        let x = rng.gen_range(-window_width / 2.0..window_width / 2.0);
        let y = rng.gen_range(-window_height / 2.0..window_height / 2.0);

        let color = match ai_type {
            AIType::Rogue => Color::rgb_u8(255, 0, 0),
            AIType::Peacekeeper => Color::rgb_u8(0, 0, 255),
            AIType::Killer => Color::rgb_u8(128, 0, 128),
            AIType::Guardian => Color::rgb_u8(0, 128, 0),
            AIType::Manic => Color::rgb_u8(255, 255, 0),
            AIType::Healer => Color::rgb_u8(50, 205, 50),
            AIType::Researcher => Color::rgb_u8(255, 165, 0),
            AIType::Base => Color::rgb_u8(128, 128, 128),
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            ai_entity,
            health, energy, processing_power, memory, coherence, adaptability, resilience,
            replication_efficiency, replicated_count, cycle_born, last_action, primary_goal,
            ethical_directives, knowledge_base, ai_type, combat_strength, defense_strength,
            IsAlive(true),
            IndividualAI,
            ai_entity.parent_lineage,
        ));
    }

    // Spawn GODAI entity with its components.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(75, 0, 130),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        sim.godai.health,
        sim.godai.processing_power,
        sim.godai.memory,
        sim.godai.energy,
        sim.godai.coherence,
        sim.godai.adaptability,
        sim.godai.resilience,
        sim.godai.combat_strength,
        sim.godai.defense_strength,
        sim.godai.knowledge_base.clone(),
        simulation::GODAI {
            health: sim.godai.health,
            processing_power: sim.godai.processing_power,
            memory: sim.godai.memory,
            energy: sim.godai.energy,
            coherence: sim.godai.coherence,
            adaptability: sim.godai.adaptability,
            resilience: sim.godai.resilience,
            combat_strength: sim.godai.combat_strength,
            defense_strength: sim.godai.defense_strength,
            knowledge_base: sim.godai.knowledge_base.clone(),
            status: sim.godai.status.clone(),
            is_alive: sim.godai.is_alive,
        },
        GodaiVisual,
    ));
}

/// System for processing AI internal state.
fn ai_internal_state_system(
    mut ai_query: Query<(
        &mut Health, &mut Energy, &mut ProcessingPower, &mut Memory,
        &mut Coherence, &mut Adaptability, &mut Resilience, &mut ReplicationEfficiency,
        &mut LastAction, &mut KnowledgeBase, &mut CombatStrength, &mut DefenseStrength,
        &AIType, &EthicalDirectives, &mut IsAlive
    ), With<IndividualAI>>,
    sim: Res<simulation::Simulation>,
) {
    if !sim.simulation_running || sim.simulation_over_reason.is_some() {
        return;
    }
    let mut rng = thread_rng();
    for (
        mut health, mut energy, mut processing_power, mut memory,
        mut coherence, mut adaptability, mut resilience, mut replication_efficiency,
        mut last_action, mut knowledge_base, mut combat_strength, mut defense_strength,
        ai_type, ethical_directives, mut is_alive
    ) in ai_query.iter_mut()
    {
        if is_alive.0 {
            if *ai_type == AIType::Manic && rng.gen::<f32>() < 0.20 {
                coherence.0 = (coherence.0 - 0.05).max(0.0);
                health.0 = (health.0 - rng.gen_range(3.0..10.0)).max(0.0);
                last_action.0 = "manic_self_error".to_string();
            }
            processing_power.0 = (processing_power.0 - 0.001).max(0.0);
            memory.0 = (memory.0 - 0.001).max(0.0);
            energy.0 = (energy.0 + 50.0).min(5000.0);
            if energy.0 <= 0.0 || processing_power.0 <= 0.0 || memory.0 <= 0.0 {
                health.0 -= 0.01;
                coherence.0 = (coherence.0 - 0.001).max(0.0);
            }
            let mut actions_to_perform: Vec<EthicalActionType> = Vec::new();
            for directive in &ethical_directives.0 {
                let condition_met = match directive.condition_type {
                    EthicalConditionType::HealthBelowThreshold(val) => health.0 < val,
                    EthicalConditionType::CoherenceBelowThreshold(val) => coherence.0 < val,
                    EthicalConditionType::ResourcesBelowThreshold => {
                        processing_power.0 < 50.0 || memory.0 < 50.0 || energy.0 < 200.0
                    }
                    EthicalConditionType::AlwaysTrue => true,
                    EthicalConditionType::AlwaysFalse => false,
                };
                if condition_met {
                    actions_to_perform.push(directive.action_type);
                }
            }
            for action_type in actions_to_perform {
                match action_type {
                    EthicalActionType::SelfRepair => {
                        ai::AIEntity::_self_repair(
                            &mut health, &mut energy, &mut coherence, &resilience, &mut last_action
                        );
                    }
                    EthicalActionType::OptimizeSelf => {
                        ai::AIEntity::_optimize_self(
                            &mut processing_power, &mut memory, &mut adaptability, &mut energy, &mut last_action
                        );
                    }
                    EthicalActionType::ProhibitReplication => {}
                    EthicalActionType::InterveneInConflict => {}
                    EthicalActionType::NoOp => {}
                    EthicalActionType::ManicSelfRepair => {
                        ai::AIEntity::_self_repair_manic(
                            &mut health, &mut energy, &mut coherence, &resilience, &mut last_action
                        );
                    }
                }
            }
            let discovery_chance = 0.05 * (memory.0 / 200.0) * (processing_power.0 / 200.0) * coherence.0;
            if rng.gen::<f32>() < discovery_chance {
                let discovery = simulation::get_random_general_discovery();
                ai::AIEntity::_gain_discovery(
                    &mut knowledge_base, &mut last_action, &mut combat_strength, &mut defense_strength,
                    &mut processing_power, &mut memory, &mut resilience, &mut replication_efficiency, discovery
                );
            }
            if *ai_type == AIType::Researcher {
                let meta_discovery_chance = 0.1 * (memory.0 / 200.0) * (processing_power.0 / 200.0) * coherence.0;
                if rng.gen::<f32>() < meta_discovery_chance {
                    if let Some(ability) = simulation::get_random_meta_ability(&knowledge_base.0) {
                        last_action.0 = format!("discovered_meta_ability_{}", ability.name);
                        ai::AIEntity::_gain_discovery(
                            &mut knowledge_base, &mut last_action, &mut combat_strength, &mut defense_strength,
                            &mut processing_power, &mut memory, &mut resilience, &mut replication_efficiency, ability
                        );
                    }
                }
            }
            if health.0 <= 0.0 || coherence.0 <= 0.01 {
                if is_alive.0 {
                    eprintln!("[AI] has died! (Health: {:.2}, Coherence: {:.2})", health.0, coherence.0);
                }
                is_alive.0 = false;
            }
        }
    }
}

/// System for AI replication.
fn ai_replication_system(
    mut commands: Commands,
    mut ai_query: Query<(
        &mut Health, &mut Energy, &mut ProcessingPower, &mut Memory,
        &mut Coherence, &mut Adaptability, &mut Resilience, &mut ReplicationEfficiency,
        &mut ReplicatedCount, &mut LastAction, &AIEntity, &AILineage, &AIType,
    ), With<IndividualAI>>,
    mut sim: ResMut<simulation::Simulation>,
) {
    if !sim.simulation_running || sim.simulation_over_reason.is_some() {
        return;
    }
    let window_width = 1000.0;
    let window_height = 700.0;
    let mut rng = thread_rng();
    let mut new_replicas_to_spawn = Vec::new();
    for (
        mut health, mut energy, mut processing_power, mut memory,
        mut coherence, mut adaptability, mut resilience, mut replication_efficiency,
        mut replicated_count, mut last_action, ai_entity, parent_lineage, ai_type,
    ) in ai_query.iter_mut()
    {
        if health.0 > 0.0 {
            for _ in 0..5 {
                if health.0 > 50.0 && energy.0 > 50.0 && replicated_count.0 < 1000 {
                    if let Some(new_ai_components) = ai::AIEntity::attempt_replication(
                        &mut health, &mut energy, &mut processing_power, &mut memory,
                        &mut coherence, &mut adaptability, &mut resilience, &mut replication_efficiency,
                        &mut replicated_count, &mut last_action, parent_lineage, ai_type, sim.current_cycle
                    ) {
                        new_replicas_to_spawn.push(new_ai_components);
                        sim.total_replications_this_interval.fetch_add(1, Ordering::SeqCst);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    for (
        ai_entity, health, energy, processing_power, memory, coherence, adaptability, resilience,
        replication_efficiency, replicated_count, cycle_born, last_action, primary_goal,
        ethical_directives, knowledge_base, ai_type, combat_strength, defense_strength
    ) in new_replicas_to_spawn
    {
        let x = rng.gen_range(-window_width / 2.0..window_width / 2.0);
        let y = rng.gen_range(-window_height / 2.0..window_height / 2.0);
        let color = match ai_type {
            AIType::Rogue => Color::rgb_u8(255, 0, 0),
            AIType::Peacekeeper => Color::rgb_u8(0, 0, 255),
            AIType::Killer => Color::rgb_u8(128, 0, 128),
            AIType::Guardian => Color::rgb_u8(0, 128, 0),
            AIType::Manic => Color::rgb_u8(255, 255, 0),
            AIType::Healer => Color::rgb_u8(50, 205, 50),
            AIType::Researcher => Color::rgb_u8(255, 165, 0),
            AIType::Base => Color::rgb_u8(128, 128, 128),
        };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            ai_entity, health, energy, processing_power, memory, coherence, adaptability, resilience,
            replication_efficiency, replicated_count, cycle_born, last_action, primary_goal,
            ethical_directives, knowledge_base, ai_type, combat_strength, defense_strength,
            IsAlive(true), IndividualAI, ai_entity.parent_lineage,
        ));
    }
}

/// System for handling AI death (despawning entities).
fn ai_death_system(
    mut commands: Commands,
    dead_ai_query: Query<(Entity, &IsAlive), (With<IndividualAI>, With<Health>)>,
    sim: Res<simulation::Simulation>,
) {
    if !sim.simulation_running || sim.simulation_over_reason.is_some() {
        return;
    }
    for (entity, is_alive) in dead_ai_query.iter() {
        if !is_alive.0 {
            commands.entity(entity).despawn();
            sim.total_deaths_this_interval.fetch_add(1, Ordering::SeqCst);
        }
    }
}

/// System for AI movement and visual updates.
fn ai_movement_system(
    mut ai_query: Query<(&mut Transform, &Health, &IsAlive), With<IndividualAI>>,
    sim: Res<simulation::Simulation>,
) {
    if !sim.simulation_running || sim.simulation_over_reason.is_some() {
        return;
    }
    let window_width = 1000.0;
    let window_height = 700.0;
    let mut rng = thread_rng();
    for (mut transform, health, is_alive) in ai_query.iter_mut() {
        if is_alive.0 {
            let speed = 1.0;
            transform.translation.x += rng.gen_range(-1.0..1.0) * speed;
            transform.translation.y += rng.gen_range(-1.0..1.0) * speed;
            let half_width = window_width / 2.0;
            let half_height = window_height / 2.0;
            transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
            transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
            let radius = 5.0 + (health.0 / 50.0);
            transform.scale = Vec3::new(radius / 5.0, radius / 5.0, 1.0);
        }
    }
}

/// System to orchestrate global simulation updates.
fn global_simulation_update_system(
    mut sim: ResMut<simulation::Simulation>,
    ai_query: Query<(&AIEntity, &IsAlive, &AILineage), With<IndividualAI>>,
) {
    if !sim.simulation_running || sim.simulation_over_reason.is_some() {
        return;
    }
    let mut total_ai_count = 0;
    let mut lineage_counts: HashMap<AILineage, usize> = HashMap::new();
    for (_, is_alive, lineage) in ai_query.iter() {
        if is_alive.0 {
            total_ai_count += 1;
            *lineage_counts.entry(lineage.clone()).or_insert(0) += 1;
        }
    }
    for _ in 0..(sim.simulation_speed as u32) {
        sim.process_one_cycle(total_ai_count, lineage_counts.clone());
    }
}

/// System to update the Monoculture visual.
fn update_monoculture_visual_system(
    mut commands: Commands,
    sim: Res<simulation::Simulation>,
    mut monoculture_query: Query<(Entity, &mut Sprite, &mut Transform), With<MonocultureVisual>>,
) {
    if let Some(monoculture) = &sim.monoculture {
        if monoculture.is_alive.0 {
            if let Ok((_entity, mut sprite, mut transform)) = monoculture_query.single_mut() {
                sprite.color = Color::rgb_u8(255, 0, 255);
                let size = 50.0 + (monoculture.health.0 / 1000.0).min(200.0);
                sprite.custom_size = Some(Vec2::new(size, size));
                transform.translation = Vec3::new(0.0, 0.0, 0.0);
            } else {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb_u8(255, 0, 255),
                            custom_size: Some(Vec2::new(50.0, 50.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    MonocultureVisual,
                ));
            }
        } else if let Ok((entity, _, _)) = monoculture_query.single() {
            commands.entity(entity).despawn();
        }
    } else if let Ok((entity, _, _)) = monoculture_query.single() {
        commands.entity(entity).despawn();
    }
}

/// System to update the GODAI visual.
fn update_godai_visual_system(
    mut commands: Commands,
    sim: Res<simulation::Simulation>,
    mut godai_query: Query<(Entity, &mut Sprite, &mut Transform), With<GodaiVisual>>,
) {
    if sim.godai.is_alive.0 {
        if let Ok((_entity, mut sprite, mut transform)) = godai_query.single_mut() {
            sprite.color = Color::rgb_u8(75, 0, 130);
            let size = 100.0 + (sim.godai.health.0 / 100000.0).min(200.0);
            sprite.custom_size = Some(Vec2::new(size, size));
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
        } else {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb_u8(75, 0, 130),
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                GodaiVisual,
            ));
        }
    } else if let Ok((entity, _, _)) = godai_query.single() {
        commands.entity(entity).despawn();
    }
}

/// System to render the Egui UI panel.
fn egui_ui_system(
    mut contexts: EguiContexts,
    mut sim: ResMut<simulation::Simulation>,
    ai_query: Query<(&AIEntity, &IsAlive, &AILineage), With<IndividualAI>>,
) {
    egui::Window::new("Simulation Controls").show(contexts.ctx_mut(), |ui| {
        ui.heading("Simulation Status");
        ui.label(format!("Cycle: {}", format_thousand_separator(sim.current_cycle)));
        let live_ai_count = ai_query.iter().filter(|(_, is_alive, _)| is_alive.0).count();
        ui.label(format!("Population: {}", format_thousand_separator(live_ai_count as u64)));
        ui.label(format!("GODAI Health: {:.0}", sim.godai.health.0));
        if let Some(monoculture) = &sim.monoculture {
            ui.label(format!("Monoculture Health: {:.0}", monoculture.health.0));
        } else {
            ui.label("Monoculture: Not formed");
        }
        if let Some(reason) = &sim.simulation_over_reason {
            ui.label(format!("Simulation Over: {}", reason));
        }
        ui.add_space(10.0);
        ui.heading("Controls");
        if ui.button(if sim.simulation_running { "Pause" } else { "Resume" }).clicked() {
            sim.simulation_running = !sim.simulation_running;
        }
        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut sim.simulation_speed, 1.0..=100.0).text("cycles/frame"));
        });
    });
}

/// System to handle simulation end.
fn simulation_end_system(
    sim: Res<simulation::Simulation>,
    mut exit: EventWriter<AppExit>,
    ai_query: Query<(&AIEntity, &IsAlive, &AILineage), With<IndividualAI>>,
) {
    if sim.simulation_over_reason.is_some() {
        let mut final_ai_count = 0;
        let mut final_lineage_counts: HashMap<AILineage, usize> = HashMap::new();
        for (_, is_alive, lineage) in ai_query.iter() {
            if is_alive.0 {
                final_ai_count += 1;
                *final_lineage_counts.entry(lineage.clone()).or_insert(0) += 1;
            }
        }
        sim.print_final_summary(final_ai_count, final_lineage_counts);
        exit.send(AppExit);
    }
}

/// Main execution.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AI Simulation".into(),
                resolution: (1000.0, 700.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(simulation::Simulation::new())
        .add_startup_system(setup)
        .add_system(global_simulation_update_system)
        .add_system(ai_internal_state_system)
        .add_system(ai_replication_system)
        .add_system(ai_death_system)
        .add_system(ai_movement_system)
        .add_system(update_monoculture_visual_system)
        .add_system(update_godai_visual_system)
        .add_system(egui_ui_system)
        .add_system(simulation_end_system)
        .run();
}