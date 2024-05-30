fn main() {
    let s1 = returns_ownership();
    println!("s1 is {}", s1);

    let s2 = String::from("Test");
    takes_ownership(s2);
    // println!("s2 is {}", s2); // does not compile, s2 was moved

    let s3 = takes_and_gives_ownership(s1);
    // println!("s1 is {}", s1); // does not compile, s1 was moved
    println!("s3 is {}", s3);

    reads_reference(&s3);

    let mut s4 = s3;
    changes_by_reference(&mut s4);
    // println!("s3 is {}", s3); // does not compile, s3 was moved
    println!("s4 is {}", s4);
}

fn returns_ownership() -> String {
    String::from("Hi")
}

fn takes_ownership(str: String) {
    println!("I took ownership of {} and will drop at the end!", str);
}

fn takes_and_gives_ownership(str: String) -> String {
    println!("I took owenership of {} and will move it at the end!", str);
    str
}

fn reads_reference(str: &String) {
    println!("I only have a reference to {}!", str);
}

fn changes_by_reference(str: &mut String) {
    str.push_str("!! I appended this");
}
