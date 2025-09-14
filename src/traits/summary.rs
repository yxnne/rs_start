pub trait Summary {
  fn summarize(&self) -> String;
}


// 作为参数
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}


