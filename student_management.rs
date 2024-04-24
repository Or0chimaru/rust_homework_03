/* 
请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。
*/
#![allow(dead_code)]
// Define the Student struct
#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    class_id: u32, // Relationship to Class
}

// Define the Class struct
struct Class {
    id: u32,
    name: String,
    students: Vec<Student>, // Relationship to Student
}

// Function to create a new student
fn create_student(id: u32, name: &str, class_id: u32) -> Student {
    // Create a new Student
    Student {
        id,
        name: name.to_string(),
        class_id,
    }
}

// Function to create a new class
fn create_class(id: u32, name: &str) -> Class {
    // Create a new Class
    Class {
        id,
        name: name.to_string(),
        students: Vec::new(),
    }
}

// Function to add a student to a class
fn add_student_to_class(student: Student, class: &mut Class) {
    class.students.push(student);
}

// Function to find a student by ID
fn find_student_by_id(students: &[Student], target_id: u32) -> Option<&Student> {
    students.iter().find(|&student| student.id == target_id)
}

// Function to update a student's name
fn update_student_name(student: &mut Student, new_name: &str) {
    student.name = new_name.to_string();
}

// Function to delete a student by ID
fn delete_student_by_id(students: &mut Vec<Student>, target_id: u32) -> bool {
    if let Some(index) = students.iter().position(|student| student.id == target_id) {
        students.remove(index);
        true
    } else {
        false
    }
}

// Main function to demonstrate CRUD operations
fn main() {
    let mut class = create_class(1, "Math Class");
    let student1 = create_student(1, "Alice", class.id);
    
    add_student_to_class(student1, &mut class);
    
    println!("Class: {} - Students: {:?}", class.name, class.students);
    
    if let Some(student) = find_student_by_id(&class.students, 1) {
        println!("Found student: {}", student.name);
        update_student_name(&mut class.students[0], "Bob");
        println!("Updated student name: {}", class.students[0].name);
        delete_student_by_id(&mut class.students, 1);
        println!("Deleted student with ID 1");
    }
}
