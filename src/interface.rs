use crate::company::Company;
use std::{
    process::Command,
    io::stdin
};

#[derive(Debug)]
pub struct Interface {
    company: Company
}

impl Interface {
    pub fn new() -> Self {
        let mut company = Company::new();

        company.add_a_new_department_to_the_company(
            String::from("Sales")
        );

        company.add_a_new_department_to_the_company(
            String::from("Engineering")
        );

        company.add_a_new_employee_to_the_department(
            String::from("Engineering"),
            String::from("Sally")
        );

        company.add_a_new_employee_to_the_department(
            String::from("Sales"),
            String::from("Amir")
        );

        Self {
            company
        }
    }

    fn clean_terminal(
        &mut self
    ) {
        Command::new("clear").status().unwrap();
    }

    fn get_head(
        &mut self
    ) {
        self.clean_terminal();

        println!(
            "{:-^20}",
            " Your Company "
        );
    }

    pub fn show_the_interface(
        &mut self
    ) {
        self.main_screen_menu(
            String::from("")
        );
        // self.menu_screen_add_a_new_department();
        // self.menu_screen_select_a_department_add_a_new_employee(
        //     String::from("Sales")
        // );
    }

    fn get_the_input_of_an_interger(
        &mut self,
        limit_number: u8,
        text_to_show: String,
    ) -> u8 {
        loop {
            self.get_head();

            println!(
                "{text_to_show}"
            );

            println!(
                "Which option do you choose?"
            );

            let mut input = String::new();

            stdin().read_line(&mut input).unwrap();

            let limit_number: u8 = limit_number;

            match input.trim().parse::<u8>() {
                Ok(number) => {
                    if number <= limit_number {
                        return number;
                    } else {
                        println!(
                            "\nEnter only 1 through {}!\n",
                            limit_number
                        );
                    }
                }
                Err(_) => println!(
                    "\nEnter only positive intergers!\n"
                ),
            }
        }
    }

    fn get_the_input_of_an_string(
        &mut self,
        text_to_show: String,
        input_call_text: String,
    ) -> String {
        let mut input_return_text = String::new();

        loop {
            self.get_head();

            println!(
                "{text_to_show}"
            );

            println!(
                "{input_return_text}"
            );

            input_return_text = String::new();

            println!(
                "{}",
                input_call_text
            );

            let mut input = String::new();

            stdin().read_line(&mut input).unwrap();

            // Step 1
            let input = input.trim();
            let input_chars: Vec<char> = input.chars().collect();
            let mut first_word_in_input = String::new();

            for char_value in input_chars.clone().into_iter() {
                if char_value == ' ' {
                    break;
                }

                first_word_in_input.push(char_value);
            }

            // Step 2
            if first_word_in_input.len() > 1 {
                let first_word_in_input: Vec<char> = first_word_in_input.chars().collect();

                let length_to_first_word_in_input = first_word_in_input.len();

                let mut letters_of_first_word_in_input: Vec<char> = Vec::new();

                for char in first_word_in_input.clone().into_iter() {
                    if char.is_ascii_digit() {
                        input_return_text = format!(
                            "\n{}\n",
                            "Names do not contain numbers!"
                        );

                        break;
                    }
                    
                    letters_of_first_word_in_input.push(char);
                }

                if letters_of_first_word_in_input.len() == length_to_first_word_in_input {
                    let mut word = String::new();

                    for (
                        index_to_char,
                        char_value
                    ) in letters_of_first_word_in_input.clone().into_iter().enumerate() {
                        if index_to_char == 0 {
                            word += &char_value.to_uppercase().to_string();
                        } else {
                            word += &char_value.to_lowercase().to_string();
                        }

                    }

                    return word;
                }
            } else {
                // len equals 1
                let first_word_in_input: Vec<char> = first_word_in_input.chars().collect();

                if first_word_in_input.len() > 0 {
                    if first_word_in_input[0].is_ascii_digit() {
                        match first_word_in_input[0] {
                            '1' => return String::from(
                                first_word_in_input[0]
                            ),
                            _ => input_return_text = format!(
                                "\n{}\n",
                                "You only entered one letter, in this\ncase, only the number 1 is accepted!"
                            ),
                        }
                    } else {
                        input_return_text = format!(
                            "\n{}\n",
                            "You only entered one letter, in this\ncase, only the number 1 is accepted!"
                        );
                    }
                } else {
                    input_return_text = format!(
                        "\n{}\n",
                        "Please type something!"
                    );
                }
            }
        }
    }

