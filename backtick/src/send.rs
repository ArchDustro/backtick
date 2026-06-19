use zbus::Connection;
use std::collections::HashMap;
use crate::file::FileLogger;
use crate::thread::ThreadHandle;

pub enum Urgency {
    LOW,
    NEUTRAL,
    CRITICAL,
}

// stdout

pub fn send_to_stdout(
    thread: &ThreadHandle,
    name: String,
    contents: String,
    has_time: bool,
) {
    thread.run_on_thread(move || {
        let msg = if has_time {
            format!("{} | {} LOG: {}", crate::unix_timestamp(), name, contents)
        } else {
            format!("{} LOG: {}", name, contents)
        };
        println!("{msg}");
    });
}

// stderr

pub fn send_to_stderr(
    thread: &ThreadHandle,
    name: String,
    contents: String,
    has_time: bool,
) {
    thread.run_on_thread(move || {
        let msg = if has_time {
            format!("{} | {} LOG: {}", crate::unix_timestamp(), name, contents)
        } else {
            format!("{} LOG: {}", name, contents)
        };
        eprintln!("{msg}");
    });
}

// file

pub fn send_to_file(
    thread: &ThreadHandle,
    mut logger: FileLogger,
    name: String,
    contents: String,
    has_time: bool,
) {
    thread.run_on_thread(move || {
        let msg = if has_time {
            format!("{} | {} LOG: {}", crate::unix_timestamp(), name, contents)
        } else {
            format!("{} LOG: {}", name, contents)
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            logger.write(&msg).await.unwrap();
        });
    });
}

// dbus

pub fn send_via_dbus(
    thread: &ThreadHandle,
    name: String,
    contents: String,
    urgency: Urgency,
) {
    thread.run_on_thread(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let connection = Connection::session().await.unwrap();

            let mut hints = HashMap::new();
            match urgency {
                Urgency::CRITICAL => { hints.insert("urgency".to_string(), 2u8); }
                Urgency::NEUTRAL  => { hints.insert("urgency".to_string(), 1u8); }
                Urgency::LOW      => { hints.insert("urgency".to_string(), 0u8); }
            }

            let _ = connection
                .call_method(
                    Some("org.freedesktop.Notifications"),
                    "/org/freedesktop/Notifications",
                    Some("org.freedesktop.Notifications"),
                    "Notify",
                    &(
                        "backtick-logging",
                        0u32,
                        "",
                        name,
                        contents,
                        Vec::<String>::new(),
                        hints,
                        -1i32,
                    ),
                )
                .await;
        });
    });
}

