use std::collections::HashMap;

fn main() {
  // Vector
  let mut numbers = vec![1,2,3,4,5];
  numbers.push(6);
  println!("{:?}", numbers);

  // hashmap
  let mut mao = HashMap::new();
  map.insert("name","Rahul");
  map.insert("age","140");

  for(key, value) in &map{
    println!("Key: {}, value: {}", key,value);
  }
}
