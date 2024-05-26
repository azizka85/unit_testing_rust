use super::{Post, User};

#[derive(Debug, Clone)]
pub struct Like {
	pub id: usize,
	pub user: User,
	pub post: Post	
}
