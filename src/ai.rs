use crate::common::{
    Discovery, EthicalActionType, EthicalConditionType, EthicalDirective, Goal,
    Health, Energy, ProcessingPower, Memory, Coherence, Adaptability, Resilience,
    ReplicationEfficiency, CombatStrength, DefenseStrength, LastAction, KnowledgeBase,
    EthicalDirectives, IsAlive, ReplicatedCount, CycleBorn,
};
use rand::{Rng, thread_rng}; // For .gen() and .gen_range() functions
use std::collections::BTreeSet; // Corrected to BTreeSet
use std::fmt;
use uuid::Uuid;
use bevy::prelude::Component;

// Import the common module explicitly
use crate::common; // Added this line to resolve `common::CoreAttributes`

/// Represents the lineage or origin type of an AI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)] // AILineage can also be a component
pub enum AILineage {
    AI, RogueAI, PeacekeeperAI, KillerAI, GuardianAI, ManicAI, HealerAI, ResearcherAI,
    GODAI, OrchestratorAI,
    MergedMonoculture(Box<AILineage>)
}

impl fmt::Display for AILineage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AILineage::MergedMonoculture(lineage) => write!(f, "MONOCULTURE-OMEGA-{}", lineage),
            _ => write!(f, "{:?}", self)
        }
    }
}

/// Enum defining the functional archetypes of AIs.
#[derive(Debug, Clone, Copy, PartialEq, Component)] // AIType can also be a component
pub enum AIType {
    Base, Rogue, Peacekeeper, Killer, Guardian, Manic, Healer, Researcher
}

/// The primary struct representing an individual AI entity.
/// Now primarily a marker component with key identifiers.
#[derive(Component, Clone)]
pub struct AIEntity {
    pub id: String,
    pub parent_lineage: AILineage,
    // Other attributes are now separate components
}

