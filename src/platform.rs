//! Linux integration: GApplication owns the session-bus name and forwards commands.
use std::{cell::RefCell, rc::Rc};

use gtk::{gio, glib, prelude::*};

use crate::{cli::Command, ui::SearchWindow};

const APPLICATION_ID: &str = "io.github.rodro.TurboSearch";

pub fn run() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id(APPLICATION_ID)
        .flags(
            gio::ApplicationFlags::HANDLES_COMMAND_LINE | gio::ApplicationFlags::SEND_ENVIRONMENT,
        )
        .build();

    let window: Rc<RefCell<Option<SearchWindow>>> = Rc::default();
    let hold = Rc::new(RefCell::new(None));

    app.connect_command_line({
        let window = window.clone();
        let hold = hold.clone();
        move |app, invocation| {
            let command = match Command::parse(invocation.arguments().into_iter().skip(1)) {
                Ok(command) => command,
                Err(message) => {
                    eprintln!("{message}");
                    return glib::ExitCode::from(2);
                }
            };
            if command == Command::Quit {
                app.quit();
                return glib::ExitCode::SUCCESS;
            }

            // Keep the event loop alive while the only window is hidden.
            if hold.borrow().is_none() {
                *hold.borrow_mut() = Some(app.hold());
            }
            let mut window = window.borrow_mut();
            let window = window.get_or_insert_with(|| SearchWindow::new(app));
            if command == Command::Toggle {
                // Use the invoking process's token, never a stale token from startup.
                // GNOME may omit it: present() is then only a best-effort request.
                let token = invocation
                    .getenv("XDG_ACTIVATION_TOKEN")
                    .or_else(|| invocation.getenv("DESKTOP_STARTUP_ID"));
                window.toggle(token.as_deref());
            }
            glib::ExitCode::SUCCESS
        }
    });

    let result = app.run();
    // Break the lifetime chain: widgets and hold guard reference the application.
    window.borrow_mut().take();
    hold.borrow_mut().take();
    result
}