    fn main_screen_menu(
        &mut self,
        input_return_text: String
    ) {
        let text_to_show = format!(
            "\n{}[ 1 ] Show All Departments\n[ 2 ] Add a New Department\n[ 3 ] Remove a Department\n[ 4 ] Select a Department\n[ 5 ] Close the program\n",
            input_return_text
        );

        let number_of_option: u8 = self.get_the_input_of_an_interger(
            5,
            text_to_show.clone()
        );

        match number_of_option {
            1 => self.menu_screen_show_all_departments(),
            2 => self.menu_screen_add_a_new_department(),
            3 => self.menu_screen_remove_a_department(),
            4 => {
                self.menu_screen_select_a_department()
            },
            _ => (),
        }
    }

    fn menu_screen_show_all_departments(
        &mut self
    ) {
        let all_departments = self.company.get_all_departments_names().clone(); 

        let mut departments = String::new();

        if all_departments.len() > 0 {
            for department in all_departments.into_iter() {
                let text = format!(
                    " ° {}\n",
                    department
                );
    
                departments += &text;
            }
        } else {
            departments.push_str(
                "No Departments Found!\n"
            );
        }


        let text_to_show = format!(
            "\n{:^20}\n\n{}\n[ 1 ] To Go Back\n",
            "Your Departments",
            departments
        );

        let number_of_option = self.get_the_input_of_an_interger(
            1,
            text_to_show.clone()
        );

        match number_of_option {
            1 => self.main_screen_menu(
                String::from("")
            ),
            _ => (),
        }
    }

    fn menu_screen_add_a_new_department(
        &mut self
    ) {
        let text_from_this_menu = format!(
            "\n{:^20}\n\n[ 1 ] To Go Back",
            "Add a New Department"
        );

        let input_call_text = String::from(
            "What is the name of the new department?"
        );

        let call_return = self.get_the_input_of_an_string(
            text_from_this_menu.clone(),
            input_call_text.clone(),
        );
        
        if call_return != "1" {
            self.company.add_a_new_department_to_the_company(
                call_return.clone()
            );

            let input_return_text = format!(
                "The {} department was created!\n\n",
                call_return
            );

            self.main_screen_menu(
                input_return_text
            );
        } else {
            self.main_screen_menu(
                String::from("")
            );
        }
    }

    fn menu_screen_remove_a_department(
        &mut self
    ) {
        let all_departments = self.company.get_all_departments_names().clone();

        let total_of_options_in_menu = all_departments.len() + 1;

        let mut departments = String::new();

        if all_departments.len() > 0 {
            for (
                index,
                department
            ) in all_departments.clone().into_iter().enumerate() {
                let text = format!(
                    "[ {} ] {}\n",
                    index + 1,
                    department
                );
    
                departments += &text;
            }
        } else {
            departments.push_str(
                "No Departments Found!\n\n"
            )
        }

        let text_to_show = format!(
            "\n{:^20}\n\n{}[ {} ] To Go Back\n",
            "Remove a Department",
            departments,
            total_of_options_in_menu
        );

        let number_of_option = self.get_the_input_of_an_interger(
            total_of_options_in_menu as u8,
            text_to_show.clone()
        );

        if usize::from(number_of_option) < total_of_options_in_menu {
            self.company.remove_an_department_to_the_company(
                all_departments[
                    usize::from(number_of_option - 1)
                ].clone()
            );

            let input_return_text = format!(
                "The {} department has\nbeen removed!\n\n",
                all_departments[
                    usize::from(number_of_option - 1)
                ].clone()
            );
            
            self.main_screen_menu(
                input_return_text.clone()
            );
        } else {
            self.main_screen_menu(
                String::from("")
            );
        }
    }

    fn menu_screen_select_a_department(
        &mut self
    ) {
        let all_departments = self.company.get_all_departments_names().clone();

        let total_of_options_in_menu = all_departments.len() + 1;

        let mut departments = String::new();

        if all_departments.len() > 0 {
            for (
                index,
                department
            ) in all_departments.clone().into_iter().enumerate() {
                let text = format!(
                    "[ {} ] {}\n",
                    index + 1,
                    department
                );
    
                departments += &text;
            }
        } else {
            departments.push_str(
                "No Departments Found!\n\n"
            );
        }

        let text_to_show = format!(
            "\n{:^20}\n\n{}[ {} ] To Go Back\n",
            "Select a Department",
            departments,
            total_of_options_in_menu
        );

        let number_of_option = self.get_the_input_of_an_interger(
            total_of_options_in_menu as u8,
            text_to_show.clone()
        );

        if usize::from(number_of_option) < total_of_options_in_menu {
            self.menu_screen_selected_department(
                all_departments[
                    usize::from(
                        number_of_option - 1
                    )
                ].clone(),
                String::from("")
            );
        } else {
            self.main_screen_menu(
                String::from("")
            );
        }
    }

