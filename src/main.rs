use gtk::{prelude::*, ApplicationWindow, Orientation, Stack, StackSidebar};
use gtk::{glib, Application};

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

fn build_ui(app: &Application) {
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

    let label2 = gtk::Label::new(Some("Soundcloud Content")); 
    let grid_layout = gtk::Grid::builder()
        .row_spacing(5)
        .column_spacing(5)
        .build();
    grid_layout.attach(&label2, 0, 0, 1, 1);
    stack.add_titled(&grid_layout, Some("page2"), "Soundcloud"); 


    let label3 = gtk::Label::new(Some("Spotify Content")); 
    stack.add_titled(&label3, Some("page3"), "Spotify"); 


    let sidebar = StackSidebar::new(); 
    sidebar.set_stack(&stack); 
    hbox.append(&sidebar); 
    hbox.append(&stack); 

    window.set_child(Some(&hbox));

    window.present();
}