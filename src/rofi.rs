use crate::{config::Config, notification::Notification};
use ::rofi::Rofi;
use std::sync::{Arc, RwLock};
use tera::Tera;

const NO_NOTIFICATIONS_MENU: [&str; 1] = ["No notifications"];

pub struct Menu {
    open: Arc<RwLock<()>>,
    config: Arc<Config>,
    template: Arc<Tera>,
}

impl Clone for Menu {
    fn clone(&self) -> Self {
        Self {
            open: Arc::clone(&self.open),
            config: self.config.clone(),
            template: Arc::clone(&self.template),
        }
    }
}

impl Menu {
    pub fn init(config: Arc<Config>, template: Tera) -> Self {
        Self {
            config,
            open: Arc::new(RwLock::new(())),
            template: Arc::new(template),
        }
    }

    pub fn list(&self, notifications: &Vec<Notification>) -> Option<u32> {
        let open = self.open.try_write();

        if let Err(e) = open {
            tracing::warn!("failed to acquire lock. Rofi may already be open: {}", e);
            return None;
        };

        if notifications.is_empty() {
            let _no_notifications = Rofi::new(&NO_NOTIFICATIONS_MENU).run_index();

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

        let element = if let Ok(element) = Rofi::new(&notification_messages).run_index() {
            element
        } else {
            return None;
        };

        let id = notifications.get(element).map(|n| n.id);

        if id.is_none() {
            tracing::error!("failed to get notification id from notifications list at element: {}. notifications: {:#?}", element, notifications);
        }

        id
    }
}
