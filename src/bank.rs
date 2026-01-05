use std::{collections::HashMap, io::{self, Write}};
use colored::*;

// Define custom error types for our banking application
#[derive(Debug, PartialEq)]
enum BankingError {
    InsufficientFunds,
    AccountNotFound,
    InvalidAmount,
    TransferError(String),
    AccountError,
}

// Implement Display trait for BankingError to make it printable
impl std::fmt::Display for BankingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BankingError::InsufficientFunds => write!(f, "Insufficient funds"),
            BankingError::AccountNotFound => write!(f, "Account not found"),
            BankingError::InvalidAmount => write!(f, "Invalid amount"),
            BankingError::AccountError => write!(f, "Account creation went wrong"),
            BankingError::TransferError(msg) => write!(f, "Transfer error: {}", msg),
        }
    }
}

// Implement the std::error::Error trait for BankingError
impl std::error::Error for BankingError {}

// Define the Account struct
#[derive(Debug, Clone)]
struct Account {
    id: String,
    balance: f64,
}

// Define the Bank struct that will manage accounts
struct Bank {
    accounts: HashMap<String, Account>,
}

impl Bank {
    // Create a new bank instance
    fn new() -> Self {
        // TODO: Implement the new function that initializes an empty HashMap for accounts
        Bank {
            accounts: HashMap::new(),
        }
    }

    // Create a new account with an initial balance
    fn create_account(&mut self, id: String, initial_balance: f64) -> Result<(), BankingError> {
        if self.accounts.contains_key(&id) {
            return Err(BankingError::AccountError);
        }
        if initial_balance < 0.0 {
            return Err(BankingError::InvalidAmount);
        }

        self.accounts.insert(
            id.clone(),
            Account {
                id,
                balance: initial_balance,
            },
        );
        Ok(())
    }

    // Get the balance of an account
    fn get_balance(&self, account_id: &str) -> Result<f64, BankingError> {
        // Look up the account by ID and return its balance or AccountNotFound
        if let Some(account) = self.accounts.get(account_id) {
            Ok(account.balance)
        } else {
            Err(BankingError::AccountNotFound)
        }
    }

    // Deposit money into an account
    fn deposit(&mut self, account_id: &str, amount: f64) -> Result<(), BankingError> {
        // TODO: Implement deposit function

        if amount < 0.0 {
            return Err(BankingError::InvalidAmount);
        }

        if let Some(account) = self.accounts.get_mut(account_id) {
            account.balance += amount;
            Ok(())
        } else {
            Err(BankingError::AccountNotFound)
        }
    }

    // Withdraw money from an account
    fn withdraw(&mut self, account_id: &str, amount: f64) -> Result<(), BankingError> {
        // TODO: Implement withdraw function
        if amount < 0.0 {
            return Err(BankingError::InvalidAmount);
        }

        if let Some(account) = self.accounts.get_mut(account_id) {
            if account.balance < amount {
                return Err(BankingError::InsufficientFunds);
            }
            account.balance -= amount;
            Ok(())
        } else {
            Err(BankingError::AccountNotFound)
        }
    }

    // Transfer money between accounts
    fn transfer(&mut self, from_id: &str, to_id: &str, amount: f64) -> Result<(), BankingError> {
        // TODO: Implement transfer function

        if amount < 0.0 {
            return Err(BankingError::InvalidAmount);
        }

        let from_balance = self.accounts.get(from_id).ok_or(BankingError::AccountNotFound)?.balance;
        if from_balance < amount {
            return Err(BankingError::InsufficientFunds);
        }

        if !self.accounts.contains_key(to_id) {
            return Err(BankingError::AccountNotFound);
        }

        self.accounts.get_mut(from_id).unwrap().balance -= amount;
        self.accounts.get_mut(to_id).unwrap().balance += amount;
        Ok(())
    }

    // List all account IDs
    fn list_accounts(&self) -> Vec<String> {
        let mut result = vec![];
        // TODO: Implement list_accounts function
        for account in &self.accounts {
            result.push(account.0.clone());
        }

        return result;
        // - Return a vector containing all account IDs
    }
}

// Function to display the main menu
fn show_menu() {
    println!("\n{}", "Banking System".cyan());
    println!("{}\n", "-------------".cyan());
    println!("  1. Create Account");
    println!("  2. Check Balance");
    println!("  3. Deposit");
    println!("  4. Withdraw");
    println!("  5. Transfer");
    println!("  6. List Accounts");
    println!("  7. Back to Main Menu\n");
    print!("Select option: ");
    io::stdout().flush().ok();
}

