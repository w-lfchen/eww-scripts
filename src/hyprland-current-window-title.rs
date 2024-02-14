use hyprland::data::Workspace;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::HyprDataActive;
use hyprland::Result;

fn main() -> Result<()> {
    print_window_title();
    let mut listener = EventListener::new();
    listener.add_active_window_change_handler(|_, _| print_window_title());
    listener.add_window_title_change_handler(|_, _| print_window_title());
    listener.add_window_close_handler(|_, _| print_window_title());
    listener.add_workspace_change_handler(|_, _| print_window_title());
    listener.start_listener()
}

fn print_window_title() {
    if let Ok(active_workspace) = Workspace::get_active() {
        println!("{}", active_workspace.last_window_title)
    } else {
        println!("Couldn't get window title!")
    }
}
