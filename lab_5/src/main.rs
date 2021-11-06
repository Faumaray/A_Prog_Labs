#![windows_subsystem = "windows"]

use gtk::prelude::*;


use db::*;
use gtk::{
    Application, TextView, TextBuffer,
};
use gtk::glib::clone;
use gtk::glib::signal::Inhibit;
use std::rc::Rc;



fn main() {
    let application =
    gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &Application) {
    let window = Rc::new(gtk::ApplicationWindow::new(application));

    window.set_title(Some("Lab_5"));
    window.set_default_size(350, 450);
    window.set_resizable(false);

    let main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let top_main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let bottom_main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let buttons_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    
    let buffer: TextBuffer = TextBuffer::new(Option::None);
    let textview: TextView = TextView::builder().editable(false).overwrite(true).visible(true).buffer(&buffer).build();
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(300)
        .min_content_height(360)
        .width_request(300)
        .height_request(350)
        .visible(true)
        .child(&textview)
        .build();
    
    let button_show = gtk::Button::builder()
        .label("Show All")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(100)
        .height_request(50)
        .visible(true)
        .build();
    let button_add = gtk::Button::builder()
        .label("Add")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(100)
        .height_request(50)
        .visible(true)
        .build();
    let button_delete = gtk::Button::builder()
        .label("Remove")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(100)
        .height_request(50)
        .visible(true)
        .build();
    let search_buffer: gtk::EntryBuffer = gtk::EntryBuffer::new(Option::None);
    let search = gtk::Entry::builder()
        .buffer(&search_buffer)
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("Search by ID")
        .height_request(20)
        .width_request(150)
        .build();
    let search_button = gtk::Button::builder()
        .label("")
        .width_request(20)
        .height_request(20)
        .visible(true)
        .build();
    
    bottom_main_box.append(&search);
    bottom_main_box.append(&search_button);
    buttons_box.append(&button_show);
    buttons_box.append(&button_add);
    buttons_box.append(&button_delete);
    top_main_box.append(&scrolled_window);
    top_main_box.append(&buttons_box);
    main_box.append(&top_main_box);
    main_box.append(&bottom_main_box);
    window.set_child(Some(&main_box));
    let connection = Rc::new(establish_connection());
    button_add.connect_clicked(clone!(@strong window, @strong connection =>
        move |_| {
            gtk::glib::MainContext::default().spawn_local(add_dialog(Rc::clone(&window),Rc::clone(&connection)));
        }
    ));
    button_delete.connect_clicked(clone!(@strong window, @strong connection =>
        move |_| {
            gtk::glib::MainContext::default().spawn_local(delete_dialog(Rc::clone(&window), Rc::clone(&connection)));
        }
    ));
    button_show.connect_clicked(clone!(@strong window, @strong connection => move |_| {
        let workers = match get_all_data(Rc::clone(&connection)){
            Ok(workers) => {workers},
            Err(error) => {
                gtk::glib::MainContext::default().spawn_local(error_dialog(Rc::clone(&window), format!("Error occured: {}", error)));
                Vec::<db::models::Worker>::new()
            },
        };
        if workers.len() == 0
        {
            return;
        }
        let mut workers_in_string = String::new();
        for worker in workers
        {
            workers_in_string.push_str(format!("id={}\nname={}\nmanager={}\nsalary={}\ndivision number={}\n-------------------------------------\n",worker.id,worker.fname,worker.manager,worker.salary,worker.div_num).as_str());
        }
        let _ = &buffer.set_text(workers_in_string.as_str());
    }));
    search_button.connect_clicked(clone!(@strong window, @strong connection => move |_| {
        let id_select: i32 = search_buffer.text().parse().unwrap();
        let worker = match get_data_by_id(Rc::clone(&connection), id_select)
        {
            Ok(worker) => {worker},
            Err(error) => {
                gtk::glib::MainContext::default().spawn_local(error_dialog(Rc::clone(&window), format!("Error occured: {}", error)));
                Vec::<db::models::Worker>::new()
            },
        };
        if worker.len() == 0
        {
            return;
        }
        let worker_in_string = format!("id={}\nname={}\nmanager={}\nsalary={}\ndivision number={}\n-------------------------------------\n",worker[0].id,worker[0].fname,worker[0].manager,worker[0].salary,worker[0].div_num);
        let _ = &textview.buffer().set_text(worker_in_string.as_str());
    }));
    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        Inhibit(false)
    });
    window.show();
}
async fn error_dialog<W: IsA<gtk::Window>>(window: Rc<W>, error_msg: String) {
    let question_dialog = gtk::MessageDialog::builder()
    .transient_for(&*window)
    .modal(true)
    .buttons(gtk::ButtonsType::Ok)
    .text(error_msg.as_str())
    .build();

    question_dialog.run_future().await;
    question_dialog.close();
}


