fn main() {

  // this is data(an instance of the Employee Struct)
  // this is immutable by default
  let employee_one = Employee {
    name: String::from("Employee One"),
    email: String::from("employee_one@company.com"),
    avatar: String::from("ebac3a7570c013f55da056956f2e1e797c5af9a"),
    active: true,
    start_date: String::from("25-04-2020")
  };

  // accessing struct values using dot notation
  println!("{}", employee_one.name);
  println!("{}", employee_one.email);
  println!("{}", employee_one.avatar);
  println!("{}", employee_one.active);
  println!("{}", employee_one.start_date);

  // employee two struct is mutable
  let mut employee_two = Employee {
    name: String::from("Employee Two"),
    email: String::from("employee_two@company.com"),
    ..employee_one // this transfers ownership of employee one data
    // to employee two, from here employee_one.avatar would return an error
    // active is still valid since it use the Copy trait
  };

  println!("{}", employee_two.start_date);

  // mutate start date
  employee_two.start_date = String::from("10-08-2023");

  println!("{}", employee_two.start_date);

  // let employee_one_info = build_employee_work_info(String::from("finance"), String::from("Finance Assistant"),String::from("Jane Doe"));

  println!("{:?}", employee_two)
}

/*
 * ==== STRUCTS ====
 * a struct(structure) is a custom data type that is used to group multiple
 * related values
 * 
 * struct kekyword -> Struct name -> opening curly brace -> fields -> closing curly brace
 * 
 * structs are immutable by default as well and we use `mut` to create a mutable struct
 * mutability is on the struct, not per field
 * 
 * dot notation to access values of a struct
*/

// this is just a struct definition
// it is not data, just defining the shape of the data
#[derive(Debug)]
struct Employee {
  name: String,
  email: String,
  avatar: String,
  active: bool,
  start_date: String,
}

struct EmployeeWorkInfo {
  department: String,
  job_role: String,
  line_manager: String
}

fn build_employee_work_info(department: String, job_role: String, line_manager: String) -> EmployeeWorkInfo {
  EmployeeWorkInfo {
    department,
    job_role,
    line_manager
  }
}