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
}