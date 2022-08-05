struct BankAccount {
    balance: i32,
    verified: bool,
}

// & sign before argument means that the argument is a reference and function can not take ownership of it

fn print_account_balance(account: &BankAccount) {
    println!("Balance: {}", account.balance);
}

fn print_account_veritification_status(account: &BankAccount) {
    println!("Verification Status: {}", account.verified);
}

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false),
    }
}

fn main() {
    let my_account = BankAccount {
        balance: 0,
        verified: false,
    };

    let verification_status = is_verified(&my_account).expect("Unable to unwrap the result");

    // calling both functions with & sign before each parameter
    print_account_balance(&my_account);
    print_account_veritification_status(&my_account);

    println!("Printing: {:?}", verification_status);
}
