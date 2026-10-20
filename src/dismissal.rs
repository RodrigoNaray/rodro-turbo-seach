//! Dismiss on outside focus changes, not during a click or compositor move.
use gtk::{glib, prelude::*};
use std::{cell::Cell, rc::Rc};

#[derive(Default)]
pub struct Interaction {
    pub pressed: Cell<bool>,
    pub moving: Cell<bool>,
}
impl Interaction {
    pub fn clear(&self) {
        self.pressed.set(false);
        self.moving.set(false);
    }
    fn should_hide(&self, active: bool, inside: bool, was_active: bool) -> bool {
        was_active && !active && !inside && !self.pressed.get() && !self.moving.get()
    }
}

pub fn install(window: &gtk::ApplicationWindow, interaction: Rc<Interaction>) {
    let pointer = gtk::EventControllerMotion::new();
    pointer.set_propagation_phase(gtk::PropagationPhase::Capture);
    let activated = Rc::new(Cell::new(false));
    let check: Rc<dyn Fn()> = Rc::new({
        let window = window.downgrade();
        let pointer = pointer.downgrade();
        let interaction = interaction.clone();
        let activated = activated.clone();
        move || {
            let (Some(window), Some(pointer)) = (window.upgrade(), pointer.upgrade()) else {
                return;
            };
            // Let GTK finish processing pointer/focus events before deciding.
            let interaction = interaction.clone();
            let activated = activated.clone();
            glib::idle_add_local_once(move || {
                if window.is_visible()
                    && interaction.should_hide(
                        window.is_active(),
                        pointer.contains_pointer(),
                        activated.get(),
                    )
                {
                    window.set_visible(false);
                }
            });
        }
    });
    let check_focus = check.clone();
    window.connect_is_active_notify(move |window| {
        if window.is_active() {
            activated.set(true);
            // A native move can consume its release; keyboard focus returning
            // to the window marks the end of that compositor interaction.
            interaction.clear();
        } else {
            check_focus();
        }
    });
    pointer.connect_leave(move |_| check());
    window.add_controller(pointer);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn focus_loss_during_click_or_move_keeps_window_open() {
        let interaction = Interaction::default();
        assert!(!interaction.should_hide(false, false, false));
        assert!(!interaction.should_hide(true, false, true));
        assert!(!interaction.should_hide(false, true, true));
        interaction.pressed.set(true);
        assert!(!interaction.should_hide(false, false, true));
        interaction.moving.set(true);
        interaction.pressed.set(false); // native move consumed the release
        assert!(!interaction.should_hide(false, false, true));
        interaction.clear();
        assert!(interaction.should_hide(false, false, true));
    }
}
