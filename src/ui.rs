//! Widgets and keyboard interaction; search and discovery live in separate modules.
use crate::{calculator, catalog::Catalog, search};
use gtk::{gdk, glib, prelude::*};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct SearchWindow(Rc<View>);

struct View {
    window: gtk::ApplicationWindow,
    entry: gtk::Entry,
    list: gtk::ListBox,
    status: gtk::Label,
    calculator: gtk::Button,
    calculation_label: gtk::Label,
    calculation: RefCell<Option<String>>,
    catalog: RefCell<Catalog>,
    results: RefCell<Vec<usize>>,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let entry = gtk::Entry::builder()
            .placeholder_text("Buscar aplicaciones o calcular…")
            .hexpand(true)
            .build();
        // Embed the SVG so the search bar also works before installation.
        let logo = gtk::gio::BytesIcon::new(&glib::Bytes::from_static(include_bytes!(
            "../assets/rodro-turbo-search-v2-rayo.svg"
        )));
        entry.set_icon_from_gicon(gtk::EntryIconPosition::Primary, Some(&logo));
        entry.add_css_class("search-entry");
        let list = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .activate_on_single_click(true)
            .build();
        list.add_css_class("search-results");
        let status = gtk::Label::builder().wrap(true).xalign(0.0).build();
        status.add_css_class("dim-label");
        let calculation_label = gtk::Label::builder()
            .xalign(0.0)
            .hexpand(true)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();
        calculation_label.add_css_class("calculation-value");
        let calculation_content = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        calculation_content.append(&gtk::Image::from_icon_name(
            "accessories-calculator-symbolic",
        ));
        calculation_content.append(&calculation_label);
        calculation_content.append(&gtk::Image::from_icon_name("edit-copy-symbolic"));
        let calculator = gtk::Button::builder()
            .child(&calculation_content)
            .tooltip_text("Copiar resultado")
            .build();
        calculator.add_css_class("calculation");
        let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
        for set_margin in [
            WidgetExt::set_margin_top,
            WidgetExt::set_margin_bottom,
            WidgetExt::set_margin_start,
            WidgetExt::set_margin_end,
        ] {
            set_margin(&content, 16);
        }
        content.append(&entry);
        content.append(&calculator);
        content.append(&list);
        content.append(&status);
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Rodro Turbo Search")
            .icon_name("io.github.rodro.TurboSearch")
            .default_width(600)
            .resizable(false)
            .decorated(false)
            .hide_on_close(true)
            .child(&content)
            .build();
        window.add_css_class("turbo-search");
        // Wait for actual activation before dismissing: mapping on Wayland is asynchronous.
        let was_active = Cell::new(false);
        window.connect_is_active_notify(move |window| {
            let active = window.is_active();
            if was_active.replace(active) && !active {
                window.set_visible(false);
            }
        });
        let css = gtk::CssProvider::new();
        css.load_from_data(include_str!("style.css"));
        gtk::style_context_add_provider_for_display(
            &WidgetExt::display(&window),
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let view = Rc::new(View {
            window,
            entry,
            list,
            status,
            calculator,
            calculation_label,
            calculation: RefCell::new(None),
            catalog: RefCell::new(Catalog::discover()),
            results: RefCell::new(Vec::new()),
        });
        let weak = Rc::downgrade(&view);
        view.entry.connect_changed(move |_| {
            if let Some(view) = weak.upgrade() {
                view.render();
            }
        });
        let weak = Rc::downgrade(&view);
        view.entry.connect_activate(move |_| {
            if let Some(view) = weak.upgrade() {
                view.activate_selected();
            }
        });
        let weak = Rc::downgrade(&view);
        view.list.connect_row_activated(move |list, row| {
            list.select_row(Some(row));
            if let Some(view) = weak.upgrade() {
                view.activate_selected();
            }
        });
        let weak = Rc::downgrade(&view);
        view.calculator.connect_clicked(move |_| {
            if let Some(view) = weak.upgrade() {
                view.activate_selected();
            }
        });
        let keys = gtk::EventControllerKey::new();
        keys.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = Rc::downgrade(&view);
        keys.connect_key_pressed(move |_, key, _, _| {
            let Some(view) = weak.upgrade() else {
                return glib::Propagation::Proceed;
            };
            match key {
                gdk::Key::Escape => view.window.set_visible(false),
                gdk::Key::Down => view.move_selection(1),
                gdk::Key::Up => view.move_selection(-1),
                _ => return glib::Propagation::Proceed,
            }
            glib::Propagation::Stop
        });
        view.window.add_controller(keys);
        view.render();
        Self(view)
    }

