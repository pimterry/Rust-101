fn returnDanglingPointer() -> &'static int {
  let myNum = 123;
  return &myNum;
}

fn main() {
  let x: &int = returnDanglingPointer();
  println!("{}", x);
}
