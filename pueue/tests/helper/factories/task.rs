use anyhow::{Context, Result};

use pueue_lib::network::message::*;
use pueue_lib::settings::*;

use crate::helper::*;

/// Adds a task to the test daemon.
pub async fn add_task(shared: &Shared, command: &str, start_immediately: bool) -> Result<Message> {
    let mut message = create_add_message(shared, command);
    message.start_immediately = start_immediately;

    send_message(shared, message)
        .await
        .context("Failed to to add task.")
}

/// Adds a task to a specific group of the test daemon.
pub async fn add_task_to_group(shared: &Shared, command: &str, group: &str) -> Result<Message> {
    let mut message = create_add_message(shared, command);
    message.group = group.to_string();

    send_message(shared, message)
        .await
        .context("Failed to to add task to group.")
}

/// Mini wrapper around add_task, which creates a task that echos PUEUE's worker environment
/// variables to `stdout`.
pub async fn add_env_task(shared: &Shared, command: &str) -> Result<Message> {
    let command = format!("echo WORKER_ID: $PUEUE_WORKER_ID; echo GROUP: $PUEUE_GROUP; {command}");
    add_task(shared, &command, false).await
}

/// Just like [add_env_task], but the task get's added to specific group.
pub async fn add_env_task_to_group(shared: &Shared, command: &str, group: &str) -> Result<Message> {
    let command = format!("echo WORKER_ID: $PUEUE_WORKER_ID; echo GROUP: $PUEUE_GROUP; {command}");
    add_task_to_group(shared, &command, group).await
}
