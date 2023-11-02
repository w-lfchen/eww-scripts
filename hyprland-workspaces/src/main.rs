use hyprland::data::{Workspace, Workspaces};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::Result;
use std::{env, process};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => listen_single(),
        2 => {
            // TODO: expect second argument to be the monitor in question, parse and handle accordingly
            print_data_mult();
            process::exit(0)
        }
        _ => {
            println!("Wrong usage: too many arguments!");
            process::exit(1)
        }
    }
}

fn listen_single() -> Result<()>{
    print_data_single();
    let mut listener = EventListener::new();
    listener.add_workspace_change_handler(|_, _| print_data_single());
    listener.add_workspace_added_handler(|_, _| print_data_single());
    listener.add_workspace_destroy_handler(|_, _| print_data_single());
    listener.add_workspace_moved_handler(|_, _| print_data_single());
    listener.start_listener()
}

fn print_data_single() {
    let active_id: i32 = Workspace::get_active()
        .expect("Couldn't get active workspace!")
        .id;
    let mut workspaces: Vec<_> = Workspaces::get()
        .expect("Couldn't get workspaces vector!")
        .into_iter()
        .collect();
    workspaces.sort_by_key(|x| x.id);
    let mut string: String = "[".to_string();
    for i in workspaces.into_iter().map(|x| {
        format!(
            "{{\"id\":{},\"name\":\"{}\",\"class\":\"ws-button ws{}\",\"active\":{}}}",
            x.id,
            x.name,
            x.id,
            x.id == active_id
        )
    }) {
        string.push_str(i.as_str());
        string.push(',');
    }
    string.pop(); // remove last comma
    string.push(']');
    println!("{string}");
}

fn print_data_mult() {
    // TODO
}
