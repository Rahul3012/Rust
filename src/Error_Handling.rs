fn divide(a: f64, b: f64) -> Result<f64, String>{
  if b == 0.0 {
    Err(String::from("Cannot divide by zero"))
  } else {
    Ok(a/b)
  }
}

fn main() {
  match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
  }
  
  match divide(50.0, 0.0) {
    Ok(result) => println!("Result: {}",result),
    Err(e) => println!("Error: {}", e),
  }
}
