//! # Model
//!
//! `model` contains the data models used across the crate.
use serde::{Deserialize, Serialize};

/// Base struct for a todo item
///
/// This will hold the information for a single todo item
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    /// Unique identifier
    pub id: u64,
    /// The todo content
    pub content: String,
    /// The state as defined in TodoState
    pub state: TodoState,
}

/// The todo state
#[derive(Serialize, Deserialize, Debug)]
pub enum TodoState {
    /// Todo is pending completion
    Pending,
    /// Todo is completed
    Completed,
}
