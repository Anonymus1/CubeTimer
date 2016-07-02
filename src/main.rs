extern crate gtk;

use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window, Button, WindowType};
use std::io;

fn main() {
	println!("Hello cube timer");
	if gtk::init().is_err(){
		println!("gtk has failed");
		return;
	}
	//MessageDialog::new(None::<&Window>, DialogFlags::empty(), MessageType::Info, ButtonsType::Ok, "CubeTimer").run();
	let window = Window::new(WindowType::Toplevel);
	window.set_title("CubeTimer");
	window.set_default_size(1280, 720);
	let button = Button::new_with_label("Click me!");
	window.add(&button);
	window.show_all();

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	button.connect_clicked(|_| {
		println!("Clicked!");
	});

	gtk::main();
}
