#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}




fn set_account_id(account: &mut Account, id: u32) {
    account.id = id;
}
 
fn main() {
    let mut account = Account::new(1, String::from("me"));
    
 
    set_account_id(&mut account, 3);
 
    println!("Account id: {}", account.id);
}

