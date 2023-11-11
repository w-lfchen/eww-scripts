use std::process::Command;

use hyprland::{data::Monitors, shared::HyprData};

fn main() {
    let monitor_0_exists = Monitors::get()
        .expect("Unable to access monitors!")
        .map(|monitor| monitor.id)
        .any(|id| id == 0);
    let _ = Command::new("eww")
        .args([
            "open-many",
            "--toggle",
            &format!("bar-left-{}", if monitor_0_exists { 0 } else { 1 }),
            &format!("bar-right-{}", if monitor_0_exists { 1 } else { 0 }),
        ])
        .spawn()
        .expect("Unable to spawn eww process!")
        .wait();
}
