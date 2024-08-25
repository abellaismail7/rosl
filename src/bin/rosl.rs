use std::{
    collections::LinkedList,
    future::pending,
    io::{self, Write},
};

use scripts::{actions::NotificationAction, connect_dbus, notification::Notification};
use std::sync::LazyLock;
use tokio::{self, sync::mpsc};

static NOTIFICATION_LIST: LazyLock<LinkedList<Notification>> = LazyLock::new(LinkedList::new);

macro_rules! emit_notification_signal {
    ($connection: ident, $action: tt, $params: expr ) => {
        $connection
            .emit_signal(
                None::<()>,
                "/org/freedesktop/Notifications",
                "org.freedesktop.Notifications",
                $action,
                $params,
            )
            .await
            .expect("could not emit NotificationClosed signal");
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, mut receiver) = mpsc::channel(5);
    let connection = connect_dbus(sender).await?;

    tokio::spawn(async move {
        while let Some(action) = receiver.recv().await {
            match action {
                NotificationAction::ActionInvoked { notification_id } => {
                    emit_notification_signal!(
                        connection,
                        "ActionInvoked",
                        &(notification_id as u32, "default")
                    );
                }
                NotificationAction::ActionClose {
                    notification_id,
                    reason,
                } => {
                    emit_notification_signal!(
                        connection,
                        "NotificationClosed",
                        &(notification_id as u32, reason)
                    );
                }
                NotificationAction::Notify { notification } => {
                    let res = format!(
                        " 
                          (box 
                            :orientation 'vertical'
                            (button :class 'notif'
                             (box 
                                  :orientation 'horizontal' :space-evenly false
                                (image :image-width 80 :image-height 80 :path '{}')
                                (box :orientation 'vertical'
                                  (label :width 100 :wrap true :text '{}')
                                  (label :width 100 :wrap true :text '{}')
                            )))
                          )
                        ",
                        notification.app_icon, notification.summary, notification.body
                    );
                    println!("{}", res.replace("\n", ""));
                    io::stdout().flush().expect("Expect ");
                }
                NotificationAction::Close { notification_id } => {}
            }
        }
    });
    pending::<()>().await;
    Ok(())
}
