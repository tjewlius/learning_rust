fn main() {
    // immutable Transaction instance
    let transaction_test = Transaction {
        id: 27,
        amount: 22_500,
        verified: true,
    };

    println!("{:#?}", transaction_test);

    let mut new_subscriber = Subscriber {
        msisdn: String::from("+2567000000000"),
        active: true,
        swap_count: 0,
    };

    // println!("{:#?}", new_subscriber);

    // println!();
    // println!("{:#?}", new_subscriber);
    // println!("{}", if is_risky(new_subscriber.swap_count) {"This is risky"} else {"This is okay"});
    // println!("=================================================");

    // flag_swap(&mut new_subscriber); // increment swap count

    // println!();
    // println!("{:#?}", new_subscriber);

    // println!("{}", if is_risky(new_subscriber.swap_count) {"This is risky"} else {"This is okay"});
    // println!("=================================================");

    // flag_swap(&mut new_subscriber); // increment swap count

    // println!();
    // println!("{:#?}", new_subscriber);
    // println!("{}", if is_risky(new_subscriber.swap_count) {"This is risky"} else {"This is okay"});
    // println!("=================================================");
    // println!("{}", new_subscriber.active);

    for _ in 0..3 {
        new_subscriber.flag_swap();

        println!("{:#?}", new_subscriber);
        println!(
            "{}",
            if new_subscriber.is_risky() {
                "Risk level is High"
            } else {
                "Risk level is Low"
            }
        );
        println!(
            "Subscribe status: {}",
            if new_subscriber.active {
                "Active"
            } else {
                "Deactivated"
            }
        );
        println!("=================================================");
    }

    // println!("{}", check(Msisdn("0712345678"))) // this returns E0308 which is calling a function while passing
    // an argument that has different from the declared function parameter
    // 
    // let new_user = TestSruct {
    //     name: String::from("Name Passed")
    // };

    // println!("{}", new_user.name);
}

/*
 * ==== STRUCTS ====
 * a struct(structure) is a custom data type that is used to group multiple
 * related values
 *
 * struct kekyword -> Struct name -> opening curly brace -> fields -> closing curly brace
 *
 * structs are immutable by default as well and we use `mut` to create a mutable struct
 * mutability is on the struct, not per field
 *
 * dot notation to access values of a struct
*/

// this is just a struct definition
// it is not data, just defining the shape of the data

#[derive(Debug)]
struct Transaction {
    id: u64,
    amount: u64,
    verified: bool,
}

#[derive(Debug)]
struct Subscriber {
    msisdn: String,
    active: bool,
    swap_count: u32,
}

impl Subscriber {
    fn flag_swap(&mut self) {
        self.swap_count += 1;
        self.active = self.swap_count < 3
    }

    fn is_risky(&self) -> bool {
        self.swap_count >= 3
    }
}

struct Msisdn(String);

// struct TestSruct {
//     name: &str
// }

impl Msisdn {
    fn check(&self) -> bool {
        self.0.len() == 10 || self.0.len() == 12 || self.0.len() == 13
        //0712345678 or 256712345678 or +256712345678
    }
}