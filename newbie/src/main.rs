use std::io::{self, stdin, Error, Write};
use std::ops::{Add, Deref};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

struct Student {
    pub id: i32,
    pub name: String,
    pub class: String
}
impl Student {
    fn default() -> Student {
        Student{id: 0,name: String::new(),class: String::new()}
    }
    fn print(&self) {
        println!("--> id: {}, name: {}, class: {}", self.id, self.class, self.name);
    }
}

fn input_student() -> Student {
    let mut student = Student::default();

    println!("Please input your id: ");
    let mut id_str = String::new();
    let result = stdin().read_line(&mut id_str);
    if let Err(err) = result {
        panic!("Error reading input: {}", err);
    }
    let id = id_str.trim().parse::<i32>().unwrap();
    student.id = id;


    println!("Please input your class: ");
    let mut class_str = String::new();
    let rs = stdin().read_line(&mut class_str);
    if let Err(err) = rs {
        panic!("Error reading input class: {}", err);
    }
    student.class = class_str.trim().to_string();


    println!("Please input your name: ");
    let mut name_str = String::new();
    let rs = stdin().read_line(&mut name_str);
    if let Err(err) = rs {
        panic!("Error reading input name: {}", err);
    }
    student.name = name_str.trim().to_string();
    student
}

fn main() {
    let mut students = Vec::new();
    let std = Student{id: 1, name: "foo1".to_string(),class: "bar1".to_string()};
    let std2 = Student{id: 2, name: "foo2".to_string(),class: "bar2".to_string()};
    let std3 = Student{id: 3, name: "foo3".to_string(),class: "bar3".to_string()};

    students.push(std);
    students.push(std2);
    students.push(std3);

    for student in &students {
        println!("{} - {} - {}", &student.id, student.name, student.class);
    }

    let data = Box::new(0);
    let mut share_count = Arc::new(Mutex::new(data));
    println!("before clone rc : {}", Arc::strong_count(&share_count));
    let mut clone_count = share_count.clone();
    println!("afters clone rc : {}", Arc::strong_count(&share_count));
    std::thread::spawn( move|| {
        for i in 0..10000{
            sleep(Duration::from_secs(1));
            let mut m_g = clone_count.lock().unwrap();
            let mut  data = m_g.as_mut();
            *data = *data + 1;
        }
    });
    sleep(Duration::from_secs(2));
    println!("final rc : {}", Arc::strong_count(&share_count));
    println!("count = {}", *share_count.lock().unwrap());

}
