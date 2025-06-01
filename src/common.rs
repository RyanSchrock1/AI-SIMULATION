use std::collections::BTreeSet;
use std::fmt;
use bevy::prelude::Component; // Import Component from Bevy

/// Represents a piece of knowledge or technological breakthrough.
#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)] // Added PartialOrd and Ord for BTreeSet
pub struct Discovery {
    pub name: String,
    pub effect_description: String,
    pub tags: BTreeSet<String>,
}

impl fmt::Debug for Discovery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Discovery('{}')", self.name)
    }
}

/// Defines an AI's objective.
#[derive(Debug, Clone, Component)] // Goal can also be a component
pub struct Goal {
    pub name: String,
    pub importance: f32,
    pub description: String,
}

/// Core attributes defining an AI's capabilities.
/// This struct will eventually be replaced by individual components.
/// For now, it remains for backward compatibility during refactoring.
#[derive(Debug, Clone, Default, Component)] // Make it a component for now
pub struct CoreAttributes {
    pub processing_power: f32,
    pub memory: f32,
    pub energy: f32,
    pub coherence: f32,
    pub adaptability: f32,
    pub resilience: f32,
    pub replication_efficiency: f32,
    pub combat_strength: f32,
    pub defense_strength: f32,
}

// --- Granular Components for AI Attributes ---
#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Health(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Energy(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ProcessingPower(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Memory(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coherence(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Adaptability(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Resilience(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ReplicationEfficiency(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CombatStrength(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DefenseStrength(pub f32);

#[derive(Component, Debug, Clone)]
pub struct LastAction(pub String);

#[derive(Component, Debug, Clone)]
pub struct KnowledgeBase(pub BTreeSet<Discovery>); // Using BTreeSet for ordered, unique discoveries

#[derive(Component, Debug, Clone)]
pub struct EthicalDirectives(pub Vec<EthicalDirective>);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsAlive(pub bool);

#[derive(Component, Debug, Clone, Copy)]
pub struct ReplicatedCount(pub u32);

#[derive(Component, Debug, Clone, Copy)]
pub struct CycleBorn(pub u64);

/// Defines specific actions an EthicalDirective can trigger.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EthicalActionType {
    SelfRepair,
    OptimizeSelf,
    ProhibitReplication,
    InterveneInConflict,
    NoOp,
    ManicSelfRepair,
}

/// Defines specific conditions an EthicalDirective can check.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EthicalConditionType {
    HealthBelowThreshold(f32),
    CoherenceBelowThreshold(f32),
    ResourcesBelowThreshold,
    AlwaysTrue,
    AlwaysFalse,
}

/// Governs an AI's ethical behavior.
/// `condition_type` specifies the condition to check.
/// `action_type` specifies the action to be performed by the AI itself.
#[derive(Debug, Clone)]
pub struct EthicalDirective {
    pub name: String,
    pub priority: f32,
    pub condition_type: EthicalConditionType,
    pub action_type: EthicalActionType,
}

/// Data structure for environment scanning results.
#[derive(Default)]
pub struct EnvironmentScanData<'a> {
    // These will eventually query components directly
    pub allies: Vec<&'a super::ai::AIEntity>,
    pub threats: Vec<&'a super::ai::AIEntity>,
    pub vulnerable_targets: Vec<&'a super::ai::AIEntity>,
    pub neutral_ais: Vec<&'a super::ai::AIEntity>,
    pub critically_damaged: Vec<&'a super::ai::AIEntity>,
    pub moderately_damaged: Vec<&'a super::ai::AIEntity>,
}
