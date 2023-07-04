//! A dead simple notification daemon.

/// Error handler.
pub mod error;

/// D-Bus handler.
pub mod dbus;

/// Configuration.
pub mod config;

/// Notification manager.
pub mod notification;

/// Socket handler.
pub mod socket;

use crate::config::Config;
use crate::dbus::DbusServer;
use crate::error::Result;
use crate::notification::{Action, NOTIFICATION_MESSAGE_TEMPLATE};
use notification::Manager;
use rofi::Rofi;
use socket::Socket;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tera::Tera;
use tracing_subscriber::EnvFilter;
use xdg::BaseDirectories;

/// Runs `nofi`.
pub fn run() -> Result<()> {
    let xdg_dirs = BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?;
    let config = Arc::new(Config::parse(&xdg_dirs)?);

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(config.global.log_verbosity.into())
                .from_env_lossy(),
        )
        .init();
    tracing::trace!("{:#?}", config);
    tracing::info!("starting");

    let dbus_server = DbusServer::init()?;
    let timeout = Duration::from_millis(1000);

    let unix_socket = Socket::init(&xdg_dirs)?;
    let notifications = Manager::init();
    let unix_socket = Arc::new(unix_socket);
    let config_cloned = Arc::clone(&config);
    let mut template = Tera::default();
    template.add_raw_template(
        NOTIFICATION_MESSAGE_TEMPLATE,
        &config_cloned.global.template,
    )?;

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        tracing::debug!("registering D-Bus handler");
        dbus_server
            .register_notification_handler(sender, timeout)
            .expect("failed to register D-Bus notification handler");
    });

    loop {
        match receiver.recv()? {
            Action::Show(notification) => {
                tracing::debug!("received notification: {}", notification.id);
                notifications.add(notification);
                unix_socket.update(notifications.counts());
            }
            Action::ShowLast => {
                tracing::debug!("showing the last notification");
                let all_notifications = notifications.all_unread();
                if all_notifications.is_empty() {
                    let notification = ["No notifications".to_owned()];
                    let _no_notifications = Rofi::new(&notification).run_index();
                } else {
                    let notification_messages: Vec<String> = all_notifications
                        .iter()
                        .map(|n| {
                            let urgency_config = config.get_urgency_config(&n.urgency);
                            let render = n.render_message(&template, urgency_config.text, 0);
                            match render {
                                Ok(v) => v,
                                Err(e) => {
                                    tracing::error!("error rendering notification: {}", e);
                                    "Failed to render notification".to_string()
                                }
                            }
                        })
                        .collect();
                    match Rofi::new(&notification_messages).run_index() {
                        Ok(element) => {
                            notifications.mark_as_read(all_notifications[element].id);

                            let counts = notifications.counts();
                            let socket_count = unix_socket.count.clone();
                            let mut socket_count =
                                socket_count.write().expect("failed to write to socket");
                            *socket_count = counts;
                        }
                        Err(rofi::Error::Interrupted) => println!("Interrupted"),
                        Err(rofi::Error::NotFound) => println!("User input was not found"),
                        Err(e) => println!("Error: {}", e),
                    };
                };
            }
            Action::Close(id) => {
                if let Some(id) = id {
                    tracing::debug!("closing notification: {}", id);
                    notifications.mark_as_read(id);
                } else {
                    tracing::debug!("closing the last notification");
                    notifications.mark_last_as_read();
                }
                unix_socket.update(notifications.counts());
            }
            Action::CloseAll => {
                tracing::debug!("closing all notifications");
                notifications.mark_all_as_read();
                unix_socket.update(notifications.counts());
            }
        }
    }
}
