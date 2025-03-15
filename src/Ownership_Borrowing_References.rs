Ownership:
fn main(){
    let s1 = String::from("Hello");
    let s2 = s1; //owbership of hello is moved to s2
    // println!("{}", s1); This would give an error because s1 is no longer owns the string
    println!("{}", s2);
}

Borrowing:
fn main() {
    let s1 =String::from("Hello");

    borrow_string(&s1); // borrow s1
    println!("{}",s1); // this works because we borrowed it, not moved it
}

fn borrow_string(s: &String){
    println("{}",s); //we can read the string but not modify it
}
