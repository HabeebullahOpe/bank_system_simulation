use chrono::{DateTime, Utc};
use std::{io, sync::mpsc::Receiver};

#[derive(Debug)]
// enum TransactionType {
//     Credit,
//     Debit,
// }
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer { to_account_id: i32 },
}

#[derive(Debug)]
struct Transaction {
    amount: f64,
    transaction_type: TransactionType,
    timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn new(amount: f64, transaction_type: TransactionType) -> Self {
        Self {
            amount,
            transaction_type,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug)]
struct Account {
    id: i32,
    account_holder: String,
    balance: f64,
    transactions: Vec<Transaction>
}

impl Account {
    pub fn new(id: i32, account_holder: String) -> Self {
        Self {
            id,
            account_holder,
            balance: 0.0,
            transactions: Vec::new(),
        }
    }
    pub fn deposit(&mut self, amount: f64) {
        if amount <= 0.0 {
            println!("Deposit amount must be positive");
            return;
        }
        self.balance += amount;

        let transaction = Transaction {
            amount,
            transaction_type: TransactionType::Deposit,
            timestamp: Utc::now(),
        };

        self.transactions.push(transaction);
        println!("Deposit of #{:.2} successful", amount);
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount <= 0.0 {
            println!("Withdraw amout must be positive");
            return;
        }
        self.balance -= amount;

        let transaction = Transaction {
            amount,
            transaction_type: TransactionType::Withdrawal,
            timestamp: Utc::now(),
        };

        self.transactions.push(transaction);
        println!("Withdrawal of #{:.2} was successful", amount);
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
    next_account_id: i32, //should assign unique ids to new users
}

impl Bank {
    pub fn new(name: String) -> Self {
        Self {
            name,
            accounts: Vec::new(),
            next_account_id: 0,
        }
    }
    pub fn create_account(&mut self) {
        self.next_account_id += 1;

        let mut name_input = String::new();

        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read line");
        let trimmed_name = name_input.trim().to_string();
        let account = Account::new(self.next_account_id, trimmed_name);
        self.accounts.push(account);

        println!("Account created with ID: {}", self.next_account_id);
        self.next_account_id += 1;
    }

    pub fn transfer(&mut self) {
        println!("Enter sender account ID:");
        let mut sender_input = String::new();
        io::stdin()
            .read_line(&mut sender_input)
            .expect("Failed to read sender ID");
        let sender_id: i32 = match sender_input.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid sender ID.");
                return;
            }
        };

        println!("Enter reciever account ID:");
        let mut receiver_input = String::new();
        io::stdin()
            .read_line(&mut receiver_input)
            .expect("Failed to read Reciever ID.");
        let receiver_id: i32 = match receiver_input.trim().parse() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid reciever ID.");
                return;
            }
        }

    }

}

fn main() {
    println!("Hello, world!");
}
