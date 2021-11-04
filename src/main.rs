use gdk_x11_sys::gdk_x11_display_get_xdisplay;
use gdkx11::{X11Display, X11Window};
use glib::translate::*;
use glib::Cast;
use gtk::prelude::*;
use x11_dl::xlib::Xlib;

use std::sync::{Arc, Mutex};

fn main() {
    unsafe {
        gtk::init().unwrap();

        let display = gdk::Display::default().unwrap();
        let display: X11Display = display.downcast().unwrap();
        let raw: *mut gdkx11::ffi::GdkX11Display = display.to_glib_none().0;
        let d = gdk_x11_display_get_xdisplay(raw) as *mut x11_dl::xlib::_XDisplay;

        let xlib = Xlib::open().unwrap();
        let root = (xlib.XDefaultRootWindow)(d);
        let w = (xlib.XCreateSimpleWindow)(d, root, 0, 0, 300, 300, 0, 0, 0);
        (xlib.XMapRaised)(d, w);

        let gw = X11Window::foreign_new_for_display(&display, w).upcast();

        let gtk = gtk::Window::new(gtk::WindowType::Toplevel);
        gtk.connect_realize(move |widget| {
            widget.set_window(&gw);
        });
        gtk.set_has_window(true);
        gtk.realize();

        let menubar = gtk::MenuBar::new();
        let file = gtk::MenuItem::with_label("File");
        let filemenu = gtk::Menu::new();
        let quit = gtk::MenuItem::with_label("Quit");
        filemenu.append(&quit);
        file.set_submenu(Some(&filemenu));
        menubar.append(&file);

        let boox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        boox.pack_start(&menubar, false, false, 3);
        gtk.add(&boox);

        let running = Arc::new(Mutex::new(true));
        let r_ = running.clone();
        quit.connect_activate(move |_| {
            let mut r = r_.lock().unwrap();
            *r = false;
        });

        gtk.show_all();

        while *running.lock().unwrap() {
            while gtk::events_pending() {
                gtk::main_iteration();
            }
        }
    }
}
