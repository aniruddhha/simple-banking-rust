struct Account {
    ac_number: String,
    balance: f32,
    ac_name: String,
}

fn main() {
    let mut db: Vec<Account> = Vec::new();

    // create new account
    db.push(Account {
        ac_number: "ac-101".to_string(),
        balance: 100.,
        ac_name: "and1".to_string(),
    });
    db.push(Account {
        ac_number: "ac-102".to_string(),
        balance: 30.,
        ac_name: "and2".to_string(),
    });

    // get account by index
    let ac_op = db.get(0);
    match ac_op {
        Some(ac) => println!("Number {}", ac.ac_number),
        None => println!("No Account available"),
    }
    let ac = db.get(0).unwrap();
    db.get(1).expect("Index out of bund");

    // find by account number
    let ac = db.iter().find(|ac: &&Account| ac.ac_number.eq("10"));

    // iterate over all
    db.iter()
        .for_each(|ac: &Account| println!("Ac Num {}", ac.ac_number));

    for ac in &db {
        println!("Ac Num {}", ac.ac_number)
    }

    // delete account
    let ac_ind = db.iter().position(|a: &Account| a.ac_number.eq("ac-101"));
    if let Some(ind) = ac_ind {
        println!("->{ind}");
        db.remove(ind);
    }


    for ac in &db {
        println!("Ac Num {}", ac.ac_number)
    }
}
