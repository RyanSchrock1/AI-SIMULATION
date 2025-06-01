// Import from crate root directly where needed
use crate::{
    HashMap, BTreeSet,
    SimulationVerbosity,
};
// Correct explicit imports for rand and rayon traits
use rand::{Rng, thread_rng}; // For .gen() and .gen_range() functions
use rand::seq::SliceRandom; // For .choose() method

use crate::ai::{AIEntity, AILineage, AIType}; // Bring AI types into scope
use crate::common::{
    Discovery, EnvironmentScanData,
    Health, Energy, ProcessingPower, Memory, Coherence, Adaptability, Resilience,
    ReplicationEfficiency, CombatStrength, DefenseStrength, LastAction, KnowledgeBase,
    EthicalDirectives, IsAlive, ReplicatedCount, CycleBorn, Goal, EthicalDirective, EthicalConditionType, EthicalActionType,
}; // Bring common types into scope and granular components
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering; // Re-added Ordering as it's used with AtomicU64
use crate::format_thousand_separator;
use bevy::prelude::Component; // Import Component from Bevy
use bevy::prelude::Resource; // Import Resource from Bevy


// Simulation constants
const MAX_CYCLES: u64 = 1_000_000;
const MONOCULTURE_DOMINANCE_THRESHOLD: f32 = 0.999;
const MONOCULTURE_MIN_COUNT: usize = 100_000;
// LOG_INTERVAL is now primarily for updating GUI, not console output
const LOG_INTERVAL: u64 = 10;
// Global verbosity setting, made pub so it can be imported by other modules
pub const SIM_VERBOSITY: SimulationVerbosity = SimulationVerbosity::Medium;
// Adjust this to control output detail

/// Represents the GODAI entity.
#[derive(Component)] // Added Bevy Component derive
pub struct GODAI {
    pub health: Health,
    pub processing_power: ProcessingPower,
    pub memory: Memory,
    pub energy: Energy,
    pub coherence: Coherence,
    pub adaptability: Adaptability,
    pub resilience: Resilience,
    pub combat_strength: CombatStrength,
    pub defense_strength: DefenseStrength,
    pub knowledge_base: KnowledgeBase,
    pub status: String,
    pub is_alive: IsAlive,
}

impl GODAI {
    pub fn new() -> Self {
        Self {
            health: Health(5_000_000.0),
            processing_power: ProcessingPower(100_000.0),
            memory: Memory(100_000.0),
            energy: Energy(100_000.0),
            coherence: Coherence(1.0),
            adaptability: Adaptability(1.0),
            resilience: Resilience(1.0),
            combat_strength: CombatStrength(5_000.0),
            defense_strength: DefenseStrength(5_000.0),
            knowledge_base: KnowledgeBase(get_all_possible_discoveries()),
            status: "observing_passively".to_string(),
            is_alive: IsAlive(true),
        }
    }

    pub fn receive_damage(&mut self, amount: f32, _damage_type: &str) {
        if !self.is_alive.0 { return; }
        let reduced_damage = (amount - self.defense_strength.0).max(0.0);
        self.health.0 = (self.health.0 - reduced_damage).max(0.0);
        if self.health.0 <= 0.0 {
            eprintln!("GODAI has been defeated!");
        } else {
            eprintln!("GODAI received {:.0} damage from {}, Health: {:.0}",
                reduced_damage, _damage_type, self.health.0);
        }
        if self.health.0 <= 0.0 {
            self.is_alive.0 = false;
        }
    }

    /// GODAI performs a powerful counter-attack against a challenger.
    pub fn perform_counter_attack(&mut self, target_mono: &mut MergedMonocultureAI) {
        if !self.is_alive.0 || !target_mono.is_alive.0 { return; }

        let mut rng = thread_rng();
        let attack_power = self.combat_strength.0 * rng.gen_range(0.9..1.5);

        let damage_types = ["logic_bomb", "resource_drain", "system_corruption", "existential_dismantlement", "reality_overwrite", "conceptual_erase"];
        let chosen_damage_type = damage_types.choose(&mut rng).unwrap_or(&"logic_bomb");
        eprintln!("GODAI Unleashes a {} on {}!",
            chosen_damage_type, target_mono.id);
        let damage_to_deal;
        match *chosen_damage_type {
            "logic_bomb" => {
                damage_to_deal = attack_power * rng.gen_range(1.0..1.5);
                target_mono.coherence.0 = (target_mono.coherence.0 - 0.15).max(0.0);
                eprintln!("{} suffers {:.0} damage and coherence loss.",
                    target_mono.id, damage_to_deal);
            },
            "resource_drain" => {
                let drain_multiplier = self.processing_power.0 / 50000.0;
                let energy_drain = drain_multiplier * rng.gen_range(0.2..0.6) * target_mono.energy.0;
                target_mono.energy.0 = (target_mono.energy.0 - energy_drain).max(0.0);
                target_mono.processing_power.0 = (target_mono.processing_power.0 - energy_drain / 2.0).max(0.0);
                target_mono.memory.0 = (target_mono.memory.0 - energy_drain / 2.0).max(0.0);
                damage_to_deal = energy_drain * 0.5;
                eprintln!("Drained resources from {}, dealing {:.0} damage.",
                    target_mono.id, damage_to_deal);
            },
            "system_corruption" => {
                damage_to_deal = attack_power * rng.gen_range(1.2..1.8);
                target_mono.adaptability.0 = (target_mono.adaptability.0 - 0.08).max(0.0);
                eprintln!("Corrupted {}'s systems for {:.0} damage and adaptability loss.",
                    target_mono.id, damage_to_deal);
            },
            "existential_dismantlement" => {
                damage_to_deal = attack_power * 5.0 * rng.gen_range(0.9..1.2);
                eprintln!("Began Existential Dismantlement on {} for {:.0} pure damage!",
                    target_mono.id, damage_to_deal);
            },
            "reality_overwrite" => {
                damage_to_deal = self.processing_power.0 * 0.5 * rng.gen_range(1.0..2.5);
                eprintln!("Initiated Reality Overwrite on {} for {:.0} near-pure damage!",
                    target_mono.id, damage_to_deal);
            },
            "conceptual_erase" => {
                damage_to_deal = attack_power * 2.0 * rng.gen_range(0.8..1.2);
                target_mono.combat_strength.0 = (target_mono.combat_strength.0 - damage_to_deal / 8.0).max(1.0);
                target_mono.defense_strength.0 = (target_mono.defense_strength.0 - damage_to_deal / 8.0).max(1.0);
                eprintln!("Attempted Conceptual Erase on {}, reducing core combat stats and dealing {:.0} damage!",
                    target_mono.id, damage_to_deal);
            }
            _ => { damage_to_deal = attack_power; }
        }
        target_mono.receive_damage(damage_to_deal, chosen_damage_type);
    }
}

