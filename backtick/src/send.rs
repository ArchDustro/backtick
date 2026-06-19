use zbus::Connection;
use std::collections::HashMap;
use crate::file::FileLogger;
use crate::thread::ThreadHandle;
use zbus::zvariant::Value;
use once_cell::sync::Lazy;
use crate::thread::THREAD;

pub enum Urgency {
    LOW,
    NEUTRAL,
    CRITICAL,
}

// stdout

pub fn send_to_stdout(name: String, contents: String, has_time: bool) {
    THREAD.run_on_thread(move || async move {
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
    name: String,
    contents: String,
    has_time: bool,
) {
    THREAD.run_on_thread(move || async move {
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
    mut logger: FileLogger,
    name: String,
    contents: String,
    has_time: bool,
) {
    THREAD.run_on_thread(move || async move {
        let msg = if has_time {
            format!("{} | {} LOG: {}", crate::unix_timestamp(), name, contents)
        } else {
            format!("{} LOG: {}", name, contents)
        };

        logger.write(&msg).await.unwrap();
    });
}

// dbus
pub fn send_via_dbus(
    name: String,
    contents: String,
    urgency: Urgency,
) {
    THREAD.run_on_thread(move || async move {
        eprintln!("[dbus] about to send notification");

        let connection = Connection::session()
            .await
            .expect("[dbus] failed to connect to session bus");

        let mut hints: HashMap<&str, Value> = HashMap::new();
        match urgency {
            Urgency::CRITICAL => hints.insert("urgency", Value::U8(2)),
            Urgency::NEUTRAL  => hints.insert("urgency", Value::U8(1)),
            Urgency::LOW      => hints.insert("urgency", Value::U8(0)),
        };

        let res = connection
            .call_method(
                Some("org.freedesktop.Notifications"),
                "/org/freedesktop/Notifications",
                Some("org.freedesktop.Notifications"),
                "Notify",
                &(
                    "backtick-logging",
                    0u32,
                    "dialog-information",
                    name,
                    contents,
                    Vec::<&str>::new(),
                    hints,
                    -1i32,
                ),
            )
            .await;

        eprintln!("[dbus] call_method result: {:?}", res);
    });
}
