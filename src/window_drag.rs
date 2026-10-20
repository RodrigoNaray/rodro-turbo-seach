//! Move the native surface after the normal drag threshold, preserving simple clicks.
use crate::dismissal::Interaction;
use gtk::{gdk, glib, prelude::*};
use std::{cell::RefCell, rc::Rc};

struct Press {
    x: f64,
    y: f64,
    event: gdk::Event,
    target: glib::WeakRef<gtk::Widget>,
    moving: bool,
}

pub fn install(
    window: &gtk::ApplicationWindow,
    excluded: &gtk::ToggleButton,
    interaction: Rc<Interaction>,
) {
    let events = gtk::EventControllerLegacy::new();
    events.set_propagation_phase(gtk::PropagationPhase::Capture);
    let weak_window = window.downgrade();
    let weak_excluded = excluded.downgrade();
    let press: RefCell<Option<Press>> = RefCell::new(None);
    events.connect_event(move |_, event| {
        let (Some(window), Some(excluded)) = (weak_window.upgrade(), weak_excluded.upgrade())
        else {
            return glib::Propagation::Proceed;
        };
        match event.event_type() {
            gdk::EventType::ButtonPress => {
                press.borrow_mut().take();
                if event
                    .downcast_ref::<gdk::ButtonEvent>()
                    .is_none_or(|e| e.button() != 1)
                {
                    return glib::Propagation::Proceed;
                }
                interaction.pressed.set(true);
                interaction.moving.set(false);
                if let Some((x, y)) = event.position()
                    && let Some(target) = window.pick(x, y, gtk::PickFlags::DEFAULT)
                    && target != excluded.clone().upcast::<gtk::Widget>()
                    && !target.is_ancestor(&excluded)
                {
                    *press.borrow_mut() = Some(Press {
                        x,
                        y,
                        event: event.clone(),
                        target: target.downgrade(),
                        moving: false,
                    });
                }
            }
            gdk::EventType::MotionNotify => {
                // The compositor may consume the release after a native move.
                if !event
                    .modifier_state()
                    .contains(gdk::ModifierType::BUTTON1_MASK)
                {
                    press.borrow_mut().take();
                    interaction.clear();
                    return glib::Propagation::Proceed;
                }
                let mut press = press.borrow_mut();
                let Some(press) = press.as_mut() else {
                    return glib::Propagation::Proceed;
                };
                if press.moving {
                    return glib::Propagation::Stop;
                }
                let Some((x, y)) = event.position() else {
                    return glib::Propagation::Proceed;
                };
                let threshold = window.settings().property::<i32>("gtk-dnd-drag-threshold") as f64;
                if (x - press.x).abs().max((y - press.y).abs()) <= threshold {
                    return glib::Propagation::Proceed;
                }
                if let Some(surface) = window.surface().and_downcast::<gdk::Toplevel>()
                    && let Some(device) = press.event.device()
                {
                    // Cancel the original widget's click/selection gestures so dragging
                    // a result or calculator button never activates it on release.
                    let mut widget = press.target.upgrade();
                    while let Some(current) = widget {
                        if current == window.clone().upcast::<gtk::Widget>() {
                            break;
                        }
                        let controllers = current.observe_controllers();
                        for i in 0..controllers.n_items() {
                            if let Some(gesture) =
                                controllers.item(i).and_downcast::<gtk::Gesture>()
                            {
                                gesture.reset();
                            }
                        }
                        widget = current.parent();
                    }
                    press.moving = true;
                    interaction.moving.set(true);
                    surface.begin_move(&device, 1, x, y, press.event.time());
                    return glib::Propagation::Stop;
                }
            }
            gdk::EventType::ButtonRelease
                if event
                    .downcast_ref::<gdk::ButtonEvent>()
                    .is_some_and(|e| e.button() == 1) =>
            {
                interaction.clear();
                if press.borrow_mut().take().is_some_and(|p| p.moving) {
                    return glib::Propagation::Stop;
                }
            }
            _ => {}
        }
        glib::Propagation::Proceed
    });
    window.add_controller(events);
}
