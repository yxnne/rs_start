use std::ops::Deref;

pub fn try_box() {
  // 申请 堆 内存
  let b = Box::new(5);
  println!("b = {}", b);
}


// enum List {
//   Cons(i32, List),
//   Nil,
// }

// 这是不行的
// 会被报错：recursive type `List` has infinite size

// fn main() {
//   let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

#[derive(Debug)]
pub enum List {
  Cons(i32, Box<List>),
  Nil,
}


// 模拟 Box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref
// 这样就可以调用 * 来解引用
impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}