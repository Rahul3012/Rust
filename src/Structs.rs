struct Person{
  name: String,
  age: u32,
}

fn main() {
  let person = Person {
    name: string::from("Alice"),
    age: 30,
  };

  println!("{} is {} years old.",person.name, person.age);
}