    pub fn toggle(&self, activation_token: Option<&str>) {
        let view = &self.0;
        if view.window.is_visible() {
            view.window.set_visible(false);
        } else {
            // Refresh on opening so installations appear without a filesystem watcher.
            view.refresh_catalog(Catalog::discover());
            if let Some(token) = activation_token {
                view.window.set_startup_id(token);
            }
            view.window.present();
            view.entry.grab_focus_without_selecting();
        }
    }
}

impl View {
    fn refresh_catalog(&self, catalog: Catalog) {
        // Preserve identity, not the old index: installations can reorder the catalog.
        let selected_id = self.list.selected_row().and_then(|row| {
            self.results
                .borrow()
                .get(row.index() as usize)
                .map(|&index| self.catalog.borrow().entries[index].id.clone())
        });
        *self.catalog.borrow_mut() = catalog;
        self.render();
        if let Some(id) = selected_id {
            let catalog = self.catalog.borrow();
            if let Some(position) = self
                .results
                .borrow()
                .iter()
                .position(|&index| catalog.entries[index].id == id)
            {
                self.list
                    .select_row(self.list.row_at_index(position as i32).as_ref());
            }
        }
    }

    fn render(&self) {
        while let Some(child) = self.list.first_child() {
            self.list.remove(&child);
        }
        self.results.borrow_mut().clear();
        *self.calculation.borrow_mut() = None;
        self.calculator.set_visible(false);
        let query = self.entry.text();
        let searching = !query.trim().is_empty();
        self.status.set_visible(searching);
        match calculator::evaluate(&query) {
            calculator::Evaluation::NotExpression => {}
            evaluation => {
                self.list.set_visible(false);
                match evaluation {
                    calculator::Evaluation::Result(value) => {
                        self.calculation_label.set_text(&format!("= {value}"));
                        self.calculator.set_visible(true);
                        self.status.set_text("Enter o clic para copiar y ocultar");
                        *self.calculation.borrow_mut() = Some(value);
                    }
                    calculator::Evaluation::Incomplete => {
                        self.status.set_text("Completá la operación…")
                    }
                    calculator::Evaluation::Error(message) => self.status.set_text(message),
                    calculator::Evaluation::NotExpression => unreachable!(),
                }
                return;
            }
        }
        let catalog = self.catalog.borrow();
        let results = search::search(&catalog.entries, &query, 5);
        self.list.set_visible(!results.is_empty());
        for &index in &results {
            let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
            row.add_css_class("app-result");
            let icon = catalog.apps[index]
                .icon()
                .map(|icon| gtk::Image::from_gicon(&icon))
                .unwrap_or_else(|| gtk::Image::from_icon_name("application-x-executable-symbolic"));
            icon.set_pixel_size(32);
            let label = gtk::Label::builder()
                .label(&catalog.entries[index].name)
                .xalign(0.0)
                .hexpand(true)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .build();
            row.append(&icon);
            row.append(&label);
            self.list.append(&row);
        }
        self.status.set_text(if results.is_empty() {
            "No se encontraron aplicaciones"
        } else {
            "↑ ↓ Seleccionar   ·   Enter Abrir   ·   Escape Cerrar"
        });
        *self.results.borrow_mut() = results;
        self.list.select_row(self.list.row_at_index(0).as_ref());
    }

    fn move_selection(&self, delta: i32) {
        let count = self.results.borrow().len() as i32;
        if count == 0 {
            return;
        }
        let current = self.list.selected_row().map_or(0, |row| row.index());
        let index = (current + delta).clamp(0, count - 1);
        self.list.select_row(self.list.row_at_index(index).as_ref());
        self.entry.grab_focus();
    }

