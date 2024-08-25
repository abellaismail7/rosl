use actions::NotificationAction;
use handler::NotificationHandler;
use tokio::sync::mpsc::Sender;
use zbus::connection;
pub mod actions;
pub mod errors;
pub mod handler;
pub mod notification;

const DBUS_NOTIFICATIONS_NAME: &str = "org.freedesktop.Notifications";
const DBUS_NOTIFICATIONS_PATH: &str = "/org/freedesktop/Notifications";

pub async fn connect_dbus(
    sender: Sender<NotificationAction>,
) -> Result<zbus::Connection, Box<dyn std::error::Error>> {
    let handler = NotificationHandler::new(sender);
    let connection = connection::Builder::session()?
        .name(DBUS_NOTIFICATIONS_NAME)?
        .serve_at(DBUS_NOTIFICATIONS_PATH, handler)?
        .build()
        .await?;
    Ok(connection)
}
