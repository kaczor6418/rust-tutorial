/// If it is possible that we will return every parameter of method every parameter need to have **same lifetime**
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    return if x.len() > y.len() { x } else { y };
}

/// If method will always return first parameter only first parameter lifetime need to be specified
fn first_parameter<'a>(x: &'a str, y: &str) -> &'a str {
    return x;
}

/// This method is correct because variables `string1` and `string2` have the same lifetime
fn correct_lifetime1() {
    let string1 = String::from("abcd");
    let string2 = "abcdef";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

/// This method is correct because lifetime of `string2` is in range of variable `string1` (`string1` has longer lifetime then `string2`)
fn correct_lifetime2() {
    let string1 = String::from("abcd");
    {
        let string2 = "abcdef";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is '{}'", result);
    }
}

/// This method is incorrect because lifetime of `result` used in second `println!` is different then lifetime of `string2`
fn incorrect_lifetime1() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result);
}

/// This method is incorrect because lifetime of `string3` assigned to variable `sting2` is shorter then lifetime of variable `string`
fn incorrect_lifetime2() {
    let string1 = String::from("abcd");
    let string2;
    {
        let string3 = "abcdef";
        string2 = &string3;
    }
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is '{}'", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        return 3;
    }

    /// Will not throw an error because returns **`self`** which first 3rd *Lifetime Elision* rules return type has the same type as **`self`** part of **`self`** is returned so rule is true
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }

    // Will throw an error because `announcement` is in different lifetime then **`self`**
    // fn return_announcement(&self, announcement: &str) -> &str {
    //     return announcement;
    // }
}

fn main() {
    /// static lifetime gives a guarantee that variable will be stored directly in program binary. Before you use this lifetime you should think twice because it will make your program to weight more
    let i_will_never_die: &'static str = "I have a static lifetime.";

    // Lifetimes in methods
    correct_lifetime1();
    correct_lifetime2();
    incorrect_lifetime1();
    incorrect_lifetime2();

    // Lifetime in structures and impl
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("{}", first_sentence);
    let important_expert = ImportantExcerpt {
        part: first_sentence,
    };
    let announcement = "Lifetime announcement";
    let some_number = important_expert.level();
    let also_first_sentence = important_expert.announce_and_return_part(announcement);
    println!("Some number: {}", some_number);
    println!("Also first sentence: {}", also_first_sentence);
}
