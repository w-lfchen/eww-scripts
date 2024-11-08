use hyprland::data::Workspace;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActive;
use hyprland::Result;

pub(super) fn listen_window_title() -> Result<()> {
    print_window_title();
    let mut listener = EventListener::new();
    listener.add_active_monitor_changed_handler(|_| print_window_title());
    listener.add_active_window_changed_handler(|_| print_window_title());
    listener.add_window_closed_handler(|_| print_window_title());
    listener.add_window_title_changed_handler(|_| print_window_title());
    listener.add_workspace_changed_handler(|_| print_window_title());
    listener.start_listener()
}

fn print_window_title() {
    if let Ok(active_workspace) = Workspace::get_active() {
        println!("{}", active_workspace.last_window_title)
    } else {
        println!("Couldn't get window title!")
    }
}
