use crate::{config::Config, notification::Notification};
use ::rofi::Rofi;
use std::sync::Arc;
use tera::Tera;

const NO_NOTIFICATIONS_MENU: [&str; 1] = ["No notifications"];

pub struct Menu {
    open: bool,
    config: Arc<Config>,
    template: Tera,
}

impl Menu {
    pub fn init(config: Arc<Config>, template: Tera) -> Self {
        Self {
            config,
            open: false,
            template,
        }
    }

    pub fn list(&mut self, notifications: &Vec<Notification>) -> Option<u32> {
        if self.open {
            return None;
        }

        self.open = true;
        if notifications.is_empty() {
            let _no_notifications = Rofi::new(&NO_NOTIFICATIONS_MENU).run_index();
            self.open = false;

            return None;
        }

        let notification_messages: Vec<String> = notifications
            .iter()
            .map(|n| {
                let urgency_config = self.config.get_urgency_config(&n.urgency);
                let render = n.render_message(&self.template, urgency_config.text, 0);
                match render {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("error rendering notification: {}", e);
                        "Failed to render notification".to_string()
                    }
                }
            })
            .collect();

        self.open = false;

        match Rofi::new(&notification_messages).run_index() {
            Ok(element) => Some(notifications[element].id),
            _ => None,
        }
    }
}
