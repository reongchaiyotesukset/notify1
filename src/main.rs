use std::rc::Rc;

use gtk::prelude::*;

use gtk::{Button, Entry, Window, WindowType,Layout};
use notify_rust::Notification;
use notify_rust::Hint;



   
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    gtk::init();


let window = Window::new(WindowType::Toplevel);
window.set_title("MySQL Example");
window.set_default_size(350, 250);

let layout = Layout::builder()
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();

let entry_id = Entry::builder()
            .placeholder_text("id")
            .can_default(true)
            .can_focus(true)          
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
let entry_data = Entry::builder()
            .placeholder_text("data")
            .can_default(true)
            .can_focus(true)          
            .margin(20)
            .margin_bottom(20)
            .margin_top(62)
            .margin_start(20)
            .margin_end(20)
            .build();
           
let save_button = Button::builder()
            .label("Click1")
            .margin(60)
            .margin_bottom(60)
            .margin_top(108)
            .margin_start(60)
            .margin_end(60)
            .build();
       
layout.add(&entry_id);
        layout.add(&entry_data);
layout.add(&save_button);
window.add(&layout);
//test



//

        save_button.connect_clicked(move|_| {

				//let id  =   entry_id.text().parse::<i32>().unwrap();
				//let data = entry_data.text().to_string();

                //insert_data2();
                
                Notification::new().summary("click me")
                   .action("default", "default")
                   .action("clicked", "click here")
                   .hint(Hint::Resident(true))
                   .show()
                   .unwrap()
                   .wait_for_action(|action| match action {
                                        "default" => println!("you clicked \"default\""),
                                        "clicked" => println!("that was correct"),
                                        // here "__closed" is a hard coded keyword
                                        "__closed" => println!("the notification was closed"),
                                        _ => ()
                                    });
                 
                  
        });
       
        window.connect_delete_event(|_, _| {
           gtk::main_quit();
           Inhibit(false)
        });
       
     
        window.show_all();
       gtk::main();   
          Ok(())
}

	
