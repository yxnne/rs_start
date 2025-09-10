use std::collections::HashMap;

pub fn try_map () {

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  println!("{:?}", scores);
}

pub fn try_map_or_insert () { 
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.entry(String::from("Yellow")).or_insert(50);

  println!("{:?}", scores);
}

pub fn try_map_item_statistic () { 
  let text = String::from("hello world hello world hello world");

  let mut map = HashMap::new();

  for word in text.split_whitespace() {

    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}

pub fn try_map_update () { 
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);
}

