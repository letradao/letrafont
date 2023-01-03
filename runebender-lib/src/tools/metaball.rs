//! The metaball tool

use druid::{Env, EventCtx, KbKey, KeyEvent, MouseEvent};

use crate::design_space::DPoint;
use crate::edit_session::EditSession;
use crate::mouse::{Drag, Mouse, MouseDelegate, TaggedEvent};
use crate::point::{EntityId, PathPoint};
use crate::tools::{EditType, Tool, ToolId};

/// The state of the Metaball tool.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Metaball {
    state: State,
}

impl Metaball {
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Ready,
    /// The mouse is down and has added a new point.
    AddPoint(EntityId),
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