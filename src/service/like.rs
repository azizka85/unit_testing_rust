use std::collections::HashMap;

use crate::{models, repository};

pub struct Like {
  like_repository: repository::Like
}

impl Like {
  pub fn new(like_repository: repository::Like) -> Self {
    Self { like_repository }
  }

  pub fn most_liked_author(&self) -> Option<models::User> {
    let mut users = HashMap::new();

    for like in self.like_repository.list() {
      let t = users
        .entry(like.post.author.id)
        .or_insert((
          like.post.author,
          0
        ));

      t.1 += 1;
    }

    let mut max_liked = 0;
    let mut max_user = None;

    for (_, t) in users {
      if max_liked < t.1 {
        max_liked = t.1;
        max_user = Some(t.0);
      }
    }

    max_user
  }
}

/* 
#[cfg(test)]
mod tests {
  use crate::service;
  use crate::repository;

  #[test]
  fn test_author() {
    let like_service = service::Like::new(
      repository::Like::new()
    );

    let res = like_service.most_liked_author();

    assert!(res.is_ok());
  }
}
*/
