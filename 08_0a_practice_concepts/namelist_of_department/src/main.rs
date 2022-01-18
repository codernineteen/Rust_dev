use std::collections::HashMap;

fn main() {
    let mut department = HashMap::new();
    department.insert(String::from("Sally"), String::from("Engineering"));
    department.insert(String::from("Gregory"), String::from("Engineering"));
    department.insert(String::from("Bill"), String::from("Engineering"));
    department.insert(String::from("Mark"), String::from("Engineering"));
    department.insert(String::from("Jeff"), String::from("Engineering"));
    department.insert(String::from("Ellon"), String::from("Engineering"));
    department.insert(String::from("Kant"), String::from("HumanResource"));
    get_employers_from_department("Engineering", &mut department);
}

fn get_employers_from_department(department: &str, map: &mut HashMap<String, String>) {
    for (key, value) in &*map {
        if department == value {
            println!("{}-{}", key, value);
        }
    }
}
