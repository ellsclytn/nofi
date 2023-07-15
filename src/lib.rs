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

/// Rofi handler
pub mod rofi;

use crate::config::Config;
use crate::dbus::DbusServer;
use crate::error::Result;
use crate::notification::{Action, NOTIFICATION_MESSAGE_TEMPLATE};
use notification::Manager;
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
    let config_cloned = Arc::clone(&config);
    let menu = Arc::new(rofi::Menu::init(config_cloned, template));

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
                tracing::debug!("listing notifications");
                let notifications = notifications.clone();
                let all_notifications = notifications.all_unread();
                let menu_cloned = menu.clone();
                let unix_socket = unix_socket.clone();

                thread::spawn(move || {
                    let selected_notification = menu_cloned.list(&all_notifications);

                    if let Some(id) = selected_notification {
                        notifications.mark_as_read(id);
                        unix_socket.update(notifications.counts());
                    }
                });
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
