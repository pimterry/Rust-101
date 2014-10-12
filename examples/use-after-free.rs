fn print_and_free(value: &int) {
  println!("Value was: {}", *value)
}

fn main() {
  let value: Box<int> = box 123;
  print_and_free(&*value);

  println!("Value now is: {}", *value)
}

