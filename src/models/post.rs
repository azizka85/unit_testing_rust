use super::User;

#[derive(Debug, Clone)]
pub struct Post {
  pub id: usize,
  pub author: User,
  pub title: String,
  pub text: Option<String>,
  pub description: Option<String>
}
