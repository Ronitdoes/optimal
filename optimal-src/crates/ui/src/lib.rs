//! GPUI window (thin surface). The window never touches the OS; it renders a
//! projection of `harness.v1.stream` notifications from the Rust core.
//!
//! The `ui` feature is opt-in because GPUI 0.2.2's native build needs
//! libclang / a Windows resource compiler (see docs/DEPENDENCY-PINS.md). The
//! stream-to-view state machine is feature-independent and fully testable.

use optimal_gateway::StreamEvent;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiState {
    pub tokens: String,
    pub badges: Vec<String>,
    pub done: bool,
    pub killed: bool,
}

impl UiState {
    pub fn apply(&mut self, event: &StreamEvent) {
        match event {
            StreamEvent::Token { text } => self.tokens.push_str(text),
            StreamEvent::ToolBadge { id, tier } => self.badges.push(format!("{id}:{tier}")),
            StreamEvent::Done { .. } => self.done = true,
            StreamEvent::Killed { .. } => self.killed = true,
        }
    }
}

#[cfg(feature = "ui")]
mod window;
#[cfg(feature = "ui")]
pub use window::run_ui;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_events_project_into_view_state() {
        let mut s = UiState::default();
        s.apply(&StreamEvent::Token { text: "Hel".into() });
        s.apply(&StreamEvent::Token { text: "lo".into() });
        s.apply(&StreamEvent::ToolBadge { id: "env.probe".into(), tier: "readonly".into() });
        s.apply(&StreamEvent::Done { task_id: "t".into() });
        assert_eq!(s.tokens, "Hello");
        assert_eq!(s.badges, vec!["env.probe:readonly"]);
        assert!(s.done && !s.killed);
    }
}
