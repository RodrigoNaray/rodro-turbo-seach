//! A clipped color wipe, redrawn only during the short transition.
use gtk::{glib, prelude::*};
use std::{cell::Cell, rc::Rc};

pub struct Background {
    pub widget: gtk::DrawingArea,
    progress: Rc<Cell<f64>>,
    target: Rc<Cell<f64>>,
    running: Rc<Cell<bool>>,
}
impl Background {
    pub fn new() -> Self {
        let widget = gtk::DrawingArea::new();
        widget.set_can_target(false);
        let progress = Rc::new(Cell::new(0.0_f64));
        let value = progress.clone();
        widget.set_draw_func(move |_, cr, width, height| {
            let (w, h) = (width as f64, height as f64);
            let r = 24.0_f64.min(h / 2.0).min(w / 2.0);
            for (x, y, angle) in [
                (w - r, r, -90.0_f64),
                (w - r, h - r, 0.0),
                (r, h - r, 90.0),
                (r, r, 180.0),
            ] {
                cr.arc(x, y, r, angle.to_radians(), (angle + 90.0).to_radians());
            }
            cr.close_path();
            cr.clip();
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.82);
            let _ = cr.paint();
            let p = value.get();
            let eased = p * p * (3.0 - 2.0 * p);
            cr.rectangle(0.0, 0.0, w * eased, h);
            cr.clip();
            cr.set_operator(gtk::cairo::Operator::Source);
            cr.set_source_rgba(0.30, 0.12, 0.49, 0.82);
            let _ = cr.paint();
        });
        Self {
            widget,
            progress,
            target: Rc::new(Cell::new(0.0)),
            running: Rc::new(Cell::new(false)),
        }
    }
    pub fn set_enabled(&self, enabled: bool) {
        let target = if enabled { 1.0 } else { 0.0 };
        self.target.set(target);
        if !self.widget.settings().is_gtk_enable_animations() {
            self.progress.set(target);
            self.widget.queue_draw();
            return;
        }
        if self.running.replace(true) {
            return;
        }
        let progress = self.progress.clone();
        let target = self.target.clone();
        let running = self.running.clone();
        let last = Cell::new(None);
        self.widget.add_tick_callback(move |widget, clock| {
            let now = clock.frame_time();
            let elapsed = last
                .replace(Some(now))
                .map_or(0.0, |previous| (now - previous) as f64 / 280_000.0);
            let next = progress.get() + (target.get() - progress.get()).clamp(-elapsed, elapsed);
            progress.set(next);
            widget.queue_draw();
            if (next - target.get()).abs() < f64::EPSILON {
                running.set(false);
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    }
}
