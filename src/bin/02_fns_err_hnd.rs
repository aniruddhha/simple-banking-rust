struct Account {
    ac_number: String,
    balance: f32,
    ac_name: String,
}

fn create_new_account(db: &mut Vec<Account>, ac: Account) {
    db.push(ac);
}

fn find_by_ac_number(db: &Vec<Account>, ac_num: String) -> Result<&Account, String> {
    let op = db.iter().find(|ac: &&Account| ac.ac_number.eq(&ac_num));
    match op {
        Some(ac) => Ok(ac),
        None => Err("Account Number Not Found".to_string()),
    }
}

fn show_all_accounts(db: &Vec<Account>)  {
    db.iter()
        .for_each(|ac: &Account| println!("Ac Num {}", ac.ac_number));
}

fn show_single_account(db: &Vec<Account>, ac_num: String) {
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
) -> Result<(), String> {
    let ac = db
        .iter_mut()
        .find(|ac: &&mut Account| ac.ac_number.eq(&ac_num));
    match ac {
        Some(ac) => {
            ac.ac_name = new_name.to_string();
            ac.balance = new_balance;

            Ok(())
        }
        None => Err("Account Not Found".to_string()),
    }
}

fn delete_account(db: &mut Vec<Account>, ac_num: String) -> Result<(), String> {
    let ac_ind = db.iter().position(|a: &Account| a.ac_number.eq(&ac_num));
    if let Some(ind) = ac_ind {
        db.remove(ind);
        Ok(())
    } else {
        Err("Account Not Found".to_string())
    }
}

fn main() {
    let mut db: Vec<Account> = Vec::new();

    create_new_account(
        &mut db,
        Account {
            ac_number: "ac-101".to_string(),
            balance: 100.,
            ac_name: "and1".to_string(),
        },
    );

    create_new_account(
        &mut db,
        Account {
            ac_number: "ac-102".to_string(),
            balance: 200.,
            ac_name: "and2".to_string(),
        },
    );

    create_new_account(
        &mut db,
        Account {
            ac_number: "ac-103".to_string(),
            balance: 20.,
            ac_name: "and3".to_string(),
        },
    );

    // show_single_account(&db, "ac-103".to_string());

    // show_all_accounts(&db);
    // show_single_account(&db, "ac-102".to_string());
    //update_account_information(&mut db, "ac-103".to_string(), "and3-updated".to_string(), 300_00_00.);
    show_all_accounts(&db);
    delete_account(&mut db, "ac-102".to_string());
    show_all_accounts(&db);
    // show_single_account(&db, "ac-103".to_string());
}
