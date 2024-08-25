use crate::notification::Notification;

#[derive(Debug)]
pub enum NotificationAction {
    ActionClose { notification_id: u32, reason: u32 },
    ActionInvoked { notification_id: u32 },
    Notify { notification: Notification },
    Close { notification_id: u32 },
}
