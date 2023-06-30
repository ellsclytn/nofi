//! A dead simple notification daemon.

#![warn(missing_docs, clippy::unwrap_used)]

/// Error handler.
pub mod error;

/// D-Bus handler.
pub mod dbus;

/// Configuration.
pub mod config;

/// Notification manager.
pub mod notification;

use crate::config::Config;
use crate::dbus::{DbusClient, DbusServer};
use crate::error::Result;
use crate::notification::{Action, NOTIFICATION_MESSAGE_TEMPLATE};
use estimated_read_time::Options;
use notification::Manager;
use rofi;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tera::Tera;
use tracing_subscriber::EnvFilter;

/// Runs `nofi`.
pub fn run() -> Result<()> {
    let config = Arc::new(Config::parse()?);

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
    let dbus_client = Arc::new(DbusClient::init()?);
    let timeout = Duration::from_millis(1000);

    let notifications = Manager::init();
    let config_cloned = Arc::clone(&config);
    let mut template = Tera::default();
    template.add_raw_template(
        NOTIFICATION_MESSAGE_TEMPLATE,
        &config_cloned.global.template,
    )?;

    let dbus_client_cloned = Arc::clone(&dbus_client);
    let config_cloned = Arc::clone(&config);
    let notifications_cloned = notifications.clone();

    // thread::spawn(move || {
    //     if let Err(e) = x11_cloned.handle_events(
    //         window_cloned,
    //         notifications_cloned,
    //         config_cloned,
    //         |notification| {
    //             tracing::debug!("user input detected");
    //             dbus_client_cloned
    //                 .close_notification(notification.id, timeout)
    //                 .expect("failed to close notification");
    //         },
    //     ) {
    //         eprintln!("Failed to handle X11 events: {e}")
    //     }
    // });

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
                let timeout = notification.expire_timeout.unwrap_or_else(|| {
                    let urgency_config = config.get_urgency_config(&notification.urgency);
                    Duration::from_secs(if urgency_config.auto_clear.unwrap_or(false) {
                        notification
                            .render_message(&template, urgency_config.text, 0)
                            .map(|v| estimated_read_time::text(&v, &Options::default()).seconds())
                            .unwrap_or_default()
                    } else {
                        urgency_config.timeout.into()
                    })
                });
                if !timeout.is_zero() {
                    tracing::debug!("notification timeout: {}ms", timeout.as_millis());
                    let dbus_client_cloned = Arc::clone(&dbus_client);
                    let notifications_cloned = notifications.clone();
                    // thread::spawn(move || {
                    //     thread::sleep(timeout);
                    //     if notifications_cloned.is_unread(notification.id) {
                    //         dbus_client_cloned
                    //             .close_notification(notification.id, timeout)
                    //             .expect("failed to close notification");
                    //     }
                    // });
                }
                notifications.add(notification);
            }
            Action::ShowLast => {
                tracing::debug!("showing the last notification");
                let notifications = notifications.all_unread();
                if notifications.len() == 0 {
                    let notification = ["No notifications".to_owned()];
                    let _no_notifications = rofi::Rofi::new(&notification).run_index();
                } else {
                    let notifications: Vec<String> = notifications
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
                    match rofi::Rofi::new(&notifications).run_index() {
                        Ok(element) => println!("Choice: {:#?}", notifications[element]),
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
                // x11_cloned.hide_window(&window)?;
                if notifications.get_unread_count() >= 1 {
                    //     x11_cloned.show_window(&window)?;
                }
            }
            Action::CloseAll => {
                tracing::debug!("closing all notifications");
                notifications.mark_all_as_read();
                // x11_cloned.hide_window(&window)?;
            }
        }
    }
}
