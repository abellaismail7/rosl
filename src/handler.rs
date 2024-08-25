use crate::{actions::NotificationAction, errors::RoslError, notification::Notification};
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use zbus::{interface, zvariant::Value};

pub struct NotificationHandler {
    count: u32,
    sender: Sender<NotificationAction>,
}

impl NotificationHandler {
    pub fn new(sender: Sender<NotificationAction>) -> Self {
        NotificationHandler { count: 0, sender }
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationHandler {
    #[dbus_interface(name = "CloseNotification")]
    async fn close_notification(&mut self, notification_id: u32) -> zbus::fdo::Result<()> {
        self.sender
            .send(NotificationAction::Close { notification_id })
            .await
            .map_err(RoslError::from)?;
        Ok(())
    }

    #[dbus_interface(name = "Notify")]
    async fn notify(
        &mut self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: HashMap<String, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        let notification_id = if replaces_id == 0 {
            self.count += 1;
            self.count
        } else {
            replaces_id
        };

        let desktop_entry = if hints.contains_key("desktop-entry") {
            hints["desktop-entry"].to_string()
        } else {
            String::new()
        };

        let notification = Notification {
            app_name,
            replaces_id,
            app_icon,
            summary,
            body,
            actions,
            expire_timeout,
            notification_id,
            desktop_entry,
        };

        self.sender
            .send(NotificationAction::Notify { notification })
            .await
            .map_err(RoslError::from)?;

        Ok(notification_id)
    }

    #[dbus_interface(
        out_args("name", "vendor", "version", "spec_version"),
        name = "GetServerInformation"
    )]
    fn get_server_information(&mut self) -> zbus::fdo::Result<(String, String, String, String)> {
        let name = String::from("Rosl Notification Daemon");
        let vendor = String::from(env!("CARGO_PKG_NAME"));
        let version = String::from(env!("CARGO_PKG_VERSION"));
        let specification_version = String::from("1.2");

        Ok((name, vendor, version, specification_version))
    }

    #[dbus_interface(name = "GetCapabilities")]
    fn get_capabilities(&mut self) -> zbus::fdo::Result<Vec<&str>> {
        let capabilities = vec![
            "action-icons",
            "actions",
            "body",
            "body-hyperlinks",
            "body-images",
            "body-markup",
            "icon-multi",
            "icon-static",
            "persistence",
            "sound",
        ];

        Ok(capabilities)
    }
}
