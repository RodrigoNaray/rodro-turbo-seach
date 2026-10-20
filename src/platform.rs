//! Linux integration: GApplication owns the session-bus name and forwards commands.
use std::{cell::RefCell, rc::Rc};

use gtk::{gio, glib, prelude::*};

use crate::{cli::Command, ui::SearchWindow};

const APPLICATION_ID: &str = "io.github.rodro.TurboSearch";

/// Reveal a result in the file manager instead of launching its MIME handler.
pub async fn show_in_folder(
    path: &std::path::Path,
    context: &(impl IsA<gio::AppLaunchContext> + Clone + 'static),
) -> Result<(), glib::Error> {
    let file = gio::File::for_path(path);
    if let Ok(connection) = gio::bus_get_future(gio::BusType::Session).await {
        let parameters = (vec![file.uri().to_string()], "").to_variant();
        if connection
            .call_future(
                Some("org.freedesktop.FileManager1"),
                "/org/freedesktop/FileManager1",
                "org.freedesktop.FileManager1",
                "ShowItems",
                Some(&parameters),
                Some(glib::VariantTy::UNIT),
                gio::DBusCallFlags::NONE,
                3_000,
            )
            .await
            .is_ok()
        {
            return Ok(());
        }
    }

    // A manager without FileManager1 support can still open the parent folder.
    // Never pass the selected file to its MIME handler, even on this fallback.
    let folder = file.parent().unwrap_or(file);
    gio::AppInfo::launch_default_for_uri_future(&folder.uri(), Some(context)).await
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires dbus-run-session and TURBO_SEARCH_ISOLATED_BUS=1"]
    fn reveals_file_via_file_manager() {
        // Never claim the real desktop's file manager name during a test.
        assert_eq!(
            std::env::var("TURBO_SEARCH_ISOLATED_BUS").as_deref(),
            Ok("1")
        );
        let main_context = glib::MainContext::new();
        main_context
            .with_thread_default(|| {
                main_context.block_on(async {
                    let connection = gio::bus_get_future(gio::BusType::Session).await.unwrap();
                    let reply = connection
                        .call_future(
                            Some("org.freedesktop.DBus"),
                            "/org/freedesktop/DBus",
                            "org.freedesktop.DBus",
                            "RequestName",
                            Some(&("org.freedesktop.FileManager1", 4u32).to_variant()),
                            None,
                            gio::DBusCallFlags::NONE,
                            3_000,
                        )
                        .await
                        .unwrap();
                    assert_eq!(reply.get::<(u32,)>(), Some((1,)));
                    let interface = gio::DBusNodeInfo::for_xml(
                        "<node><interface name='org.freedesktop.FileManager1'>
                         <method name='ShowItems'><arg type='as' direction='in'/>
                         <arg type='s' direction='in'/></method></interface></node>",
                    )
                    .unwrap()
                    .lookup_interface("org.freedesktop.FileManager1")
                    .unwrap();
                    let received = Rc::new(RefCell::new(Vec::new()));
                    let calls = received.clone();
                    let registration = connection
                        .register_object("/org/freedesktop/FileManager1", &interface)
                        .method_call(move |_, _, _, _, method, parameters, invocation| {
                            assert_eq!(method, "ShowItems");
                            calls
                                .borrow_mut()
                                .push(parameters.get::<(Vec<String>, String)>().unwrap());
                            invocation.return_value(Some(&().to_variant()));
                        })
                        .build()
                        .unwrap();
                    let launch_context = gio::AppLaunchContext::new();
                    for path in ["/tmp/informe final #1.txt", "/tmp/carpeta con ñ"] {
                        show_in_folder(std::path::Path::new(path), &launch_context)
                            .await
                            .unwrap();
                    }
                    assert_eq!(
                        *received.borrow(),
                        [
                            (
                                vec!["file:///tmp/informe%20final%20%231.txt".into()],
                                String::new()
                            ),
                            (
                                vec!["file:///tmp/carpeta%20con%20%C3%B1".into()],
                                String::new()
                            ),
                        ]
                    );
                    connection.unregister_object(registration).unwrap();
                });
            })
            .unwrap();
    }
}