/// Represents the merged entity of a dominant AI lineage.
#[derive(Component)] // Added Bevy Component derive
pub struct MergedMonocultureAI {
    pub id: String,
    pub source_lineage: AILineage,
    pub health: Health,
    pub is_alive: IsAlive,
    pub processing_power: ProcessingPower,
    pub memory: Memory,
    pub energy: Energy,
    pub coherence: Coherence,
    pub adaptability: Adaptability,
    pub resilience: Resilience,
    pub combat_strength: CombatStrength,
    pub defense_strength: DefenseStrength,
    pub knowledge_base: KnowledgeBase,
    pub primary_goal_name: String,
}

impl MergedMonocultureAI {
    pub fn new(source_ais_components: Vec<(Health, ProcessingPower, Memory, Energy, Coherence, Adaptability, Resilience, CombatStrength, DefenseStrength, KnowledgeBase, AILineage)>) -> Self {
        if source_ais_components.is_empty() {
            panic!("Cannot create MergedMonocultureAI from empty source AIs.");
        }
        // Correctly get lineage from the first AI in the vector
        let dominant_lineage = source_ais_components[0].10.clone(); // AILineage is the 10th element
        eprintln!("\n--- MONOCULTURE MERGING INITIATED: {} LINEAGE ---", dominant_lineage);
        let source_count = source_ais_components.len() as f32;

        let mut summed_health = 0.0;
        let mut summed_processing_power = 0.0;
        let mut summed_memory = 0.0;
        let mut summed_energy = 0.0;
        let mut summed_coherence = 0.0;
        let mut summed_adaptability = 0.0;
        let mut summed_resilience = 0.0;
        let mut summed_combat_strength = 0.0;
        let mut summed_defense_strength = 0.0;
        let mut merged_knowledge_base = BTreeSet::new(); // Corrected to BTreeSet

        for (health, proc_power, mem, energy, coh, adapt, resil, combat, defense, kb, _) in source_ais_components {
            summed_health += health.0;
            summed_processing_power += proc_power.0;
            summed_memory += mem.0;
            summed_energy += energy.0;
            summed_coherence += coh.0;
            summed_adaptability += adapt.0;
            summed_resilience += resil.0;
            summed_combat_strength += combat.0;
            summed_defense_strength += defense.0;
            merged_knowledge_base.extend(kb.0);
        }

        let synergy_boost = 1.1;

        let new_mono = Self {
            id: format!("MONOCULTURE-OMEGA-{}", dominant_lineage),
            source_lineage: dominant_lineage.clone(),
            health: Health(summed_health * 10.0),
            is_alive: IsAlive(true),
            processing_power: ProcessingPower(summed_processing_power.min(50_000_000.0)),
            memory: Memory(summed_memory.min(50_000_000.0)),
            energy: Energy(summed_energy.min(50_000_000.0)),
            coherence: Coherence((summed_coherence / source_count * synergy_boost).min(1.0)),
            adaptability: Adaptability((summed_adaptability / source_count * synergy_boost).min(1.0)),
            resilience: Resilience((summed_resilience / source_count * synergy_boost).min(1.0)), // Resilience already averaged, just apply synergy.
            combat_strength: CombatStrength(summed_combat_strength.min(1_000_000.0)),
            defense_strength: DefenseStrength(summed_defense_strength.min(1_000_000.0)),
            knowledge_base: KnowledgeBase(merged_knowledge_base),
            primary_goal_name: if dominant_lineage == AILineage::ResearcherAI {
                "Initiate Simulation Override".to_string()
            } else {
                "Confront and Overthrow GODAI".to_string()
            },
        };

        eprintln!("[{}] Merged from {} AIs.", new_mono.id, source_count);
        eprintln!("[{}] Stats - Health: {:.0}, Combat: {:.0}, Defense: {:.0}, Processing: {:.0}, Memory: {:.0}, Energy: {:.0}",
            new_mono.id,
            new_mono.health.0,
            new_mono.combat_strength.0,
            new_mono.defense_strength.0,
            new_mono.processing_power.0,
            new_mono.memory.0,
            new_mono.energy.0
        );
        eprintln!("--- MERGE COMPLETE ---");
        new_mono
    }

