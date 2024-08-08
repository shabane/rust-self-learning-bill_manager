use std::fmt::Error;
use std::time;
use std::time::SystemTime;

pub enum BillState {
    Payed,
    UnPayed,
}

pub struct Bill {
    pub name: String,
    pub date: SystemTime,
    pub description: String,
    pub price: i32,
    pub state: BillState,
}

pub struct Bills {
    pub bills: Vec<Bill>,
}

impl Bills {
    pub fn add(
        &mut self,
        name: String,
        date: SystemTime,
        description: String,
        price: i32,
        state: BillState,
    ) {
        self.bills.push(Bill {
            name: name,
            date: date,
            description: description,
            price: price,
            state: state,
        })
    }
    pub fn show(&self) {
        for bill in &self.bills {
            println!("{}", bill.name)
        }
    }

    pub fn remove(&mut self, name: String) {
        let mut index: usize = 0;
        let mut counter: usize = 0;
        {
            for bill in &self.bills {
                if (bill.name == name) {
                    index = counter
                }
                counter += 1;
            }
        }
        &mut self.bills.remove(index);
    }

    pub fn str_to_billstate(text: &str) -> Result<BillState, String> {
        match text.trim().to_lowercase().as_str() {
            "payed" => Ok(BillState::Payed),
            "unpayed" => Ok(BillState::UnPayed),
            _ => Err("states are: Payed or UnPayed".to_string()),
        }
    }
}