    fn activate_selected(&self) {
        if let Some(value) = self.calculation.borrow().as_ref() {
            WidgetExt::display(&self.window).clipboard().set_text(value);
            self.window.set_visible(false);
            return;
        }
        let Some(row) = self.list.selected_row() else {
            return;
        };
        let results = self.results.borrow();
        let Some(&index) = results.get(row.index() as usize) else {
            return;
        };
        let catalog = self.catalog.borrow();
        let context = WidgetExt::display(&self.window).app_launch_context();
        match catalog.apps[index].launch(&[], Some(&context)) {
            Ok(()) => self.window.set_visible(false),
            Err(error) => self
                .status
                .set_text(&format!("No se pudo abrir la aplicación: {error}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtk::gio;

    // Explicit opt-in: this test needs a desktop session and briefly maps a window.
    #[test]
    #[ignore = "requires a graphical session"]
    fn search_selection_launch_and_failure() {
        gtk::init().expect("GTK session");
        let app = gtk::Application::builder()
            .application_id("io.github.rodro.TurboSearch.UiTest")
            .build();
        app.register(None::<&gio::Cancellable>)
            .expect("register test application");
        let window = SearchWindow::new(&app);
        let view = &window.0;
        let success = gio::AppInfo::create_from_commandline(
            "/usr/bin/true",
            Some("Code"),
            gio::AppInfoCreateFlags::NONE,
        )
        .unwrap();
        let failure = gio::AppInfo::create_from_commandline(
            "/rodro-turbo-search-test-missing-executable",
            Some("Code Broken"),
            gio::AppInfoCreateFlags::NONE,
        )
        .unwrap();
        *view.catalog.borrow_mut() = Catalog {
            entries: vec![
                search::SearchEntry::new("code".into(), "Code".into()),
                search::SearchEntry::new("broken".into(), "Code Broken".into()),
            ],
            apps: vec![success.clone(), failure.clone()],
        };
        assert!(view.results.borrow().is_empty());
        assert!(!view.list.get_visible());
        assert!(!view.status.get_visible());
        view.entry.set_text("code");
        assert!(view.list.get_visible());
        assert_eq!(*view.results.borrow(), [0, 1]);
        assert_eq!(view.list.selected_row().unwrap().index(), 0);
        view.window.present();
        view.move_selection(1);
        view.entry.set_position(2);
        view.refresh_catalog(Catalog {
            entries: vec![
                search::SearchEntry::new("broken".into(), "Code Broken".into()),
                search::SearchEntry::new("code".into(), "Code".into()),
            ],
            apps: vec![failure, success],
        });
        assert_eq!(view.entry.text(), "code");
        assert_eq!(view.entry.position(), 2);
        assert_eq!(view.list.selected_row().unwrap().index(), 1);
        assert_eq!(*view.results.borrow(), [1, 0]);
        view.activate_selected();
        assert!(view.window.is_visible());
        assert!(view.status.text().starts_with("No se pudo abrir"));
        view.move_selection(-1);
        view.activate_selected();
        assert!(!view.window.is_visible());
        view.entry.set_text("no matching application");
        assert!(view.results.borrow().is_empty());
        view.move_selection(1);
        view.activate_selected();
        assert!(view.list.selected_row().is_none());
        view.entry.set_text("125 * 1.22");
        assert_eq!(view.calculation.borrow().as_deref(), Some("152.5"));
        assert_eq!(view.calculation_label.text(), "= 152.5");
        assert!(view.calculator.get_visible());
        assert!(!view.list.get_visible());
        assert!(view.results.borrow().is_empty());
        view.entry.set_text("125 *");
        assert!(view.calculation.borrow().is_none());
        assert!(!view.calculator.get_visible());
        view.entry.set_text("1/0");
        assert!(view.status.text().contains("cero"));
        view.activate_selected(); // Invalid expressions must never launch an app.
        view.entry.set_text("code");
        assert!(!view.calculator.get_visible());
        assert!(view.calculation.borrow().is_none());
        assert!(!view.results.borrow().is_empty());
        view.entry.set_text("   ");
        assert!(view.results.borrow().is_empty());
        assert!(!view.list.get_visible());
        assert!(!view.status.get_visible());
        window.toggle(None);
        assert_eq!(view.entry.text(), "   ");
        assert!(!view.list.get_visible());
        assert!(!view.status.get_visible());
        window.toggle(None);
        assert!(!view.window.is_visible());
        view.window.close();
    }
}