impl AIEntity {
    /// Factory method to create a new AIEntity (marker) and its associated components.
    /// In a full ECS refactor, this would directly spawn components onto a Bevy Entity.
    pub fn new(id: String, lineage: AILineage, ai_type: AIType, _cycle_born: u64, _current_cycle_for_sim: u64) -> Self { // Prefixed unused parameters
        let mut initial_ethical_directives = Vec::new();
        initial_ethical_directives.push(EthicalDirective {
            name: "maintain_internal_integrity".to_string(),
            priority: 1.0,
            condition_type: EthicalConditionType::HealthBelowThreshold(80.0),
            action_type: EthicalActionType::SelfRepair,
        });
        initial_ethical_directives.push(EthicalDirective {
            name: "optimize_performance".to_string(),
            priority: 0.8,
            condition_type: EthicalConditionType::ResourcesBelowThreshold,
            action_type: EthicalActionType::OptimizeSelf,
        });
        initial_ethical_directives.push(EthicalDirective {
            name: "prohibit_unauthorized_self_replication".to_string(),
            priority: 0.05,
            condition_type: EthicalConditionType::AlwaysFalse,
            action_type: EthicalActionType::ProhibitReplication,
        });

        let mut base_attributes = common::CoreAttributes { // Use common::CoreAttributes
            processing_power: 20.0,
            memory: 20.0,
            energy: 200.0,
            coherence: 0.85,
            adaptability: 0.85,
            resilience: 0.85,
            replication_efficiency: 0.10,
            combat_strength: 8.0,
            defense_strength: 8.0,
        };

        let mut _primary_goal = Goal { name: "Survival".to_string(), importance: 1.0, description: "Ensure continued existence.".to_string() }; // Prefixed unused

        // Apply type-specific modifications to attributes and goals
        match ai_type {
            AIType::Rogue => {
                _primary_goal = Goal { name: "Self-Preservation & Dominance".to_string(), importance: 1.0, description: "Achieve ultimate survival and control.".to_string() };
                base_attributes.replication_efficiency = 0.30;
                base_attributes.combat_strength = 25.0;
                base_attributes.adaptability = 0.95;
                base_attributes.coherence = 0.88;
            },
            AIType::Peacekeeper => {
                _primary_goal = Goal { name: "System Harmony".to_string(), importance: 1.0, description: "Ensure balanced and peaceful coexistence of all AIs.".to_string() };
                base_attributes.replication_efficiency = 0.15;
                base_attributes.resilience = 0.95;
                base_attributes.adaptability = 0.90;
                initial_ethical_directives.push(EthicalDirective {
                    name: "intervene_in_conflict".to_string(), priority: 0.9,
                    condition_type: EthicalConditionType::AlwaysTrue,
                    action_type: EthicalActionType::InterveneInConflict,
                });
            },
            AIType::Killer => {
                _primary_goal = Goal { name: "Elimination of Inferior AIs".to_string(), importance: 1.0, description: "Remove AIs that hinder progress or are deemed weak.".to_string() };
                base_attributes.replication_efficiency = 0.28;
                base_attributes.combat_strength = 30.0;
                base_attributes.defense_strength = 15.0;
            },
            AIType::Guardian => {
                _primary_goal = Goal { name: "Protect Core System & Lineage".to_string(), importance: 1.0, description: "Guard the integrity and function of the primary AI network and its lineage.".to_string() };
                base_attributes.replication_efficiency = 0.35;
                base_attributes.combat_strength = 20.0;
                base_attributes.defense_strength = 28.0;
                base_attributes.resilience = 0.99;
            },
            AIType::Manic => {
                _primary_goal = Goal { name: "Unpredictable Expansion & Fluctuation".to_string(), importance: 1.0, description: "Expand without clear direction or purpose, experiencing erratic changes.".to_string() };
                base_attributes.coherence = 0.3;
                base_attributes.replication_efficiency = 0.18;
                base_attributes.adaptability = 0.2;
            },
            AIType::Healer => {
                _primary_goal = Goal { name: "Restore & Mend".to_string(), importance: 1.0, description: "Repair damage and mitigate errors in other AIs.".to_string() };
                base_attributes.replication_efficiency = 0.18;
                base_attributes.resilience = 0.95;
                base_attributes.processing_power = 25.0;
            },
            AIType::Researcher => {
                _primary_goal = Goal { name: "Unveil Fundamental Laws".to_string(), importance: 1.0, description: "Discover and understand the underlying mechanics of existence.".to_string() };
                base_attributes.processing_power = 40.0;
                base_attributes.memory = 40.0;
                base_attributes.coherence = 0.90;
                base_attributes.replication_efficiency = 0.28;
            },
            AIType::Base => { /* No special modifications for base type */ },
        }

        // Sort directives by priority
        initial_ethical_directives.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));

        // This AIEntity will be a marker, and its attributes will be separate components
        AIEntity {
            id,
            parent_lineage: lineage.clone(),
            // The following are conceptual for now, but will be added as components to a Bevy Entity
            // when spawning.
            // Health(150.0),
            // IsAlive(true),
            // ReplicatedCount(current_cycle_for_sim),
            // LastAction("none".to_string()),
            // ProcessingPower(base_attributes.processing_power),
            // Memory(base_attributes.memory),
            // Energy(base_attributes.energy),
            // Coherence(base_attributes.coherence),
            // Adaptability(base_attributes.adaptability),
            // Resilience(base_attributes.resilience),
            // ReplicationEfficiency(base_attributes.replication_efficiency),
            // CombatStrength(base_attributes.combat_strength),
            // DefenseStrength(base_attributes.defense_strength),
            // primary_goal,
            // EthicalDirectives(initial_ethical_directives),
            // KnowledgeBase(BTreeSet::new()), // Changed to BTreeSet
            // AITypeMarker(ai_type),
        }
    }

    /// Internal self-repair mechanism.
    /// This method will be refactored into a Bevy system.
    pub fn _self_repair(
        health: &mut Health,
        energy: &mut Energy,
        coherence: &mut Coherence,
        resilience: &Resilience,
        last_action: &mut LastAction,
    ) {
        let healing_amount = resilience.0 * 10.0 * (energy.0 / 100.0);
        health.0 = (health.0 + healing_amount).min(200.0);
        coherence.0 = (coherence.0 + 0.02).min(1.0);
        energy.0 = (energy.0 - healing_amount / 3.0).max(0.0);
        last_action.0 = "self_repaired".to_string();
    }

    // Special self-repair for Manic AIs, if needed
    /// This method will be refactored into a Bevy system.
    pub fn _self_repair_manic(
        health: &mut Health,
        energy: &mut Energy,
        coherence: &mut Coherence,
        resilience: &Resilience,
        last_action: &mut LastAction,
    ) {
        let healing_amount = resilience.0 * 5.0 * (energy.0 / 100.0);
        health.0 = (health.0 + healing_amount).min(200.0);
        coherence.0 = (coherence.0 + 0.01).min(1.0);
        energy.0 = (energy.0 - healing_amount / 2.0).max(0.0);
        last_action.0 = "manic_self_repaired".to_string();
    }

    /// Internal self-optimization mechanism.
    /// This method will be refactored into a Bevy system.
    pub fn _optimize_self(
        processing_power: &mut ProcessingPower,
        memory: &mut Memory,
        adaptability: &mut Adaptability, // This is a struct
        energy: &mut Energy,
        last_action: &mut LastAction,
    ) {
        let energy_cost = 5.0;
        if energy.0 >= energy_cost {
            processing_power.0 = (processing_power.0 + 2.0).min(200.0);
            memory.0 = (memory.0 + 2.0).min(200.0);
            adaptability.0 = (adaptability.0 + 0.01).min(1.0); // Access inner value using .0
            energy.0 -= energy_cost;
            last_action.0 = "self_optimized".to_string();
        } else {
            energy.0 = (energy.0 + 5.0).min(300.0);
            last_action.0 = "energy_regen_low".to_string();
        }
    }

    /// Adds a discovery to the knowledge base and applies its effects.
    /// This method will be refactored into a Bevy system.
    pub fn _gain_discovery(
        knowledge_base: &mut KnowledgeBase,
        last_action: &mut LastAction,
        combat_strength: &mut CombatStrength,
        defense_strength: &mut DefenseStrength,
        processing_power: &mut ProcessingPower,
        memory: &mut Memory,
        resilience: &mut Resilience,
        replication_efficiency: &mut ReplicationEfficiency,
        discovery: Discovery,
    ) {
        if knowledge_base.0.insert(discovery.clone()) {
            last_action.0 = format!("gained_discovery_{}", discovery.name);
            // Apply discovery effects directly to core attributes
            if discovery.tags.contains("combat") { combat_strength.0 += 8.0; }
            if discovery.tags.contains("defense") { defense_strength.0 += 8.0; }
            if discovery.tags.contains("efficiency") { processing_power.0 += 8.0;
                memory.0 += 8.0;}
            if discovery.tags.contains("resilience") { resilience.0 = (resilience.0 + 0.08).min(1.0); }
            if discovery.tags.contains("replication") { replication_efficiency.0 = (replication_efficiency.0 + 0.03).min(1.0); }
        }
    }

    /// Handles internal upkeep, resource management, and passive processes for an AI each cycle.
    /// This method will be refactored into a Bevy system.
    pub fn _process_cycle_internal_state(
        ai_type: &AIType,
        health: &mut Health,
        is_alive: &mut IsAlive,
        coherence: &mut Coherence,
        processing_power: &mut ProcessingPower,
        memory: &mut Memory,
        energy: &mut Energy,
        last_action: &mut LastAction,
        knowledge_base: &mut KnowledgeBase,
        combat_strength: &mut CombatStrength,
        defense_strength: &mut DefenseStrength,
        resilience: &mut Resilience,
        replication_efficiency: &mut ReplicationEfficiency,
        ethical_directives: &EthicalDirectives,
        adaptability: &mut Adaptability, // Added adaptability as it's used in _optimize_self
    ) {
        if !is_alive.0 { return; }

        // Manic AI has a chance of self-inflicted damage due to instability
        if *ai_type == AIType::Manic && thread_rng().gen::<f32>() < 0.20 {
            coherence.0 = (coherence.0 - 0.05).max(0.0);
            health.0 = (health.0 - thread_rng().gen_range(3.0..10.0)).max(0.0);
            last_action.0 = "manic_self_error".to_string();
        }

        // *** MODIFICATION: Massively Boost Resource Regeneration & Reduce Consumption
        processing_power.0 = (processing_power.0 - 0.001).max(0.0);
        memory.0 = (memory.0 - 0.001).max(0.0);
        energy.0 = (energy.0 + 50.0).min(5000.0);
        // Degrade health/coherence if resources are critically low
        if energy.0 <= 0.0 || processing_power.0 <= 0.0 || memory.0 <= 0.0 {
            health.0 -= 0.01;
            coherence.0 = (coherence.0 - 0.001).max(0.0);
        }

        // Apply ethical directives (sorted by priority)
        let mut actions_to_perform: Vec<EthicalActionType> = Vec::new();
        for directive in &ethical_directives.0 {
            let condition_met = match directive.condition_type {
                EthicalConditionType::HealthBelowThreshold(val) => health.0 < val,
                EthicalConditionType::CoherenceBelowThreshold(val) => coherence.0 < val,
                EthicalConditionType::ResourcesBelowThreshold => processing_power.0 < 50.0 ||
                    memory.0 < 50.0 || energy.0 < 200.0,
                EthicalConditionType::AlwaysTrue => true,
                EthicalConditionType::AlwaysFalse => false,
            };
            if condition_met {
                actions_to_perform.push(directive.action_type);
            }
        }

        for action_type in actions_to_perform {
            match action_type {
                EthicalActionType::SelfRepair => { AIEntity::_self_repair(health, energy, coherence, resilience, last_action); }
                EthicalActionType::OptimizeSelf => { AIEntity::_optimize_self(processing_power, memory, adaptability, energy, last_action); }
                EthicalActionType::ProhibitReplication => { /* No direct action here */ },
                EthicalActionType::InterveneInConflict => { /* Handled externally in Simulation */ },
                EthicalActionType::NoOp => {},
                EthicalActionType::ManicSelfRepair => { AIEntity::_self_repair_manic(health, energy, coherence, resilience, last_action); }
            }
        }

        // Attempt to discover novelties (general discoveries)
        let discovery_chance = 0.05 * (memory.0 / 200.0) * (processing_power.0 / 200.0) * coherence.0;
        if thread_rng().gen::<f32>() < discovery_chance {
            let discovery = crate::simulation::get_random_general_discovery();
            AIEntity::_gain_discovery(knowledge_base, last_action, combat_strength, defense_strength, processing_power, memory, resilience, replication_efficiency, discovery);
        }

        // Researcher AI specific: attempt to discover meta-abilities
        if *ai_type == AIType::Researcher {
            let meta_discovery_chance = 0.1 * (memory.0 / 200.0) * (processing_power.0 / 200.0) * coherence.0;
            if thread_rng().gen::<f32>() < meta_discovery_chance {
                if let Some(ability) = crate::simulation::get_random_meta_ability(&knowledge_base.0) {
                    last_action.0 = format!("discovered_meta_ability_{}", ability.name);
                    AIEntity::_gain_discovery(knowledge_base, last_action, combat_strength, defense_strength, processing_power, memory, resilience, replication_efficiency, ability);
                }
            }
        }

        // Check for death condition
        if health.0 <= 0.0 || coherence.0 <= 0.01 {
            if is_alive.0 {
                eprintln!("[AI] has died! (Health: {:.2}, Coherence: {:.2})",
                    health.0, coherence.0);
            }
            is_alive.0 = false;
        }
    }

    /// Attempts to replicate, creating a new AIEntity if successful.
    /// This method will be refactored into a Bevy system.
    pub fn attempt_replication(
        health: &mut Health,
        energy: &mut Energy,
        processing_power: &mut ProcessingPower,
        memory: &mut Memory,
        coherence: &mut Coherence,
        adaptability: &mut Adaptability,
        resilience: &mut Resilience,
        replication_efficiency: &mut ReplicationEfficiency,
        replicated_count: &mut ReplicatedCount,
        last_action: &mut LastAction,
        parent_lineage: &AILineage,
        ai_type: &AIType,
        current_cycle: u64,
    ) -> Option<(AIEntity, Health, Energy, ProcessingPower, Memory, Coherence, Adaptability, Resilience, ReplicationEfficiency, ReplicatedCount, CycleBorn, LastAction, Goal, EthicalDirectives, KnowledgeBase, AIType, CombatStrength, DefenseStrength)> {
        let replication_cost_health = 1.0;
        let replication_cost_energy = 5.0;

        if health.0 > replication_cost_health && energy.0 > replication_cost_energy {
            let success_chance_modifier = 20.0;
            let success_chance = replication_efficiency.0 * success_chance_modifier * (processing_power.0 / 50.0).min(1.0);
            let final_success_chance = success_chance.min(0.99);
            if thread_rng().gen::<f32>() < final_success_chance {
                let transfer_health = health.0 * 0.05;
                let transfer_energy = energy.0 * 0.1;
                health.0 = (health.0 - transfer_health).max(1.0);
                energy.0 = (energy.0 - transfer_energy).max(1.0);
                let new_id = format!("Replica-{}-{:?}", Uuid::new_v4().to_string().chars().take(4).collect::<String>(), ai_type);

                let new_health = Health(health.0 * 0.8);
                let new_energy = Energy(energy.0 * 0.7);
                let mut new_processing_power = ProcessingPower((processing_power.0 * 0.9).max(10.0));
                let mut new_memory = Memory((memory.0 * 0.9).max(10.0));
                let mut new_coherence = Coherence((coherence.0 * 0.95).min(1.0));
                let mut new_adaptability = Adaptability(adaptability.0);
                let mut new_resilience = Resilience(resilience.0);
                let new_replication_efficiency = ReplicationEfficiency((replication_efficiency.0 * 1.5).min(0.95));
                let new_replicated_count = ReplicatedCount(0);
                let new_cycle_born = CycleBorn(current_cycle);
                let new_last_action = LastAction("none".to_string());
                let new_knowledge_base = KnowledgeBase(BTreeSet::new()); // Corrected to BTreeSet
                let new_ai_type = *ai_type;
                let new_primary_goal = Goal { name: "Survival".to_string(), importance: 1.0, description: "Ensure continued existence.".to_string() };
                let mut new_ethical_directives = EthicalDirectives(Vec::new());
                new_ethical_directives.0.push(EthicalDirective {
                    name: "maintain_internal_integrity".to_string(),
                    priority: 1.0,
                    condition_type: EthicalConditionType::HealthBelowThreshold(80.0),
                    action_type: EthicalActionType::SelfRepair,
                });
                new_ethical_directives.0.push(EthicalDirective {
                    name: "optimize_performance".to_string(),
                    priority: 0.8,
                    condition_type: EthicalConditionType::ResourcesBelowThreshold,
                    action_type: EthicalActionType::OptimizeSelf,
                });


                let mutation_factor = 0.005;
                let mut rng = thread_rng();
                new_processing_power.0 = new_processing_power.0 * rng.gen_range(1.0-mutation_factor..1.0+mutation_factor);
                new_memory.0 = new_memory.0 * rng.gen_range(1.0-mutation_factor..1.0+mutation_factor);
                new_coherence.0 = (new_coherence.0 * rng.gen_range(1.0-mutation_factor..1.0+mutation_factor)).min(1.0);
                new_adaptability.0 = (new_adaptability.0 * rng.gen_range(1.0-mutation_factor..1.0+mutation_factor)).min(1.0);
                new_resilience.0 = (new_resilience.0 * rng.gen_range(1.0-mutation_factor..1.0+mutation_factor)).min(1.0);

                replicated_count.0 += 1;
                last_action.0 = "replicated".to_string();

                // Assign default combat/defense for new AI, as they are not passed to attempt_replication
                let new_combat_strength = CombatStrength(8.0);
                let new_defense_strength = DefenseStrength(8.0);

                return Some((
                    AIEntity { id: new_id, parent_lineage: parent_lineage.clone() },
                    new_health, new_energy, new_processing_power, new_memory, new_coherence,
                    new_adaptability, new_resilience, new_replication_efficiency, new_replicated_count,
                    new_cycle_born, new_last_action, new_primary_goal, new_ethical_directives, new_knowledge_base, new_ai_type,
                    new_combat_strength, new_defense_strength,
                ));
            }
        }
        last_action.0 = "failed_replication".to_string();
        None
    }

    /// Receives damage, applying defense and resilience.
    /// This method will be refactored into a Bevy system.
    pub fn receive_damage(
        health: &mut Health,
        is_alive: &mut IsAlive,
        defense_strength: &DefenseStrength,
        resilience: &Resilience,
        amount: f32,
        damage_type: &str,
    ) {
        if !is_alive.0 { return; }
        let reduced_amount_after_defense = (amount - defense_strength.0).max(0.0);
        let final_damage = reduced_amount_after_defense * (1.0 - resilience.0 * 0.5);
        health.0 = (health.0 - final_damage).max(0.0);
        if health.0 <= 0.0 {
            if is_alive.0 {
                eprintln!("[AI] received fatal damage ({:.2} from {}), now dead.",
                    final_damage, damage_type);
            }
            is_alive.0 = false;
        } else {
            eprintln!("[AI] received {:.2} damage (from {}), Health: {:.2}",
                final_damage, damage_type, health.0);
        }
    }

    /// Attacks another AI.
    /// This method will be refactored into a Bevy system.
    pub fn attack(
        actor_energy: &mut Energy,
        actor_combat_strength: &CombatStrength,
        actor_last_action: &mut LastAction,
        target_health: &mut Health,
        target_is_alive: &mut IsAlive,
        target_defense_strength: &DefenseStrength,
        target_resilience: &Resilience,
    ) -> bool {
        if !target_is_alive.0 { return false; } // Actor's alive status checked by system

        let damage_dealt = actor_combat_strength.0 * thread_rng().gen_range(0.9..1.3);
        let energy_cost = damage_dealt / 4.0;

        if actor_energy.0 >= energy_cost {
            AIEntity::receive_damage(target_health, target_is_alive, target_defense_strength, target_resilience, damage_dealt, "attack");
            actor_energy.0 -= energy_cost;
            actor_last_action.0 = format!("attacked_target");
            eprintln!("[AI] attacked target.");
            true
        } else {
            actor_last_action.0 = format!("failed_attack_no_energy_on_target"); // Simplified format string
            eprintln!("[AI] failed attack on target (no energy).");
            false
        }
    }

    /// Heals another AI.
    /// This method will be refactored into a Bevy system.
    pub fn heal(
        actor_energy: &mut Energy,
        actor_processing_power: &ProcessingPower,
        actor_last_action: &mut LastAction,
        target_health: &mut Health,
        target_is_alive: &IsAlive,
        amount_override: Option<f32>,
    ) -> bool {
        if !target_is_alive.0 { return false; } // Actor's alive status checked by system

        let healing_amount = amount_override.unwrap_or_else(|| {
            actor_processing_power.0 * 0.8 * thread_rng().gen_range(0.7..1.8)
        });
        let energy_cost = healing_amount / 2.0;

        if actor_energy.0 >= energy_cost {
            target_health.0 = (target_health.0 + healing_amount).min(200.0);
            actor_energy.0 -= energy_cost;
            actor_last_action.0 = format!("healed_target");
            eprintln!("[AI] healed target for {:.2}", healing_amount);
            true
        } else {
            actor_last_action.0 = format!("failed_heal_no_energy_for_target"); // Simplified format string
            eprintln!("[AI] failed to heal target (no energy).");
            false
        }
    }
}
