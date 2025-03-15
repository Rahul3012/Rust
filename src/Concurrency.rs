use std::thread;

fn main(){
  let handle = thread::spawn(||{
    println!("Hellow from the spawned thread!");
  });

  //the main thread waits for the spawned thread to finish
  handle.join();

  println!("Hello from the main thread!");
}
