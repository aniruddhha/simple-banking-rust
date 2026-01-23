 #[derive(Debug)]
 struct AccountError {
    msg: &'static str
 }

struct InvalidAccountNumberError;

struct AccountNumberNotFoundError;

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account Error: {}", self.msg)
    }
}

impl From<AccountNumberNotFoundError> for AccountError {
    fn from(_: AccountNumberNotFoundError) -> Self {
        AccountError { msg: "Account number not found" }
    }
}

impl From<InvalidAccountNumberError> for AccountError {
    fn from(_: InvalidAccountNumberError) -> Self {
        AccountError { msg: "Invalid account number" }
    }
    
}


struct Account {
    ac_number: String,
    balance: f32,
    ac_name: String,
}

fn create_new_account(db: &mut Vec<Account>, ac: Account) {
    db.push(ac);
}

fn show_all_accounts(db: &Vec<Account>)  {
    db.iter()
        .for_each(|ac: &Account| println!("Ac Num {}", ac.ac_number));
}

fn find_by_ac_number(db: &Vec<Account>, ac_num: String) -> Result<&Account, AccountError> {
    db.iter().find(|ac: &&Account| ac.ac_number.eq(&ac_num)).ok_or(AccountNumberNotFoundError.into())
}

fn find_by_ac_number_mutable(db: &mut Vec<Account>, ac_num: String) -> Result<&mut Account, AccountError> {
    db.iter_mut().find(|ac: &&mut Account| ac.ac_number.eq(&ac_num)).ok_or(AccountNumberNotFoundError.into())
}

fn show_single_account(db: &Vec<Account>, ac_num: String)  {
    let ac_res = find_by_ac_number(db, ac_num);

    match ac_res {
        Ok(ac) => println!(
            "Account Number {}, Account Name {}, Balance {}",
            ac.ac_number, ac.ac_name, ac.balance
        ),
        Err(e) => println!("{}", e),
    }
}

fn update_account_information(
    db: &mut Vec<Account>,
    ac_num: String,
    new_name: String,
    new_balance: f32,
) -> Result<(), AccountError> {
    let ac = find_by_ac_number_mutable(db, ac_num);
    match ac {
        Ok(ac) => {
            ac.ac_name = new_name.to_string();
            ac.balance = new_balance;

            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn delete_account(db: &mut Vec<Account>, ac_num: String) -> Result<(), AccountError> {
    let ac_ind = db.iter().position(|a: &Account| a.ac_number.eq(&ac_num));
    if let Some(ind) = ac_ind {
        db.remove(ind);
        Ok(())
    } else {
        Err(AccountNumberNotFoundError.into())
    }
}

fn main() {
    let mut db : Vec<Account> = Vec::new();
}