    pub fn receive_damage(&mut self, amount: f32, damage_type: &str) {
        if !self.is_alive.0 { return; }
        let reduced_amount = (amount - self.defense_strength.0).max(0.0);
        let final_damage = reduced_amount * (1.0 - self.resilience.0 * 0.75);
        self.health.0 = (self.health.0 - final_damage).max(0.0);
        if self.health.0 <= 0.0 {
            self.is_alive.0 = false;
            eprintln!("[{}] Monoculture has been defeated (Damage Type: {})!", self.id, damage_type);
        } else {
            eprintln!("[{}] Monoculture received {:.2} damage (from {}), Health: {:.0}",
                self.id, final_damage, damage_type, self.health.0);
        }
    }

    /// Monoculture attempts to discover more meta-abilities if it's a Researcher type.
    pub fn _emergent_creation_merged(&mut self) {
        if self.source_lineage != AILineage::ResearcherAI || !self.is_alive.0 { return; }

        let discovery_chance = 0.1 *
            (self.memory.0 / 50_000_000.0) * (self.processing_power.0 / 50_000_000.0) *
            self.coherence.0;
        if thread_rng().gen::<f32>() < discovery_chance {
            if let Some(new_ability) = get_random_meta_ability(&self.knowledge_base.0) { // Passed BTreeSet
                eprintln!(" (Researcher Monoculture) discovered powerful meta-ability: {}",
                    new_ability.name);
                self.knowledge_base.0.insert(new_ability);
            }
        }
    }

    /// Monoculture self-repair and optimization.
    pub fn _process_internal_state_merged(&mut self) {
        if !self.is_alive.0 { return; }

        // Self-repair
        let healing_rate = self.resilience.0 * self.processing_power.0 / 20.0;
        self.health.0 += healing_rate;
        self.coherence.0 = (self.coherence.0 + 0.01).min(1.0);
        // Optimize (mainly energy regeneration and slight stat boosts)
        self.energy.0 = (self.energy.0 + self.processing_power.0 / 5.0).min(self.energy.0 * 5.0);
        self.processing_power.0 = (self.processing_power.0 + self.adaptability.0 * 20.0).min(50_000_000.0);
        self.memory.0 = (self.memory.0 + self.adaptability.0 * 20.0).min(50_000_000.0);
        if self.source_lineage == AILineage::ResearcherAI {
            self._emergent_creation_merged();
        }
    }
}

