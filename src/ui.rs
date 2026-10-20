//! Widgets and keyboard interaction; search and discovery live in separate modules.
use crate::{background::Background, calculator, catalog::Catalog, files::FileSearch, search};
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
    file_results: RefCell<Vec<std::path::PathBuf>>,
    files_enabled: Cell<bool>,
    file_search: FileSearch,
    file_timer: RefCell<Option<glib::SourceId>>,
    background: Background,
    mode_button: gtk::ToggleButton,
}

impl SearchWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let entry = gtk::Entry::builder().hexpand(true).build();
        // Embed the SVG so the search bar also works before installation.
        let logo = gtk::gio::BytesIcon::new(&glib::Bytes::from_static(include_bytes!(
            "../assets/rodro-turbo-search-v2-rayo.svg"
        )));
        let mode_button = gtk::ToggleButton::builder()
            .child(&gtk::Image::from_gicon(&logo))
            .tooltip_text("Incluir archivos de tu carpeta personal")
            .build();
        mode_button.add_css_class("mode-button");
        mode_button.set_cursor_from_name(Some("pointer"));
        let search_bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        search_bar.append(&mode_button);
        search_bar.append(&entry);
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
        content.append(&search_bar);
        content.append(&calculator);
        content.append(&list);
        content.append(&status);
        let background = Background::new();
        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&background.widget));
        overlay.add_overlay(&content);
        overlay.set_measure_overlay(&content, true);
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Rodro Turbo Search")
            .icon_name("io.github.rodro.TurboSearch")
            .default_width(600)
            .resizable(false)
            .decorated(false)
            .hide_on_close(true)
            .child(&overlay)
            .build();
        window.add_css_class("turbo-search");
        let interaction = Rc::new(crate::dismissal::Interaction::default());
        crate::window_drag::install(&window, &mode_button, interaction.clone());
        crate::dismissal::install(&window, interaction);
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
            file_results: RefCell::new(Vec::new()),
            files_enabled: Cell::new(false),
            file_search: FileSearch::new(),
            file_timer: RefCell::new(None),
            background,
            mode_button,
        });
        let weak = Rc::downgrade(&view);
        view.window.connect_visible_notify(move |window| {
            if !window.is_visible()
                && let Some(view) = weak.upgrade()
            {
                view.cancel_files();
            }
        });
        let weak = Rc::downgrade(&view);
        view.mode_button.connect_toggled(move |button| {
            if let Some(view) = weak.upgrade() {
                let enabled = button.is_active();
                view.files_enabled.set(enabled);
                view.background.set_enabled(enabled);
                button.set_tooltip_text(Some(if enabled {
                    "Volver a buscar solo aplicaciones"
                } else {
                    "Incluir archivos de tu carpeta personal"
                }));
                view.render();
                view.entry.grab_focus_without_selecting();
            }
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
    fn refresh_catalog(self: &Rc<Self>, catalog: Catalog) {
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

    fn render(self: &Rc<Self>) {
        self.cancel_files();
        self.file_results.borrow_mut().clear();
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
        if self.files_enabled.get() && searching {
            if query.trim().chars().count() < 2 {
                self.status
                    .set_text("Escribí al menos 2 caracteres para buscar archivos");
            } else {
                self.schedule_files(query.to_string());
            }
        }
    }

    fn cancel_files(&self) {
        if let Some(timer) = self.file_timer.borrow_mut().take() {
            timer.remove();
        }
        self.file_search.cancel();
    }

    fn schedule_files(self: &Rc<Self>, query: String) {
        self.status
            .set_text("Buscando archivos en tu carpeta personal…");
        let weak = Rc::downgrade(self);
        let timer =
            glib::timeout_add_local_once(std::time::Duration::from_millis(200), move || {
                let Some(view) = weak.upgrade() else {
                    return;
                };
                view.file_timer.borrow_mut().take();
                let generation = view.file_search.request(glib::home_dir(), query);
                let weak = Rc::downgrade(&view);
                let timer =
                    glib::timeout_add_local(std::time::Duration::from_millis(40), move || {
                        let Some(view) = weak.upgrade() else {
                            return glib::ControlFlow::Break;
                        };
                        let Some(matches) = view.file_search.take(generation) else {
                            return glib::ControlFlow::Continue;
                        };
                        view.file_timer.borrow_mut().take();
                        for path in &matches.paths {
                            let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
                            row.add_css_class("app-result");
                            row.append(&gtk::Image::from_icon_name("text-x-generic-symbolic"));
                            let labels = gtk::Box::new(gtk::Orientation::Vertical, 2);
                            let name = path.file_name().unwrap_or_default().to_string_lossy();
                            let title = gtk::Label::builder()
                                .label(name.as_ref())
                                .xalign(0.0)
                                .ellipsize(gtk::pango::EllipsizeMode::End)
                                .hexpand(true)
                                .build();
                            let location = path.strip_prefix(glib::home_dir()).unwrap_or(path);
                            let subtitle = gtk::Label::builder()
                                .label(format!("~/{}", location.display()))
                                .xalign(0.0)
                                .ellipsize(gtk::pango::EllipsizeMode::Middle)
                                .build();
                            subtitle.add_css_class("file-path");
                            labels.append(&title);
                            labels.append(&subtitle);
                            row.append(&labels);
                            row.set_tooltip_text(Some(&path.to_string_lossy()));
                            view.list.append(&row);
                        }
                        *view.file_results.borrow_mut() = matches.paths;
                        let any = !view.results.borrow().is_empty()
                            || !view.file_results.borrow().is_empty();
                        view.list.set_visible(any);
                        if view.list.selected_row().is_none() {
                            view.list.select_row(view.list.row_at_index(0).as_ref());
                        }
                        view.status.set_text(if matches.limited {
                            "Búsqueda parcial: se alcanzó el límite de recorrido"
                        } else if any {
                            "↑ ↓ Seleccionar   ·   Enter Abrir   ·   Escape Cerrar"
                        } else {
                            "No se encontraron aplicaciones ni archivos"
                        });
                        glib::ControlFlow::Break
                    });
                *view.file_timer.borrow_mut() = Some(timer);
            });
        *self.file_timer.borrow_mut() = Some(timer);
    }

    fn move_selection(&self, delta: i32) {
        let count = (self.results.borrow().len() + self.file_results.borrow().len()) as i32;
        if count == 0 {
            return;
        }
        let current = self.list.selected_row().map_or(0, |row| row.index());
        let index = (current + delta).clamp(0, count - 1);
        self.list.select_row(self.list.row_at_index(index).as_ref());
        self.entry.grab_focus();
    }

    fn activate_selected(&self) {
        // Clearing the entry synchronously renders again; release state borrows first.
        let calculation = self.calculation.borrow().clone();
        if let Some(value) = calculation {
            WidgetExt::display(&self.window)
                .clipboard()
                .set_text(&value);
            self.hide_and_clear();
            return;
        }
        let Some(row) = self.list.selected_row() else {
            return;
        };
        let position = row.index() as usize;
        let app_count = self.results.borrow().len();
        if position >= app_count {
            let path = self
                .file_results
                .borrow()
                .get(position - app_count)
                .cloned();
            if let Some(path) = path {
                let file = gtk::gio::File::for_path(path);
                let context = WidgetExt::display(&self.window).app_launch_context();
                match gtk::gio::AppInfo::launch_default_for_uri(&file.uri(), Some(&context)) {
                    Ok(()) => self.hide_and_clear(),
                    Err(error) => self
                        .status
                        .set_text(&format!("No se pudo abrir el archivo: {error}")),
                }
            }
            return;
        }
        let index = self.results.borrow().get(position).copied();
        let Some(index) = index else {
            return;
        };
        let context = WidgetExt::display(&self.window).app_launch_context();
        let outcome = self.catalog.borrow().apps[index].launch(&[], Some(&context));
        match outcome {
            Ok(()) => self.hide_and_clear(),
            Err(error) => self
                .status
                .set_text(&format!("No se pudo abrir la aplicación: {error}")),
        }
    }

    fn hide_and_clear(&self) {
        self.window.set_visible(false);
        self.entry.set_text("");
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
        assert!(view.entry.text().is_empty());
        assert!(view.results.borrow().is_empty());
        assert!(!view.list.get_visible());
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
        // The logo switches modes without losing text; disabling/hiding cancels work.
        view.entry.set_text("code");
        view.mode_button.set_active(true);
        assert!(view.files_enabled.get());
        assert!(view.file_timer.borrow().is_some());
        assert_eq!(view.entry.text(), "code");
        view.mode_button.set_active(false);
        assert!(!view.files_enabled.get());
        assert!(view.file_timer.borrow().is_none());
        assert!(view.file_results.borrow().is_empty());
        view.mode_button.set_active(true);
        view.window.set_visible(true);
        view.window.set_visible(false);
        assert!(view.file_timer.borrow().is_none());
        view.entry.set_text("a");
        assert!(view.file_timer.borrow().is_none());
        view.entry.set_text("");
        assert!(!view.list.get_visible());
        assert!(!view.status.get_visible());
        view.window.close();
    }
}
