pub mod department;

use department::Department;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Company {
    pub company_departments: Vec<Department>,
}

impl Company {
    pub fn new() -> Self {
        let company_departments: Vec<Department> = Vec::new();
        
        Self {
            company_departments
        }
    }

    pub fn check_if_the_department_already_exists_in_the_company(
        &self,
        department_name: String
    ) -> (bool, usize) {
        for (
            index,
            department
        ) in self.company_departments.clone().into_iter().enumerate() {
            if department.department_name() == department_name {
                return (true, index);
            }
        } 
        
        return (false, 1000);
    }

    pub fn add_a_new_department_to_the_company(
        &mut self,
        department_name: String
    ) {
        if self.check_if_the_department_already_exists_in_the_company(
            department_name.clone()
        ).0 {
            println!(
                "The {} department already exists in the company",
                department_name
            );
        } else {
            let department = Department::new(
                department_name.clone()
            );

            self.company_departments.push(
                department
            );
        
            println!(
                "The {} department was added to the company",
                department_name
            );
        }
    }

    pub fn remove_an_department_to_the_company(
        &mut self,
        department_name: String
    ) {
        let department_verification = self.check_if_the_department_already_exists_in_the_company(
            department_name.clone()
        );

        if department_verification.0 {
            self.company_departments.remove(
                department_verification.1
            );

            println!(
                "The {} department was removed from the company",
                department_name
            );
        } else {
            println!(
                "The {} department does not exist in the company",
                department_name
            );
        }
    }

    pub fn add_a_new_employee_to_the_department(
        &mut self,
        department_name: String,
        employee_name: String
    ) {
        let department_verification = self.check_if_the_department_already_exists_in_the_company(
            department_name.clone()
        );

        if department_verification.0 {
            self.company_departments[
                department_verification.1
            ].add_a_new_employee_to_the_department(
                employee_name.clone()
            );
        } else {
            println!(
                "The {} department does not exist in the company",
                department_name
            );
        }
    }

    pub fn remove_an_employee_from_the_department(
        &mut self,
        department_name: String,
        employee_name: String,
    ) {
        let department_verification = self.check_if_the_department_already_exists_in_the_company(
            department_name.clone()
        );

        if department_verification.0 {
            self.company_departments[
                department_verification.1
            ].remove_an_employee_from_the_department(
                employee_name.clone()
            );
        } else {
            println!(
                "The {} department does not exist in the company",
                department_name
            );
        }
    }

    pub fn get_all_the_employees_from_the_department(
        &self,
        department_name: String
    ) -> Vec<String> {
        let department_employees: Vec<String> = Vec::new();

        let department_verification = self.check_if_the_department_already_exists_in_the_company(
            department_name.clone()
        );

        if department_verification.0 {
            let mut department_employees = self.company_departments[
                department_verification.1
            ].clone().department_employees();

            department_employees.sort_by(
                |a, b| a.cmp(b)
            );

            return department_employees;
        } else {
            println!(
                "The {} department does not exist in the company",
                department_name
            );
    
            return department_employees;
        }  
    }

    pub fn get_all_departments_names(
        &self
    ) -> Vec<String> {
        let mut all_departments_names: Vec<String> = Vec::new();

        for department in self.company_departments.clone() {
            all_departments_names.push(
                department.department_name()
            );
        }

        all_departments_names.sort_by(
            |a, b| a.cmp(b)
        );

        all_departments_names
    }

    pub fn company_departments(
        &self
    ) -> Vec<HashMap<String, Vec<String>>> {
        let mut company: Vec<HashMap<String, Vec<String>>> = Vec::new();

        for department in self.company_departments.clone() {
            company.push(
                department.get_department()
            );
        }

        company
    }
}