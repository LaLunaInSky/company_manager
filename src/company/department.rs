use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Department {
    department_name: String,
    department_employees: Vec<String>
}

impl Department {
    pub fn new(
        department_name: String
    ) -> Self {
        let department_employees: Vec<String> = Vec::new();

        Self {
            department_name,
            department_employees
        }
    }

    pub fn add_a_new_employee_to_the_department(
        &mut self,
        employee_name: String
    ) {
        if self.department_employees.contains(&employee_name) {
            println!(
                "The employee {} is already in {}",
                employee_name,
                self.department_name
            );
        } else {
            self.department_employees.push(
                employee_name.clone()
            );

            println!(
                "The employee {} was added to the {} team",
                employee_name,
                self.department_name
            );
        }
    }

    pub fn remove_an_employee_from_the_department(
        &mut self,
        employee_name: String
    ) {
        if self.department_employees.contains(&employee_name) {
            let mut index_of_employee: usize = 10000;

            for (
                index,
                employee
            ) in self.department_employees.iter().enumerate() {
                if *employee == employee_name {
                    index_of_employee = index;
                }
            }

            self.department_employees.remove(index_of_employee);

            println!(
                "The employee {} was removed from {}",
                employee_name,
                self.department_name
            );
        } else {
            println!(
                "{} is an {} employee",
                employee_name,
                self.department_name
            );
        }
    }

    pub fn department_employees(
        &self
    ) -> Vec<String> {
        self.department_employees.clone()
    }

    pub fn department_name(
        &self
    ) -> String {
        self.department_name.clone()
    }

    pub fn get_department(
        &self
    ) -> HashMap<String, Vec<String>> {
        let mut department: HashMap<String, Vec<String>> = HashMap::new();

        department.entry(
            self.department_name.clone()
        ).or_insert(
            self.department_employees.clone()
        );

        department
    }
}