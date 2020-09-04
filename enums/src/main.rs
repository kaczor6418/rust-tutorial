#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    PENNY,
    NICKLE,
    DIME,
    QUARTER
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let ip_v4 = IpAddr::V4(127, 0, 0, 1);
    let ip_v6 = IpAddr::V6(String::from("::1"));
    println!("{:#?}", ip_v4);

    let coins = [Coin::PENNY, Coin::DIME];
    for coin in coins.iter() {
        println!("Coin in cents: {}", values_in_cents(&coin));
    }
    let numbers = [1, 3, 5, 8];
    for number in numbers.iter() {
        print_number_text_name(&number);
    }

    let some_u8_value = Some(4);
    if Some(3) == some_u8_value {
        println!("It is three");
    } else {
        println!("It is not three");
    }
}

fn values_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::PENNY => {
            println!("This is a penny coin.");
            return 1;
        },
        Coin::NICKLE => 5,
        Coin::DIME => 10,
        Coin::QUARTER => 25
    }
}

fn print_number_text_name(value: &u8) {
    match value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("bigger than four") //This default patter can be very usefull if we hve cases in which we are not interested and we do not have to cover
    }
}