    fn menu_screen_selected_department(
        &mut self,
        department_name: String,
        input_return_text: String,
    ) {
        let name_string = format!(
            "{} Department",
            department_name
        );

        let text_to_show = format!(
            "\n{:^20}\n{}\n[ 1 ] Show All Employees\n[ 2 ] Add a New Employee\n[ 3 ] Remove a Employee\n[ 4 ] To Go Back\n",
            name_string,
            input_return_text
        );

        let number_of_option = self.get_the_input_of_an_interger(
            4,
            text_to_show.clone()
        );

        match number_of_option {
            1 => self.menu_screen_selected_department_show_all_employees(
                department_name.clone()
            ),
            2 => self.menu_screen_select_a_department_add_a_new_employee(
                department_name.clone()
            ),
            3 => self.menu_screen_select_a_department_remove_a_employee(
                department_name.clone()
            ),
            4 => self.menu_screen_select_a_department(),
            _ => (),
        }
    }

    fn menu_screen_selected_department_show_all_employees(
        &mut self,
        department_name: String,
    ) {
        let name_string = format!(
            "{} Department",
            department_name
        );

        let all_employees = self.company.get_all_the_employees_from_the_department(
            department_name.clone()
        ).clone();

        let mut employees = String::new();

        if all_employees.len() > 0 {
            for employee in all_employees.into_iter() {
                let text = format!(
                    " ° {}\n",
                    employee
                );
    
                employees += &text;
            }
        } else {
            employees.push_str(
                "No Employees Found!\n"
            );
        }

        let text_to_show = format!(
            "\n{:^20}\n\n{}\n[ 1 ] To Go Back\n",
            name_string,
            employees
        );

        let number_of_option = self.get_the_input_of_an_interger(
            1,
            text_to_show.clone()
        );

        match number_of_option {
            1 => self.menu_screen_selected_department(
                department_name.clone(),
                String::from("")
            ),
            _ => (),
        }
    }

    fn menu_screen_select_a_department_add_a_new_employee(
        &mut self,
        department_name: String,
    ) {
        let name_string = format!(
            "{} Department",
            department_name.clone()
        );

        let text_to_show = format!(
            "\n{:^20}\n{:^20}\n\n[ 1 ] To Go Back",
            name_string,
            "Add a New Employee"
        );

        let input_call_text = String::from(
            "What is the name of the new employee?"
        );

        let call_return = self.get_the_input_of_an_string(
            text_to_show.clone(),
            input_call_text.clone()
        );

        if call_return != "1" {
            self.company.add_a_new_employee_to_the_department(
                department_name.clone(),
                call_return.clone()
            );

            let input_return_text = format!(
                "\n{} was added to the department\n",
                call_return
            );
            
            self.menu_screen_selected_department(
                department_name.clone(),
                input_return_text.clone()
            );
        } else {
            self.menu_screen_selected_department(
                department_name.clone(),
                String::from("")
            );
        }
    }

    fn menu_screen_select_a_department_remove_a_employee(
        &mut self,
        department_name: String,
    ) {
        let name_string = format!(
            "{} Department",
            department_name
        );

        let all_employees = self.company.get_all_the_employees_from_the_department(
            department_name.clone()
        );

        let total_of_options_in_menu = all_employees.len() + 1;

        let mut employees = String::new();

        if all_employees.len() > 0 {
            for (
                index,
                employee
            ) in all_employees.clone().into_iter().enumerate() {
                let text = format!(
                    "[ {} ] {}\n",
                    index + 1,
                    employee
                );
    
                employees += &text;
            }
        } else {
            employees.push_str(
                "No Employees Found!\n\n"
            );
        }

        let text_to_show = format!(
            "\n{:^20}\n{:^20}\n\n{}[ {} ] To Go Back\n",
            name_string,
            "Remove a Employee",
            employees,
            total_of_options_in_menu
        );

        let number_of_option = self.get_the_input_of_an_interger(
            total_of_options_in_menu as u8,
            text_to_show.clone()
        );

        if usize::from(number_of_option) < total_of_options_in_menu {
            self.company.remove_an_employee_from_the_department(
                department_name.clone(),
                all_employees[
                    usize::from(number_of_option - 1)
                ].clone()
            );

            let input_return_text = format!(
                "\n{} was removed for\nthe department!\n",
                all_employees[
                    usize::from(number_of_option - 1)
                ].clone()
            );

            self.menu_screen_selected_department(
                department_name.clone(),
                input_return_text.clone()
            );
        } else {
            self.menu_screen_selected_department(
                department_name.clone(),
                String::from("")
            );
        }
    }
}