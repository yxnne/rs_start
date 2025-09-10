pub fn try_str() {
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);
} 


pub fn link_str_lost() {
  let s1 = String::from("Hello, ");
  let s2 = String::from("world 2!");
  let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

  // s1 失去了所有权
  // 因为 + 操作符拼接字符串在rs中会丢失所有权
  println!("{}", s3);
  // println!("s1 失去了所有权，因为+拼接字符串在rs中会丢失所有权{}", s1);
}

pub fn link_str_safe() {
  let s1 = String::from("Hello");
  let s2 = String::from(", world safe!");
  let s = format!("{}{}", s1, s2);

  println!("{}", s);
}

pub fn try_str_len() {
  let s = "hello world";
  println!("{}", s.len());
}

pub fn try_str_loop() {
  println!("[START] --- try_str_loop");
  for c in "Зд".chars() {
    println!("{}", c);
  }

  for b in "Зд".bytes() {
    println!("{}", b);
  }

  println!("[END] --- try_str_loop");
}