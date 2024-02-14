use hyprland::{data::Monitors, shared::HyprData};
use std::{collections::HashMap, process::Command};

fn main() {
    const LEFT_POS: i32 = 0;
    const RIGHT_POS: i32 = 2560;

    let mut map = HashMap::with_capacity(2);
    // this vector is neede because for some reason hyprland and eww sometimes disagree about which monitor is the one with id 1 and which ine has id 0
    // this is annoying and I hope it gets fixed, for example by eww using what hyprland condsiders the name attribute instead of the model
    // for now, try to decide based on the order in which hyprland returns the monitor data
    // every other solution looks even more ugly to me
    let mut check_vec = Vec::with_capacity(2);
    Monitors::get()
        .expect("Unable to access monitors!")
        .map(|monitor| (monitor.id, monitor.x))
        .for_each(|(id, x_pos)| {
            check_vec.push(id);
            if map.insert(x_pos, id).is_some() {
                panic!("Duplicate monitor x position!")
            }
        });
    let mut left_id = map
        .get(&LEFT_POS)
        .unwrap_or_else(|| panic!("No monitor with x position {}!", LEFT_POS));
    let mut right_id = map
        .get(&RIGHT_POS)
        .unwrap_or_else(|| panic!("No monitor with x position {}!", RIGHT_POS));
    // eww and hyprland disagree
    if check_vec[0] > check_vec[1] {
        // swap the ids
        let tmp = left_id;
        left_id = right_id;
        right_id = tmp
    }
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
