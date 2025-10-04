use chrono::{DateTime, Utc};
use std::fmt;
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
    balance_after: f64,
}

impl Transaction {
    pub fn new(amount: f64, transaction_type: TransactionType, balance_after: f64) -> Self {
        Self {
            amount,
            transaction_type,
            timestamp: Utc::now(),
            balance_after,
        }
    }
}

#[derive(Debug)]
struct Account {
    id: i32,
    account_holder: String,
    balance: f64,
    transactions: Vec<Transaction>,
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
    pub fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        self.balance += amount;

        let transaction = Transaction {
            amount,
            transaction_type: TransactionType::Deposit,
            timestamp: Utc::now(),
            balance_after: self.balance,
        };

        self.transactions.push(transaction);
        println!("Deposit of #{:.2} successful", amount);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        self.balance -= amount;

        let transaction = Transaction {
            amount,
            transaction_type: TransactionType::Withdrawal,
            timestamp: Utc::now(),
            balance_after: self.balance,
        };

        self.transactions.push(transaction);
        println!("Withdrawal of #{:.2} was successful", amount);
        Ok(())
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn display_transactions(&self) {
        for transaction in &self.transactions {
            let transact_date = transaction.timestamp.format("%Y-%m-%d");
            let transaction_type = match transaction.transaction_type {
                TransactionType::Deposit => "DEPOSIT",
                TransactionType::Withdrawal => "WITHDRAWAL",
                TransactionType::Transfer { to_account_id } => "TRANSFER",
            };
            println!(
                "{}: {} ${:.2} -> Balance: ${:.2}",
                transact_date, transaction_type, transaction.amount, transaction.balance_after
            );
        }
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
    }

    pub fn transfer(&mut self) -> Result<(), BankError> {
        println!("Enter sender account ID:");
        let mut sender_input = String::new();
        io::stdin()
            .read_line(&mut sender_input)
            .expect("Failed to read sender ID");
        let sender_id: i32 = match sender_input.trim().parse() {
            Ok(id) => id,
            Err(_) => return Err(BankError::AccountNotFound),
        };

        println!("Enter receiver account ID:");
        let mut receiver_input = String::new();
        io::stdin()
            .read_line(&mut receiver_input)
            .expect("Failed to read receiver ID");
        let receiver_id: i32 = match receiver_input.trim().parse() {
            Ok(id) => id,
            Err(_) => return Err(BankError::AccountNotFound),
        };

        println!("Enter amount to transfer:");
        let mut amount_input = String::new();
        io::stdin()
            .read_line(&mut amount_input)
            .expect("Failed to read amount");
        let amount: f64 = match amount_input.trim().parse() {
            Ok(a) if a > 0.0 => a,
            _ => return Err(BankError::InvalidAmount),
        };

        let sender_index = self.accounts.iter().position(|acc| acc.id == sender_id);
        let receiver_index = self.accounts.iter().position(|acc| acc.id == receiver_id);

        match (sender_index, receiver_index) {
            (Some(si), Some(ri)) => {
                if si == ri {
                    return Err(BankError::InvalidAmount); // Can't transfer to same account
                }

                let (s, r) = {
                    let (left, right) = self.accounts.split_at_mut(std::cmp::max(si, ri));
                    if si < ri {
                        (&mut left[si], &mut right[0])
                    } else {
                        (&mut right[0], &mut left[ri])
                    }
                };

                if s.balance < amount {
                    return Err(BankError::InsufficientFunds);
                }

                s.balance -= amount;
                r.balance += amount;

                let timestamp = Utc::now();

                s.transactions.push(Transaction {
                    amount,
                    transaction_type: TransactionType::Transfer {
                        to_account_id: receiver_id,
                    },
                    timestamp,
                    balance_after: s.balance,
                });

                r.transactions.push(Transaction {
                    amount,
                    transaction_type: TransactionType::Deposit,
                    timestamp,
                    balance_after: r.balance,
                });

                println!(
                    "Transferred ₦{:.2} from Account #{} to Account #{}",
                    amount, sender_id, receiver_id
                );

                Ok(())
            }
            _ => Err(BankError::AccountNotFound),
        }
    }
}

#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    AccountNotFound,
    InvalidAmount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds => {
                write!(f, "Insufficient Balance to complete this transaction")
            }
            BankError::AccountNotFound => write!(f, "Account not found"),
            BankError::InvalidAmount => write!(f, "Invalid amount"),
        }
    }
}

fn main() {
    println!("== Welcome to the Banking System Simulatiom! ==");

    let mut bank = Bank::new("Rustacean Bank".to_string());

    loop {
        println!("\n What would you like to process?");
        println!("1. Create Account");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Transfer");
        println!("5. Check Acc Balance");
        println!("6. Display Transactions");
        println!("7. Quit Process");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "1" => bank.create_account(),
            "2" => {
                println!("Enter account ID for deposit:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read account ID");
                let account_id: i32 = match id_input.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid account ID.");
                        return;
                    }
                };

                println!("Enter amount to deposit:");
                let mut amount_input = String::new();
                io::stdin()
                    .read_line(&mut amount_input)
                    .expect("Failed to read amount");
                let amount: f64 = match amount_input.trim().parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Invalid amount.");
                        return;
                    }
                };

                match bank.accounts.iter_mut().find(|acc| acc.id == account_id) {
                    Some(account) => {
                        if let Err(e) = account.deposit(amount) {
                            println!("Error: {}", e);
                        }
                    }
                    None => println!("Account not found."),
                }
            }
            "3" => {
                println!("Enter account ID for withdrawal:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read account ID");
                let account_id: i32 = match id_input.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid account ID.");
                        return;
                    }
                };
                println!("Enter amount to withdraw");
                let mut amount_input = String::new();
                io::stdin()
                    .read_line(&mut amount_input)
                    .expect("Failed to read amount");
                let amount: f64 = match amount_input.trim().parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Invalid amount.");
                        return;
                    }
                };

                match bank.accounts.iter_mut().find(|acc| acc.id == account_id) {
                    Some(account) => {
                        if let Err(e) = account.withdraw(amount) {
                            println!("Error: {}", e);
                        }
                    }
                    None => println!("Account not found."),
                }
            }
            "4" => {
                if let Err(e) = bank.transfer() {
                    println!("Error: {}", e);
                }
            }
            "5" => {
                println!("Enter Account ID to check balance:");
                let mut input_id = String::new();
                io::stdin()
                    .read_line(&mut input_id)
                    .expect("Failed to read input ID");
                let account_id = match input_id.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid account ID.");
                        return;
                    }
                };

                match bank.accounts.iter().find(|acc| acc.id == account_id) {
                    Some(account) => {
                        println!("\n==> Current balance: ₦{:.2}", account.get_balance());
                    }
                    None => println!("Account not found."),
                }
            }

            "6" => {
                println!("Enter Account ID to view transaction history:");
                let mut input_id = String::new();
                io::stdin()
                    .read_line(&mut input_id)
                    .expect("Failed to read input ID");
                let account_id = match input_id.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid account ID.");
                        return;
                    }
                };

                match bank.accounts.iter().find(|acc| acc.id == account_id) {
                    Some(account) => {
                        println!("\n== Transaction History for Account #{} ==", account.id);
                        account.display_transactions();
                    }
                    None => println!("Account not found."),
                }
            }

            "7" => {
                println!("Thank you for banking with us, See ya Next time 👋");
                break;
            }

            _ => println!("Invalid choice! Please enter btw 1-7."),
        }
    }
}
