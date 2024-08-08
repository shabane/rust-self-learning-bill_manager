use crate::bill::{Bill, BillState, Bills};
use std::io::Write;
use std::time::SystemTime;

mod bill;

fn menu() {
    let menu = r#"
    add: to add a new bill
    show: to list all the bills
    remove [name]: to rmove one of the bills from the list
    "#;
    println!("{}", menu.to_string());
}

fn parse_input(input: &str, bill: &mut Bills) {
    match input.trim().to_lowercase().as_str() {
        "add" => {
            print!("Name: ");
            std::io::stdout().flush().unwrap();
            let mut name_buffer = String::new();
            std::io::stdin()
                .read_line(&mut name_buffer)
                .expect("TODO: panic message");

            print!("Description: ");
            std::io::stdout().flush();
            let mut decsription_buffer = String::new();
            std::io::stdin().read_line(&mut decsription_buffer);

            print!("Price: ");
            std::io::stdout().flush();
            let mut price_buffer = String::new();
            std::io::stdin().read_line(&mut price_buffer);
            let mut price_buffer: i32 = match price_buffer.trim().parse() {
                Ok(data) => data,
                Err(err) => {
                    println!("error in parsing int! -> {:?}", err);
                    return;
                }
            };

            print!("State: ");
            std::io::stdout().flush();
            let mut state_buffer = String::new();
            std::io::stdin().read_line(&mut state_buffer);
            let state_buffer = bill::Bills::str_to_billstate(&state_buffer);

            bill.add(
                name_buffer,
                SystemTime::now(),
                decsription_buffer,
                price_buffer,
                state_buffer.unwrap_or(bill::BillState::UnPayed),
            );
        }
        "show" => {
            &bill.show();
        }
        "remove" => {
            &bill.show();
            print!("enter name: ");
            std::io::stdout().flush().unwrap();
            let mut name_buffer = String::new();
            std::io::stdin().read_line(&mut name_buffer);
            &bill.remove(name_buffer);
        },
        "menu" => menu(),
        _ => println!("This directive does not supported!"),
    }
}

fn main() {
    let mut bills = Bills { bills: Vec::new() };
    bills.add(
        String::from("this"),
        SystemTime::now(),
        String::from("title"),
        12,
        BillState::UnPayed,
    );

    menu();
    let mut input = String::new();
    loop {
        print!(":");
        std::io::stdout().flush().expect("wtf!");
        std::io::stdin()
            .read_line(&mut input)
            .expect("error in reading from stdin!");
        parse_input(&input, &mut bills);
        input.clear()
    }
}
