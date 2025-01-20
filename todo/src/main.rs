use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

fn main() {
    let file_path = "todo_list.txt";

    let todo_vec = Arc::new(Mutex::new(
        read_to_string(file_path)
            .unwrap_or_default()
            .lines()
            .map(String::from)
            .collect::<Vec<_>>(),
    ));

    let mut siv = Cursive::default();

    let todo_vec_add = Arc::clone(&todo_vec);
    let todo_vec_view = Arc::clone(&todo_vec);
    let todo_vec_remove = Arc::clone(&todo_vec);
    let todo_vec_save = Arc::clone(&todo_vec);

    siv.add_layer(
        Dialog::new()
            .title("Welcome to Rust-Do!")
            .content(TextView::new("Select an option").h_align(HAlign::Center))
            .button("Add Task", move |s| show_add_task(s, Arc::clone(&todo_vec_add)))
            .button("View Tasks", move |s| show_tasks(s, Arc::clone(&todo_vec_view)))
            .button("Remove Task", move |s| show_remove_task(s, Arc::clone(&todo_vec_remove)))
            .button("Save and Exit", move |s| save_and_exit(s, Arc::clone(&todo_vec_save), file_path)),
    );

    siv.run();
}

fn show_add_task(s: &mut Cursive, todo_vec: Arc<Mutex<Vec<String>>>) {
    s.add_layer(
        Dialog::new()
            .title("Add Task")
            .content(EditView::new().on_submit(move |s, input| {
                let mut tasks = todo_vec.lock().unwrap();
                tasks.push(input.to_string());
                s.pop_layer();
            }))
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn show_tasks(s: &mut Cursive, todo_vec: Arc<Mutex<Vec<String>>>) {
    let tasks = todo_vec.lock().unwrap();
    let mut list = SelectView::new();

    for task in &*tasks {
        list.add_item(task.clone(), task.clone());
    }

    s.add_layer(
        Dialog::new()
            .title("Current Tasks")
            .content(list.fixed_size((30, 10)))
            .button("OK", |s| {
                s.pop_layer();
            }),
    );
}

fn show_remove_task(s: &mut Cursive, todo_vec: Arc<Mutex<Vec<String>>>) {
    let mut list = SelectView::new();

    {
        let tasks = todo_vec.lock().unwrap();
        for (i, task) in tasks.iter().enumerate() {
            list.add_item(format!("{}. {}", i + 1, task), i);
        }
    }

    s.add_layer(
        Dialog::new()
            .title("Remove Task")
            .content(
                list.on_submit(move |s, index: &usize| {
                    let mut tasks = todo_vec.lock().unwrap();
                    tasks.remove(*index);
                    s.pop_layer();
                    s.pop_layer();
                }),
            )
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn save_and_exit(s: &mut Cursive, todo_vec: Arc<Mutex<Vec<String>>>, file_path: &str) {
    let tasks = todo_vec.lock().unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open the file!");

    let content_to_write = tasks.join("\n") + "\n";
    file.write_all(content_to_write.as_bytes())
        .expect("Could not write to the file!");

    s.quit();
}
