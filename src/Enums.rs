enum Direction {
  Up,
  Down,
  Left,
  Right,
  Straight
}

fn move_player(direction: Direction) {
  match direction {
    Direction::Up => println!("Moving Up");
    Direction::Down => println!("Moving Down");
    Direction::Left => println!("Moving Left");
    Direction::Right => println!("Moving Right");
    Direction::Straight => println!("Moving Straight");
  }
}

fn main() {
  move_player(Direction::Up);
  move_player(Direction::Down);
  move_player(Direction::Left);
  move_player(Direction::Right);
  move_player(Direction::Straight);
}
  
