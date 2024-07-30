use hyprland::{data::Monitors, shared::HyprData, Result};
use std::{collections::HashMap, process::Command};

fn main() -> Result<()> {
    let args: Vec<i32> = std::env::args()
        .skip(1)
        .enumerate()
        .map(|(i, n)| {
            n.parse()
                .unwrap_or_else(|_| panic!("Invalid argument format at position {i}!"))
        })
        .collect();
    match args.as_slice() {
        [] | [1] => spawn_one_bar(),
        [2, left_pos, right_pos] => spawn_two_bars(*left_pos, *right_pos),
        _ => panic!("Invalid argument format!"),
    }
}

fn spawn_one_bar() -> Result<()> {
    if !Command::new("eww")
        .args(["open", "--toggle", "bar"])
        .spawn()?
        .wait()?
        .success()
    {
        panic!("Eww failed to spawn bar!");
    }
    Ok(())
}

fn spawn_two_bars(left_pos: i32, right_pos: i32) -> Result<()> {
    let mut map = HashMap::with_capacity(2);
    // this vector is needed because for some reason hyprland and eww sometimes disagree about which monitor is the one with id 1 and which one has id 0
    // this is annoying and I hope it gets fixed, for example by eww using what hyprland condsiders the name attribute instead of the model
    // for now, try to decide based on the order in which hyprland returns the monitor data
    // every other solution looks even more ugly to me
    let mut check_vec = Vec::with_capacity(2);
    Monitors::get()?
        .into_iter()
        .map(|monitor| (monitor.id, monitor.x))
        .for_each(|(id, x_pos)| {
            check_vec.push(id);
            if map.insert(x_pos, id).is_some() {
                panic!("Duplicate monitor x position!")
            }
        });
    let mut left_id = map
        .get(&left_pos)
        .unwrap_or_else(|| panic!("No monitor with x position {left_pos}!"));
    let mut right_id = map
        .get(&right_pos)
        .unwrap_or_else(|| panic!("No monitor with x position {right_pos}!"));
    // eww and hyprland disagree
    if check_vec[0] > check_vec[1] {
        // swap the ids
        let tmp = left_id;
        left_id = right_id;
        right_id = tmp
    }
    // spawn bars
    if !Command::new("eww")
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
        .spawn()?
        .wait()?
        .success()
    {
        panic!("Eww failed to spawn bars!");
    }
    Ok(())
}