async fn delete_dialog<W: IsA<gtk::Window>>(window: Rc<W>, connection: Rc<diesel::PgConnection>) {
    let id = gtk::Entry::builder()
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("ID")
        .height_request(20)
        .width_request(100)
        .build();
    let add_dialog = gtk::Dialog::builder()
        .modal(true)
        .resizable(false)
        .transient_for(&*window)
        .default_width(100)
        .title("Delete")
        .default_height(50)
        .use_header_bar(1)
        .build();

    add_dialog.add_buttons(&[("Ok",gtk::ResponseType::Ok),("Cancel",gtk::ResponseType::Cancel)]);
    add_dialog.set_child(Some(&id));
    let answer = add_dialog.run_future().await;

    match answer {
        gtk::ResponseType::Ok =>{
            let id_del: i32 = id.buffer().text().parse().unwrap();
            match delete_worker(connection, id_del){
                Ok(del)=>{
                    let question_dialog = gtk::MessageDialog::builder()
                        .transient_for(&add_dialog)
                        .modal(true)
                        .buttons(gtk::ButtonsType::Ok)
                        .text(format!("Deleted id: {}", del).as_str())
                        .build();

                        question_dialog.run_future().await;
                        question_dialog.close();
                }
                Err(error)=>{
                    let question_dialog = gtk::MessageDialog::builder()
                        .transient_for(&add_dialog)
                        .modal(true)
                        .buttons(gtk::ButtonsType::Ok)
                        .text(format!("Error occured: {}", error).as_str())
                        .build();

                        question_dialog.run_future().await;
                        question_dialog.close();
                }
            }
        },
        _ =>{

        }
    }

    add_dialog.close();
}
async fn add_dialog<W: IsA<gtk::Window>>(window: Rc<W>, connection: Rc<diesel::PgConnection>)
{
    let main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let id = gtk::Entry::builder()
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("ID")
        .height_request(20)
        .width_request(100)
        .build();
    let name = gtk::Entry::builder()
        .input_purpose(gtk::InputPurpose::Name)
        .placeholder_text("First name")
        .height_request(20)
        .width_request(100)
        .build();
    let manager = gtk::Entry::builder()
        .input_purpose(gtk::InputPurpose::Name)
        .placeholder_text("Manager")
        .height_request(20)
        .width_request(100)
        .build();
    let salary = gtk::Entry::builder()
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("Salary")
        .height_request(20)
        .width_request(100)
        .build();
    let div_num = gtk::Entry::builder()
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("Division number")
        .height_request(20)
        .width_request(100)
        .build();
    main_box.append(&id);
    main_box.append(&name);
    main_box.append(&manager);
    main_box.append(&salary);
    main_box.append(&div_num);
    let add_dialog = gtk::Dialog::builder()
        .modal(true)
        .resizable(false)
        .transient_for(&*window)
        .default_width(250)
        .title("Insert")
        .default_height(250)
        .use_header_bar(1)
        .build();

    add_dialog.add_buttons(&[("Ok",gtk::ResponseType::Ok),("Cancel",gtk::ResponseType::Cancel)]);
    add_dialog.set_child(Some(&main_box));
    let answer = add_dialog.run_future().await;

    match answer {
        gtk::ResponseType::Ok =>{
            if id.buffer().text().is_empty() || name.buffer().text().is_empty() || manager.buffer().text().is_empty() || salary.buffer().text().is_empty() || div_num.buffer().text().is_empty()
            {
                let info = gtk::MessageDialog::new(Some(&add_dialog),gtk::DialogFlags::MODAL, gtk::MessageType::Error, gtk::ButtonsType::Ok, "Need fill all fields");
                info.run_future().await;
                info.close();
                
            }
            else
            {
                let id_i: i32 = id.buffer().text().parse().unwrap();
                let salary_i: i32 = salary.buffer().text().parse().unwrap(); 
                let div_num_i: i32 = div_num.buffer().text().parse().unwrap();
                match insert_worker(Rc::clone(&connection), &id_i, name.buffer().text().as_str(), manager.buffer().text().as_str(), &salary_i, &div_num_i)
                {
                    Ok(_worker)=>{},
                    Err(error)=>{
                        let question_dialog = gtk::MessageDialog::builder()
                        .transient_for(&add_dialog)
                        .modal(true)
                        .buttons(gtk::ButtonsType::Ok)
                        .text(format!("Error occured: {}", error).as_str())
                        .build();

                        question_dialog.run_future().await;
                        question_dialog.close();
                    }
                }
            }
            add_dialog.close();
        },
        _ =>{
            add_dialog.close();
        }
    }

    
}