fn main() {
  let x: &int = unsafe { std::mem::transmute(0u) };
  println!("{}", x);
}