// Function to handle user input and execute banking operations
fn handle_user_input(bank: &mut Bank, choice: &str) {

    let mut input = String::new();
    match choice.trim() {
        "1" => {
            // Create Account
            print!("Account ID: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let id = input.trim().to_string();

            print!("Initial balance: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let initial_balance: f64 = match input.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Invalid amount".red());
                    return;
                }
            };

            match bank.create_account(id, initial_balance) {
                Ok(_) => println!("{}", "Account created".green()),
                Err(e) => println!("{}", format!("Error: {}", e).red()),
            }
        }
        "2" => {
            // Get Balance
            print!("Account ID: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let id = input.trim();
            match bank.get_balance(id) {
                Ok(bal) => println!("Balance: {}", format!("${:.2}", bal).green()),
                Err(e) => println!("{}", format!("Error: {}", e).red()),
            }
        }
        "3" => {
            // Deposit
            print!("Account ID: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let id = input.trim().to_string();

            print!("Amount: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let amount: f64 = match input.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Invalid amount".red());
                    return;
                }
            };

            match bank.deposit(&id, amount) {
                Ok(_) => println!("{}", "Deposit completed".green()),
                Err(e) => println!("{}", format!("Error: {}", e).red()),
            }
        }
        "4" => {
            // Withdraw
            print!("Account ID: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let id = input.trim().to_string();

            print!("Amount: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let amount: f64 = match input.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Invalid amount".red());
                    return;
                }
            };

            match bank.withdraw(&id, amount) {
                Ok(_) => println!("{}", "Withdrawal completed".green()),
                Err(e) => println!("{}", format!("Error: {}", e).red()),
            }
        }
        "5" => {
            // Transfer
            print!("From account: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let from_id = input.trim().to_string();

            print!("To account: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let to_id = input.trim().to_string();

            print!("Amount: ");
            io::stdout().flush().ok();
            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}", "Error reading input".red());
                return;
            }
            let amount: f64 = match input.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Invalid amount".red());
                    return;
                }
            };

            match bank.transfer(&from_id, &to_id, amount) {
                Ok(_) => println!("{}", "Transfer completed".green()),
                Err(e) => println!("{}", format!("Error: {}", e).red()),
            }
        }
        "6" => {
            // List Accounts
            let ids = bank.list_accounts();
            if ids.is_empty() {
                println!("No accounts found");
            } else {
                println!("\nAccounts:");
                for id in ids {
                    println!("  - {}", id);
                }
            }
        }
        "7" => {
            // Exit
            println!("");
        }
        _ => println!("{}", "Invalid option".red()),
    }
}

pub fn run() {
    let mut bank = Bank::new();
    loop {
        show_menu();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "❌ Failed to read input.".red());
            continue;
        }
        let choice = input.trim();
        if choice == "7" {
            break;
        }
        handle_user_input(&mut bank, choice);
    }
}

#[allow(dead_code)]
fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let mut bank = Bank::new();
        assert!(bank.create_account("test".to_string(), 100.0).is_ok());
        assert!(bank.create_account("test".to_string(), 50.0).is_err()); // duplicate
        assert!(bank.create_account("test2".to_string(), -10.0).is_err()); // negative
    }

    #[test]
    fn test_get_balance() {
        let mut bank = Bank::new();
        bank.create_account("test".to_string(), 100.0).unwrap();
        assert_eq!(bank.get_balance("test"), Ok(100.0));
        assert!(bank.get_balance("nonexistent").is_err());
    }

    #[test]
    fn test_deposit() {
        let mut bank = Bank::new();
        bank.create_account("test".to_string(), 100.0).unwrap();
        assert!(bank.deposit("test", 50.0).is_ok());
        assert_eq!(bank.get_balance("test"), Ok(150.0));
        assert!(bank.deposit("test", -10.0).is_err());
        assert!(bank.deposit("nonexistent", 50.0).is_err());
    }

    #[test]
    fn test_withdraw() {
        let mut bank = Bank::new();
        bank.create_account("test".to_string(), 100.0).unwrap();
        assert!(bank.withdraw("test", 50.0).is_ok());
        assert_eq!(bank.get_balance("test"), Ok(50.0));
        assert!(bank.withdraw("test", 100.0).is_err()); // insufficient
        assert!(bank.withdraw("test", -10.0).is_err());
        assert!(bank.withdraw("nonexistent", 50.0).is_err());
    }

    #[test]
    fn test_transfer() {
        let mut bank = Bank::new();
        bank.create_account("from".to_string(), 100.0).unwrap();
        bank.create_account("to".to_string(), 50.0).unwrap();
        assert!(bank.transfer("from", "to", 30.0).is_ok());
        assert_eq!(bank.get_balance("from"), Ok(70.0));
        assert_eq!(bank.get_balance("to"), Ok(80.0));
        assert!(bank.transfer("from", "to", 100.0).is_err()); // insufficient
        assert!(bank.transfer("nonexistent", "to", 10.0).is_err());
        assert!(bank.transfer("from", "nonexistent", 10.0).is_err());
    }

    #[test]
    fn test_list_accounts() {
        let mut bank = Bank::new();
        bank.create_account("a".to_string(), 0.0).unwrap();
        bank.create_account("b".to_string(), 0.0).unwrap();
        let mut ids = bank.list_accounts();
        ids.sort();
        assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
    }
}