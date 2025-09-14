use crate::traits::summary::Summary;   // 当前 crate 里的 Summary trait

pub struct SocialPost {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub repost: bool,
}

impl Summary for SocialPost {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}