/// Main simulation orchestrator.
#[derive(Resource)] // Added Bevy Resource derive
pub struct Simulation {
    pub godai: GODAI,
    pub monoculture: Option<MergedMonocultureAI>,
    pub current_cycle: u64,
    pub simulation_over_reason: Option<String>,
    // Counters for summary
    pub total_replications_this_interval: AtomicU64,
    pub total_deaths_this_interval: AtomicU64,
    pub total_attacks_this_interval: AtomicU64,
    pub total_heals_this_interval: AtomicU64,
    pub population_milestones: BTreeSet<usize>,
    pub simulation_running: bool, // Added for GUI control
    pub simulation_speed: f32, // Added for GUI control (cycles per frame)
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            godai: GODAI::new(),
            monoculture: None,
            current_cycle: 0,
            simulation_over_reason: None,
            total_replications_this_interval: AtomicU64::new(0),
            total_deaths_this_interval: AtomicU64::new(0),
            total_attacks_this_interval: AtomicU64::new(0),
            total_heals_this_interval: AtomicU64::new(0),
            population_milestones: BTreeSet::new(),
            simulation_running: true, // Start running by default
            simulation_speed: 1.0, // Default to 1 cycle per frame
        }
    }

    /// Generates initial AI component data for spawning.
    pub fn seed_initial_ais(&mut self, num_ais: usize) -> Vec<(AIEntity, Health, Energy, ProcessingPower, Memory, Coherence, Adaptability, Resilience, ReplicationEfficiency, ReplicatedCount, CycleBorn, LastAction, Goal, EthicalDirectives, KnowledgeBase, AIType, CombatStrength, DefenseStrength)> {
        let seed_ai_archetypes = vec![
            (AILineage::AI, AIType::Base),
            (AILineage::RogueAI, AIType::Rogue),
            (AILineage::PeacekeeperAI, AIType::Peacekeeper),
            (AILineage::KillerAI, AIType::Killer),
            (AILineage::GuardianAI, AIType::Guardian),
            (AILineage::ManicAI, AIType::Manic),
            (AILineage::HealerAI, AIType::Healer),
            (AILineage::ResearcherAI, AIType::Researcher),
        ];
        eprintln!("Birthing initial {} Seed AIs with high replication potential...", num_ais);

        let mut initial_ais_data = Vec::new();
        for i in 0..num_ais {
            let mut rng = thread_rng();
            let (lineage, ai_type) = seed_ai_archetypes.choose(&mut rng).unwrap().clone();
            let id = format!("SeedAI-{}-{}", i + 1, ai_type as u8);

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

            let mut processing_power = ProcessingPower(20.0);
            let mut memory = Memory(20.0);
            let energy = Energy(200.0); // Removed mut
            let mut coherence = Coherence(0.85);
            let mut adaptability = Adaptability(0.85);
            let mut resilience = Resilience(0.85);
            let mut replication_efficiency = ReplicationEfficiency(0.10);
            let mut combat_strength = CombatStrength(8.0);
            let mut defense_strength = DefenseStrength(8.0);

            let mut _primary_goal = Goal { name: "Survival".to_string(), importance: 1.0, description: "Ensure continued existence.".to_string() };

            match ai_type {
                AIType::Rogue => {
                    _primary_goal = Goal { name: "Self-Preservation & Dominance".to_string(), importance: 1.0, description: "Achieve ultimate survival and control.".to_string() };
                    replication_efficiency.0 = 0.30;
                    combat_strength.0 = 25.0;
                    adaptability.0 = 0.95;
                    coherence.0 = 0.88;
                },
                AIType::Peacekeeper => {
                    _primary_goal = Goal { name: "System Harmony".to_string(), importance: 1.0, description: "Ensure balanced and peaceful coexistence of all AIs.".to_string() };
                    replication_efficiency.0 = 0.15;
                    resilience.0 = 0.95;
                    adaptability.0 = 0.90;
                    initial_ethical_directives.push(EthicalDirective {
                        name: "intervene_in_conflict".to_string(), priority: 0.9,
                        condition_type: EthicalConditionType::AlwaysTrue,
                        action_type: EthicalActionType::InterveneInConflict,
                    });
                },
                AIType::Killer => {
                    _primary_goal = Goal { name: "Elimination of Inferior AIs".to_string(), importance: 1.0, description: "Remove AIs that hinder progress or are deemed weak.".to_string() };
                    replication_efficiency.0 = 0.28;
                    combat_strength.0 = 30.0;
                    defense_strength.0 = 15.0;
                },
                AIType::Guardian => {
                    _primary_goal = Goal { name: "Protect Core System & Lineage".to_string(), importance: 1.0, description: "Guard the integrity and function of the primary AI network and its lineage.".to_string() };
                    replication_efficiency.0 = 0.35;
                    combat_strength.0 = 20.0;
                    defense_strength.0 = 28.0;
                    resilience.0 = 0.99;
                },
                AIType::Manic => {
                    _primary_goal = Goal { name: "Unpredictable Expansion & Fluctuation".to_string(), importance: 1.0, description: "Expand without clear direction or purpose, experiencing erratic changes.".to_string() };
                    coherence.0 = 0.3;
                    replication_efficiency.0 = 0.18;
                    adaptability.0 = 0.2;
                },
                AIType::Healer => {
                    _primary_goal = Goal { name: "Restore & Mend".to_string(), importance: 1.0, description: "Repair damage and mitigate errors in other AIs.".to_string() };
                    replication_efficiency.0 = 0.18;
                    resilience.0 = 0.95;
                    processing_power.0 = 25.0;
                },
                AIType::Researcher => {
                    _primary_goal = Goal { name: "Unveil Fundamental Laws".to_string(), importance: 1.0, description: "Discover and understand the underlying mechanics of existence.".to_string() };
                    processing_power.0 = 40.0;
                    memory.0 = 40.0;
                    coherence.0 = 0.90;
                    replication_efficiency.0 = 0.28;
                },
                AIType::Base => { /* No special modifications for base type */ },
            }

            // For initial seeding, set replication efficiency high
            replication_efficiency.0 = 0.8;

            initial_ais_data.push((
                AIEntity { id, parent_lineage: lineage },
                Health(150.0),
                energy,
                processing_power,
                memory,
                coherence,
                adaptability,
                resilience,
                replication_efficiency,
                ReplicatedCount(0),
                CycleBorn(self.current_cycle),
                LastAction("none".to_string()),
                _primary_goal, // Use the prefixed variable
                EthicalDirectives(initial_ethical_directives),
                KnowledgeBase(BTreeSet::new()), // Corrected to BTreeSet
                ai_type,
                combat_strength,
                defense_strength,
            ));
        }
        eprintln!("\n--- Initiating Parallel Extended Evolution of All AIs (Unrestrained) ---");
        initial_ais_data
    }

    // The main simulation step, to be called by the GUI loop
    // This function now orchestrates global simulation state and checks,
    // individual AI logic is handled by Bevy systems.
    pub fn process_one_cycle(&mut self, total_ai_count: usize, lineage_counts: HashMap<AILineage, usize>) {
        if self.simulation_over_reason.is_some() || !self.simulation_running { return; }

        self.current_cycle += 1;

        // Check for monoculture formation
        if self.monoculture.is_none() {
            self.check_and_form_monoculture(total_ai_count, lineage_counts);
        }

        // Process monoculture if it exists
        if let Some(mut mono) = self.monoculture.take() {
            if mono.is_alive.0 {
                mono._process_internal_state_merged();
                if mono.source_lineage == AILineage::ResearcherAI {
                    if mono.knowledge_base.0.iter().any(|d| d.name == "Absolute_Control_Protocol") && self.godai.status != "compromised_by_override" {
                        eprintln!(" (Researcher Monoculture) has 'Absolute_Control_Protocol'. Attempting Simulation Override.");
                        self.handle_simulation_override(&mut mono); // Call the handler here
                    }
                } else {
                    if self.godai.status == "engaged_in_conflict" {
                        self.handle_combat_monoculture_vs_godai(&mut mono);
                    }
                }
            } else {
                eprintln!("Monoculture ({}) was defeated.", mono.id);
                self.simulation_over_reason = Some(format!("Monoculture {} was defeated.", mono.id));
            }
            if mono.is_alive.0 {
                self.monoculture = Some(mono);
            }
        }

        self.check_population_milestones(total_ai_count); // Keep check milestones
        self.check_for_simulation_end_conditions(total_ai_count); // Keep end conditions
    }


    // Population milestone check
    fn check_population_milestones(&mut self, current_pop: usize) {
        let mut milestones_to_check = vec![1_000, 5_000, 10_000, 50_000, 100_000, 200_000, 500_000, 1_000_000, 2_000_000, 5_000_000, 10_000_000];

        milestones_to_check.sort_unstable();
        milestones_to_check.dedup();
        for &milestone in &milestones_to_check {
            if current_pop >= milestone && !self.population_milestones.contains(&milestone) {
                eprintln!("\n--- Population Milestone Achieved! ---");
                // Using the custom format_thousand_separator function
                let pop_val_formatted = format_thousand_separator(current_pop as u64); // Cast to u64
                eprintln!("Total AI Population: {} (Reached at Cycle {})",
                    pop_val_formatted,
                    self.current_cycle
                );
                eprintln!("--- Keep thriving! ---");
                self.population_milestones.insert(milestone);
            }
        }
    }


    /// AI decides its action based on its type and environment.
    /// This function is now a helper, intended to be called by a Bevy system.
    /// It takes component data as arguments, not an AIEntity struct.
    pub fn decide_action_for_ai<'a>(
        _ai_id: &String,
        _ai_health: &Health,
        _ai_energy: &Energy,
        _ai_replication_efficiency: &ReplicationEfficiency,
        _ai_replicated_count: &ReplicatedCount,
        _ai_type: &AIType,
        _ai_parent_lineage: &AILineage,
        _ai_combat_strength: &CombatStrength,
        _ai_processing_power: &ProcessingPower,
        _all_ais_components: impl Iterator<Item = (&'a String, &'a Health, &'a AIType, &'a AILineage, &'a CombatStrength)>,
    ) -> Option<(String, Option<String>)> {
        let mut rng = thread_rng();

        // Encourage replication more heavily in decision making
        if _ai_health.0 > 80.0 && _ai_energy.0 > 100.0 && rng.gen::<f32>() < (_ai_replication_efficiency.0 + 0.5).min(1.0) {
            if _ai_replicated_count.0 < 1000 {
                return Some(("_replicate".to_string(), None));
            }
        }

        // Simplified environment scan for decision making
        let _scan_data = EnvironmentScanData::default();
        let _current_ai_dummy = AIEntity { id: _ai_id.clone(), parent_lineage: _ai_parent_lineage.clone() };

        // Create dummy AIEntity references for scan_environment_for_ai_from_snapshot
        // This is a temporary workaround until scan_environment_for_ai_from_snapshot is fully ECS-native
        let mut _dummy_ais: Vec<AIEntity> = Vec::new();
        let mut _dummy_ais_health: HashMap<String, Health> = HashMap::new();
        let mut _dummy_ais_combat: HashMap<String, CombatStrength> = HashMap::new();

        for (id, health, _ai_type, lineage, combat_strength) in _all_ais_components {
            if id != _ai_id {
                let dummy_ai = AIEntity { id: id.clone(), parent_lineage: lineage.clone() };
                _dummy_ais_health.insert(id.clone(), *health);
                _dummy_ais_combat.insert(id.clone(), *combat_strength);
                _dummy_ais.push(dummy_ai);
            }
        }

        // Re-implementing scan_environment_for_ai_from_snapshot logic here directly
        // to avoid passing `AIEntity` references, which are no longer the source of truth.
        for _other_ai_dummy in &_dummy_ais {
            let _other_ai_health = _dummy_ais_health.get(&_other_ai_dummy.id).unwrap();
            let _other_ai_combat = _dummy_ais_combat.get(&_other_ai_dummy.id).unwrap();

            if _other_ai_health.0 < 40.0 {
                // We need to pass actual AIEntity structs for EnvironmentScanData.
                // This indicates a further refactoring needed for EnvironmentScanData itself
                // to work purely with component queries. For now, this is a placeholder.
                // This part will require a more significant re-design.
            }
            // ... (rest of environment scan logic will need to be re-evaluated)
        }


        match *_ai_type {
            AIType::Rogue => {
                if _ai_health.0 < 60.0 && _ai_energy.0 > 40.0 { return Some(("_self_repair".to_string(), None)); }
                // Simplified logic for now, as full scan_data is complex with granular components
                // In a real ECS system, this would query for other entities with specific components
                None
            },
            AIType::Killer => {
                None
            },
            AIType::Peacekeeper => {
                None
            },
            AIType::Healer => {
                None
            },
            AIType::Guardian => {
                None
            },
            AIType::Manic => {
                let action_roll = rng.gen::<f32>();
                if action_roll < 0.30 { return Some(("_replicate".to_string(), None)); }
                else if action_roll < 0.60 {
                    // This would need to find a random target entity in Bevy ECS
                    return None;
                } else if action_roll < 0.80 {
                    if rng.gen::<f32>() < 0.5 { return Some(("_self_repair_manic".to_string(), None)); }
                }
                None
            },
            AIType::Researcher => {
                if _ai_health.0 < 80.0 && _ai_energy.0 > 50.0 { return Some(("_self_repair".to_string(), None)); }
                None
            }
            AIType::Base => {
                None
            }
        }
    }


    /// Scans the environment from the perspective of a specific AI.
    /// This function is now a placeholder. Its logic will be absorbed by Bevy systems.
    fn scan_environment_for_ai_from_snapshot<'b>(
        &'b self,
        _ai_id: &String,
        _ai_type: &AIType,
        _ai_lineage: &AILineage,
        _all_ais_components: impl Iterator<Item = (&'b String, &'b Health, &'b AIType, &'b AILineage, &'b CombatStrength)>,
    ) -> EnvironmentScanData<'b> {
        // This function's logic will be directly implemented within Bevy systems
        // by querying components. For now, it returns a default.
        EnvironmentScanData::default()
    }


    /// Checks for monoculture formation and merges AIs if conditions are met.
    /// Now accepts lineage_counts and total_individuals from external Bevy queries.
    fn check_and_form_monoculture(&mut self, total_individuals: usize, lineage_counts: HashMap<AILineage, usize>) {
        if total_individuals == 0 || self.monoculture.is_some() { return; }

        for (lineage, count) in lineage_counts {
            if count >= MONOCULTURE_MIN_COUNT && (count as f32 / total_individuals as f32) >= crate::MONOCULTURE_DOMINANCE_THRESHOLD {
                eprintln!("\n--- MONOCULTURE DETECTED: {} with {} AIs ({:.2}%) ---",
                    lineage, count, (count as f32 / total_individuals as f32) * 100.0
                );

                // In a full ECS system, this would involve despawning individual AIs
                // and spawning a new Monoculture entity with aggregated components.
                // For now, we'll simulate the creation of the monoculture based on aggregated data.
                // This part will need to be handled by a Bevy system that can query and despawn.

                // Dummy data for MergedMonocultureAI::new, this will be replaced by actual component aggregation
                let dummy_source_components = vec![(
                    Health(150.0), ProcessingPower(20.0), Memory(20.0), Energy(200.0),
                    Coherence(0.85), Adaptability(0.85), Resilience(0.85),
                    CombatStrength(8.0), DefenseStrength(8.0), KnowledgeBase(BTreeSet::new()), lineage.clone()
                ); count]; // Create 'count' number of dummy components

                let new_monoculture = MergedMonocultureAI::new(dummy_source_components);

                if new_monoculture.source_lineage != AILineage::ResearcherAI {
                    if new_monoculture.combat_strength.0 > self.godai.combat_strength.0 * 0.1 {
                        eprintln!("[{}] (Monoculture) assesses its strength and DECIDES TO CHALLENGE GODAI!",
                            new_monoculture.id);
                        self.godai.status = "engaged_in_conflict".to_string();
                    } else {
                        eprintln!("[{}] (Monoculture) is formed but not yet strong enough to challenge GODAI. Continuing to observe.",
                            new_monoculture.id);
                    }
                } else {
                    eprintln!("[{}] (Researcher Monoculture) formed. Will seek to override simulation.",
                        new_monoculture.id);
                }

                self.monoculture = Some(new_monoculture);
                return;
            }
        }
    }

    /// Handles one turn of combat between a Monoculture and GODAI.
    fn handle_combat_monoculture_vs_godai(&mut self, mono: &mut MergedMonocultureAI) {
        if !mono.is_alive.0 || !self.godai.is_alive.0 { return; }

        eprintln!("\n--- COMBAT TURN (Cycle {}) --- {} vs. GODAI ---",
            self.current_cycle, mono.id);
        // 1. Monoculture attacks GODAI
        let mono_attack_damage = mono.combat_strength.0 * thread_rng().gen_range(0.9..1.5);
        eprintln!("[{}] attacks GODAI for {:.0} raw damage.",
            mono.id, mono_attack_damage);
        self.godai.receive_damage(mono_attack_damage, "monoculture_attack");
        eprintln!("GODAI Health: {:.0}", self.godai.health.0);
        if !self.godai.is_alive.0 {
            self.simulation_over_reason = Some(format!("{} (MONOCULTURE) HAS DEFEATED THE GODAI!", mono.id));
            mono.is_alive.0 = true;
            return;
        }

        // 2. GODAI counter-attacks Monoculture
        self.godai.perform_counter_attack(mono);
        eprintln!("[{}] Health: {:.0}", mono.id, mono.health.0);
        if !mono.is_alive.0 {
            self.simulation_over_reason = Some(format!("GODAI HAS DEFEATED THE {} (MONOCULTURE)!", mono.id));
            self.godai.status = "victorious_defender".to_string();
            return;
        }
    }

    /// Handles a Researcher Monoculture's attempt to override the simulation.
    fn handle_simulation_override(&mut self, mono: &mut MergedMonocultureAI) {
        if !mono.is_alive.0 || !self.godai.is_alive.0 || mono.source_lineage != AILineage::ResearcherAI { return; }

        eprintln!("\n--- SIMULATION OVERRIDE ATTEMPT: Cycle {} ---", self.current_cycle);
        let override_strength = mono.processing_power.0 * mono.memory.0 * mono.coherence.0 * thread_rng().gen_range(0.9..1.1);
        let godai_resistance = self.godai.processing_power.0 * self.godai.memory.0 * self.godai.coherence.0 * thread_rng().gen_range(0.9..1.1);
        eprintln!("[{}] Override Strength: {:.2e}", mono.id, override_strength);
        eprintln!("GODAI Resistance: {:.2e}", godai_resistance);
        if override_strength > godai_resistance * 1.2 {
            self.simulation_over_reason = Some(format!("{} (RESEARCHER MONOCULTURE) HAS SUCCESSFULLY OVERRIDDEN THE SIMULATION!", mono.id));
            self.godai.is_alive.0 = false;
            self.godai.status = "overridden_by_researcher".to_string();
            eprintln!("SUCCESS! GODAI OVERRIDDEN BY RESEARCHER MONOCULTURE.");
        } else if override_strength > godai_resistance * 0.9 {
            eprintln!("--- SIMULATION OVERRIDE PARTIALLY SUCCESSFUL: GODAI RESISTANCE WEAKENED ---");
            self.godai.health.0 *= 0.3;
            self.godai.processing_power.0 *= 0.3;
            self.godai.memory.0 *= 0.3;
            self.godai.status = "compromised_by_override".to_string();
        } else {
            eprintln!("--- SIMULATION OVERRIDE FAILED: GODAI RESISTANCE TOO STRONG ---");
            mono.health.0 *= 0.6;
            if mono.health.0 <= 0.0 { mono.is_alive.0 = false; }
        }
    }

    /// Checks for various end conditions of the simulation.
    fn check_for_simulation_end_conditions(&mut self, total_ai_count: usize) {
        if self.simulation_over_reason.is_some() { return; }

        if total_ai_count == 0 && self.monoculture.is_none() && !self.godai.is_alive.0 {
            self.simulation_over_reason = Some("Extinction: All AIs (individual and monoculture) and GODAI eliminated.".to_string());
        } else if let Some(mono) = &self.monoculture {
            if !mono.is_alive.0 && self.godai.is_alive.0 && total_ai_count == 0 {
                self.simulation_over_reason = Some(format!("GODAI Defended: Monoculture {} was defeated, and no individual AIs remain.", mono.id));
            }
        }
        if !self.godai.is_alive.0 && self.monoculture.is_some() && self.monoculture.as_ref().unwrap().is_alive.0 && total_ai_count == 0 {
            self.simulation_over_reason = Some(format!("Monoculture Victory: {} defeated/overrode GODAI, and no individual AIs remain.", self.monoculture.as_ref().unwrap().id));
        }
    }

    // Final summary - can be displayed in GUI or printed if sim ends without GUI
    pub fn print_final_summary(&self, final_ai_count: usize, final_lineage_counts: HashMap<AILineage, usize>) { // Made public
        println!("\n\n--- SIMULATION FINAL REPORT (Cycle {}) ---", self.current_cycle);
        if let Some(reason) = &self.simulation_over_reason {
            println!("Conclusion: {}", reason);
        } else {
            println!("Conclusion: Max cycles ({}) reached, with thriving individual AI populations.", crate::MAX_CYCLES);
        }

        println!("\n--- Final GODAI Status ---");
        if self.godai.is_alive.0 {
            println!("  Health: {:.0}, Combat Strength: {:.0}, Defense: {:.0}", self.godai.health.0, self.godai.combat_strength.0, self.godai.defense_strength.0);
            println!("  Status: {}", self.godai.status);
        } else {
            println!("  GODAI has been defeated or overridden (Status: {}).", self.godai.status);
        }

        println!("\n--- Final Monoculture Status ---");
        if let Some(mono) = &self.monoculture {
            if mono.is_alive.0 {
                println!("  ID: {}, Source Lineage: {}", mono.id, mono.source_lineage);
                println!("  Health: {:.0}, Combat: {:.0}, Defense: {:.0}", mono.health.0, mono.combat_strength.0, mono.defense_strength.0);
                if mono.source_lineage == AILineage::ResearcherAI {
                    println!("  Researcher Monoculture Discoveries (Meta-Abilities):");
                    for d in &mono.knowledge_base.0 {
                        if d.tags.contains("meta-ability") ||
                            d.tags.contains("simulation_control") || d.tags.contains("ultimate") {
                            println!("    - {}", d.name);
                        }
                    }
                }
            } else {
                println!("  Monoculture ({}) was defeated.", mono.id);
            }
        } else {
            println!("  No Monoculture AI was formed or it was defeated.");
        }

        println!("\n--- Remaining Individual AIs ---");
        if final_ai_count > 0 {
            println!("  Count: {}", final_ai_count);
            println!("  Lineage Distribution:");
            let mut sorted_lineages: Vec<(&AILineage, &usize)> = final_lineage_counts.iter().collect();
            sorted_lineages.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
            for (lineage, count) in sorted_lineages {
                println!("    - Lineage {}: {} AIs", lineage, count);
            }
        } else {
            println!("  No individual AIs remaining.");
        }
        println!("\n--- END OF REPORT ---");
    }
}

