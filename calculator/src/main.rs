extern crate gtk;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let grid = gtk::Grid::new();
    let display = gtk::Entry::new();
    let button_1 = gtk::Button::new_with_label("1");
    let button_2 = gtk::Button::new_with_label("2");
    let button_3 = gtk::Button::new_with_label("3");
    let button_4 = gtk::Button::new_with_label("4");
    let button_5 = gtk::Button::new_with_label("5");
    let button_6 = gtk::Button::new_with_label("6");
    let button_7 = gtk::Button::new_with_label("7");
    let button_8 = gtk::Button::new_with_label("8");
    let button_9 = gtk::Button::new_with_label("9");
    let button_0 = gtk::Button::new_with_label("0");
    let button_add = gtk::Button::new_with_label("+");
    let button_subtract = gtk::Button::new_with_label("-");
    let button_multiply = gtk::Button::new_with_label("*");
    let button_divide = gtk::Button::new_with_label("/");
    let button_equals = gtk::Button::new_with_label("=");

    grid.attach(&button_1, 0, 1, 1, 1);
    grid.attach(&button_2, 1, 1, 1, 1);
    grid.attach(&button_3, 2, 1, 1, 1);
    grid.attach(&button_4, 0, 2, 1, 1);
    grid.attach(&button_5, 1, 2, 1, 1);
    grid.attach(&button_6, 2, 2, 1, 1);
    grid.attach(&button_7, 0, 3, 1, 1);
    grid.attach(&button_8, 1, 3, 1, 1);
    grid.attach(&button_9, 2, 3, 1, 1);
    grid.attach(&button_0, 1, 4, 1, 1);
    grid.attach(&button_add, 3, 1, 1, 1);
    grid.attach(&button_subtract, 3, 2, 1, 1);
    grid.attach(&button_multiply, 3, 3, 1, 1);
    grid.attach(&button_divide, 3, 4, 1, 1);
    grid.attach(&button_equals, 2, 4, 1, 2);
    grid.attach(&display, 0, 0, 4, 1);

    button_1.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "1";
        display.set_text(&new_text);
    });
    button_2.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "2";
        display.set_text(&new_text);
    });
    button_3.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "3";
        display.set_text(&new_text);
    });
    button_4.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "4";
        display.set_text(&new_text);
    });
    button_5.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "5";
        display.set_text(&new_text);
    });
    button_6.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "6";
        display.set_text(&new_text);
    });
    button_7.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "7";
        display.set_text(&new_text);
    });
    button_8.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "8";
        display.set_text(&new_text);
    });
    button_9.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "9";
        display.set_text(&new_text);
    });
    button_0.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "0";
        display.set_text(&new_text);
    });
    button_add.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "+";
        display.set_text(&new_text);
    });
    button_subtract.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "-";
        display.set_text(&new_text);
    });
    button_multiply.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "*";
        display.set_text(&new_text);
    });
    button_divide.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        let new_text = current_text + "/";
        display.set_text(&new_text);
    });
    button_equals.connect_clicked(|_| {
        let current_text = display.get_text().unwrap();
        // You can parse the text and perform the calculations here using Rust's built-in mathematical functions
        let result = current_text.parse::<i32>().unwrap();
        display.set_text(&result.to_string());
    });

    window.add(&grid);
    window.set_title("Calculator");
    window.set_default_size(300, 300);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
