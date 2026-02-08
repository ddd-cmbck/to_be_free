// src/app/schedules.rs
use bevy::prelude::*;

/// High-level system sets used across the app.
///
/// We keep these in one place so features can consistently order their systems
/// without inventing new ad-hoc labels everywhere.
///
/// Note: sets are configured per-schedule (Update vs FixedUpdate), so the actual
/// `.configure_sets(...)` calls live in `AppPlugin`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppSet {
    /// Variable timestep (`Update`) input gathering and intent generation.
    Input,

    /// Fixed timestep (`FixedUpdate`) movement / physics stepping.
    FixedMovement,
}
