//! The metaball tool

use crate::tools::{Tool, ToolId};

/// The state of the Metaball tool.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Metaball {
    state: State,
}

impl Metaball {
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Ready,
}

impl Tool for Metaball {
    fn name(&self) -> ToolId {
        "Metaball"
    }
}

impl Default for State {
    fn default() -> Self {
        State::Ready
    }
}