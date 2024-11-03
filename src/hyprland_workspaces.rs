use hyprland::data::{Monitor, Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::Result;

pub(super) fn listen_single_mon() -> Result<()> {
    print_data_single();
    let mut listener = EventListener::new();
    listener.add_active_monitor_changed_handler(|_| print_data_single());
    listener.add_workspace_added_handler(|_| print_data_single());
    listener.add_workspace_changed_handler(|_| print_data_single());
    listener.add_workspace_deleted_handler(|_| print_data_single());
    listener.add_workspace_moved_handler(|_| print_data_single());
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
    workspaces.sort_by_key(|ws| ws.id);
    let mut string: String = "[".to_string();
    for str in workspaces.iter().map(|ws| {
        format!(
            r#"{{"id":{},"name":"{}","class":"ws-button ws{}","active":{}}},"#,
            ws.id,
            ws.name,
            ws.id,
            ws.id == active_id
        )
    }) {
        string.push_str(&str);
    }
    string.pop(); // remove last comma
    println!("{string}]");
}

pub(super) fn listen_mon(input: &str) -> Result<()> {
    let id = Monitors::get()?
        .into_iter()
        .find(|monitor| monitor.name == input)
        .map(|monitor| monitor.id)
        .expect("Wrong usage: Unable to find monitor \"{input}\"");
    print_mon_data(id);
    let mut listener = EventListener::new();
    listener.add_active_monitor_changed_handler(move |_| print_mon_data(id));
    listener.add_workspace_added_handler(move |_| print_mon_data(id));
    listener.add_workspace_changed_handler(move |_| print_mon_data(id));
    listener.add_workspace_deleted_handler(move |_| print_mon_data(id));
    listener.add_workspace_moved_handler(move |_| print_mon_data(id));
    listener.start_listener()
}

fn print_mon_data(id: i128) {
    let (active_ws_id, mon_name) = Monitors::get()
        .expect("Unable to access monitors!")
        .into_iter()
        .find(|monitor| monitor.id == id)
        .map(|monitor| (monitor.active_workspace.id, monitor.name))
        .expect("Recieved an invalid monitor id!");
    let mut workspaces: Vec<_> = Workspaces::get()
        .expect("Couldn't get workspaces vector!")
        .into_iter()
        .filter(|ws| ws.monitor == mon_name)
        .collect();
    workspaces.sort_by_key(|ws| ws.id);
    let mut string: String = String::from("{\"active\":");
    string.push_str(
        if Monitor::get_active()
            .expect("Couldn't access active monitor!")
            .id
            == id
        {
            "true,\"workspaces\":["
        } else {
            "false,\"workspaces\":["
        },
    );
    for str in workspaces.iter().map(|ws| {
        format!(
            r#"{{"id":{},"name":"{}","class":"ws-button ws{}","active":{}}},"#,
            ws.id,
            ws.name,
            ws.id,
            ws.id == active_ws_id
        )
    }) {
        string.push_str(&str);
    }
    string.pop(); // remove last comma
    println!("{string}]}}");
}
