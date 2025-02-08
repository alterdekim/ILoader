use gtk::{prelude::*, ApplicationWindow, Orientation, Stack, StackSidebar};
use gtk::{glib, Application};
use glib::clone;

use std::{sync::OnceLock, error::Error};
use tokio::runtime::Runtime;

use soundcloud::sobjects::CloudPlaylists;

mod disk_util;
mod ipod_util;

const VENDOR_ID: u16 = 1452;
const PRODUCT_ID: u16 = 4617;

const APP_ID: &str = "com.alterdekim.iloader";

fn main() -> glib::ExitCode {
    /*for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if VENDOR_ID == device_desc.vendor_id() && PRODUCT_ID == device_desc.product_id() {
            println!("FOUND!");
            println!("{}", ipod_util::get_ipod_path().is_some());
        }
    }*/
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

fn build_ui(app: &Application) {
    let (sender, receiver) = async_channel::bounded::<CloudPlaylists>(1);
    runtime().spawn(clone!(
        #[strong]
        sender,
        async move {
            let app_version = soundcloud::get_app().await.unwrap().unwrap();
            let client_id = soundcloud::get_client_id().await.unwrap().unwrap();
            let user_id: u64 = 774639751;

            sender
                .send(soundcloud::get_playlists(user_id, client_id, app_version).await.unwrap())
                .await
                .expect("The channel needs to be open.");
        }
    ));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("ILoader")
        .default_width(980)
        .default_height(700)
        .build();

    let hbox = gtk::Box::new(Orientation::Horizontal, 5);

    let stack = Stack::new(); 
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight); // Add some pages to the stack 

    let label1 = gtk::Label::new(Some("Youtube Content")); 
    stack.add_titled(&label1, Some("page1"), "Youtube"); 

    let grid_layout = gtk::Grid::builder()
        .row_spacing(5)
        .column_spacing(5)
        .build();
    
    window.set_child(Some(&hbox));

    let label3 = gtk::Label::new(Some("Spotify Content")); 
    stack.add_titled(&label3, Some("page3"), "Spotify"); 

    let sidebar = StackSidebar::new(); 
    sidebar.set_stack(&stack); 

    hbox.append(&sidebar); 
    hbox.append(&stack); 

    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            for playlist in response.collection {
                let label2 = gtk::Label::new(Some(&playlist.title)); 
                grid_layout.attach(&label2, 0, 0, 1, 1);
            }
            stack.add_titled(&grid_layout, Some("page2"), "Soundcloud"); 
        }
    });

    window.present();
}