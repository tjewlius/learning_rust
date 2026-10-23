fn main() {
  // immutable Transaction instance
  let transaction_test = Transaction {
    id: 27,
    amount: 22_500,
    verified: true
  };

  println!("{:#?}", transaction_test);

  let mut new_subscriber = Subscriber {
    msisdn: String::from("+2567000000000"),
    active: true,
    swap_count: 0
  };


  println!("{:#?}", new_subscriber);

  flag_swap(&mut new_subscriber); // increment swap count

  println!();
  println!("{:#?}", new_subscriber);
  println!("{}", if is_risky(new_subscriber.swap_count) {"This is risky"} else {"This is okay"});
  println!("=================================================");

  flag_swap(&mut new_subscriber); // increment swap count

  println!();
  println!("{:#?}", new_subscriber);

  println!("{}", if is_risky(new_subscriber.swap_count) {"This is risky"} else {"This is okay"});
  println!("=================================================");

  flag_swap(&mut new_subscriber); // increment swap count

  println!();
  println!("{:#?}", new_subscriber);
  println!("{}", if is_risky(new_subscriber.swap_count) {"This is risky"} else {"This is okay"});
  println!("=================================================");
  // println!("{}", new_subscriber.active);
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
  verified: bool
}

#[derive(Debug)]
struct Subscriber {
  msisdn: String,
  active: bool,
  swap_count: u32
}

fn flag_swap(sub: &mut Subscriber) {
  sub.swap_count += 1;

  if sub.swap_count >= 3 {
    sub.active = false;
  }
}

fn is_risky(swap_count: u32) -> bool {
  swap_count > 2
}