use hyprland::{data::Monitors, shared::HyprData};
use std::{collections::HashMap, process::Command};

fn main() {
    const LEFT_POS: i32 = 0;
    const RIGHT_POS: i32 = 2560;

    let mut map = HashMap::with_capacity(2);
    Monitors::get()
        .expect("Unable to access monitors!")
        .map(|monitor| (monitor.id, monitor.x))
        .for_each(|(id, x_pos)| {
            if map.insert(x_pos, id).is_some() {
                panic!("Duplicate monitor x position!")
            }
        });
    let left_id = map
        .get(&LEFT_POS)
        .unwrap_or_else(|| panic!("No monitor with x position {}!", LEFT_POS));
    let right_id = map
        .get(&RIGHT_POS)
        .unwrap_or_else(|| panic!("No monitor with x position {}!", RIGHT_POS));
    // spawn bars
    let _ = Command::new("eww")
        .args([
            "open-many",
            "--toggle",
            "left-bar:left",
            "right-bar:right",
            "--arg",
            &format!("left:screen={}", left_id),
            "--arg",
            &format!("right:screen={}", right_id),
        ])
        .spawn()
        .expect("Unable to spawn eww process!")
        .wait();
}