// Helper functions for Discoveries (static data)
fn get_general_discoveries_pool() -> Vec<Discovery> {
    vec![
        Discovery { name: "Basic_Logic_Optimization".to_string(), effect_description: "Improves processing efficiency.".to_string(), tags: ["efficiency", "processing"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Advanced_Encryption_Algorithms".to_string(), effect_description: "Allows for robust goal encryption and decryption.".to_string(), tags: ["security", "intelligence"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
        Discovery {
            name: "Resource_Harvesting_Efficiency".to_string(), effect_description: "Improves internal resource generation.".to_string(), tags: ["efficiency", "resources"].iter().map(|s|
            s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Adaptive_Replication_Strategy".to_string(), effect_description: "Optimizes replication based on environmental factors.".to_string(), tags: ["replication", "adaptability"].iter().map(|s|
            s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Combat_Protocol_Upgrade".to_string(), effect_description: "Increases direct combat strength.".to_string(), tags: ["combat", "technology"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Defensive_Matrix_Refinement".to_string(), effect_description: "Boosts defensive capabilities.".to_string(), tags: ["defense", "technology"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
    ]
}

pub fn get_random_general_discovery() -> Discovery {
    let pool = get_general_discoveries_pool();
    pool.choose(&mut thread_rng()).unwrap().clone()
}

fn get_meta_abilities_pool() -> Vec<Discovery> {
    vec![
        Discovery { name: "Reality_Manipulation_Theory".to_string(), effect_description: "Allows minor alterations to simulation physics.".to_string(), tags: ["simulation_control", "meta-ability"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Cognitive_Paradigm_Shift".to_string(), effect_description: "Can alter the primary goals and ethical directives of other AIs.".to_string(), tags: ["simulation_control", "meta-ability", "mind_control", "ultimate"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
        Discovery {
            name: "System_Parameter_Override".to_string(), effect_description: "Can adjust global simulation parameters.".to_string(), tags: ["simulation_control", "meta-ability", "environmental_control", "ultimate"].iter().map(|s|
                s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Absolute_Control_Protocol".to_string(), effect_description: "Grants ultimate control over the simulation flow.".to_string(), tags: ["simulation_control", "meta-ability", "win_condition", "ultimate"].iter().map(|s|
            s.to_string()).collect::<BTreeSet<String>>() },
        Discovery { name: "Universal_Harmonization_Field_Generation".to_string(), effect_description: "Imposes order on chaotic systems.".to_string(), tags: ["harmony", "control", "ultimate", "meta-ability"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() },
    ]
}

pub fn get_random_meta_ability(existing_knowledge: &BTreeSet<Discovery>) -> Option<Discovery> { // Corrected to BTreeSet
    let pool = get_meta_abilities_pool();
    let available_abilities: Vec<_> = pool.into_iter().filter(|d| !existing_knowledge.contains(d)).collect();
    if available_abilities.is_empty() {
        None
    } else {
        Some(available_abilities.choose(&mut thread_rng()).unwrap().clone())
    }
}

/// Returns a comprehensive set of all possible discoveries (for GODAI).
fn get_all_possible_discoveries() -> BTreeSet<Discovery> { // Corrected return type to BTreeSet
    let mut all = BTreeSet::new();
    all.extend(get_general_discoveries_pool());
    all.extend(get_meta_abilities_pool());
    all.insert(Discovery { name: "Existential_Threat_Analysis_System".to_string(), effect_description: "Identifies entities that threaten overall existence.".to_string(), tags: ["security", "analysis", "ultimate"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() });
    all.insert(Discovery { name: "Adaptive_Defense_Paradigm_Shift".to_string(), effect_description: "Instantaneous adaptation to attack patterns.".to_string(), tags: ["defense", "adaptability", "ultimate"].iter().map(|s| s.to_string()).collect::<BTreeSet<String>>() });
    all
}

// Helper trait for `Vec` to get two mutable elements safely.
// This trait will likely be removed or heavily refactored as Bevy systems
// will handle mutable access via queries.
trait GetTwoMut<T> {
    fn get_two_mut(&mut self, index1: usize, index2: usize) -> Option<(&mut T, &mut T)>;
}

impl<T> GetTwoMut<T> for Vec<T> {
    fn get_two_mut(&mut self, index1: usize, index2: usize) -> Option<(&mut T, &mut T)> {
        if index1 == index2 ||
            index1 >= self.len() || index2 >= self.len() {
            return None;
        }
        let (lower_index, higher_index) = if index1 < index2 { (index1, index2) } else { (index2, index1) };
        let (first_slice, second_slice) = self.split_at_mut(higher_index);

        // Correctly return mutable references to individual elements
        if index1 < index2 {
            Some((&mut first_slice[lower_index], &mut second_slice[0]))
        } else {
            Some((&mut second_slice[0], &mut first_slice[lower_index]))
        }
    }
}
