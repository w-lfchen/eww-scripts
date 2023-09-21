use hyprland::data::{Workspace, Workspaces};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::Result;

fn main() -> Result<()> {
    print_data();
    let mut listener = EventListener::new();
    listener.add_workspace_change_handler(|_, _| print_data());
    listener.add_workspace_added_handler(|_, _| print_data());
    listener.add_workspace_destroy_handler(|_, _| print_data());
    listener.add_workspace_moved_handler(|_, _| print_data());
    listener.start_listener()
}

fn print_data() {
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
