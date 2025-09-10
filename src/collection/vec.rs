pub fn try_vec() { 
  let v1 = vec![1, 2, 3];

  println!("{:?}", v1);


  let mut v: Vec<i32> = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  let disange: &i32 = &v[2];
  println!("The disange element is {disange}");

  let third: Option<&i32> = v.get(2);
  match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
  }

  let di10ge = v.get(10);
  match di10ge {
    Some(di10ge) => println!("The di10ge element is {di10ge}"),
    None => println!("There is no di10ge element."),
  }


  // 简单遍历
  for i in &v {
    println!("{i}");
  }
}