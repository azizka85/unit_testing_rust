use crate::models;

pub struct Like {
  likes: Vec<models::Like>
}

impl Like {
  pub fn new() -> Self {
    let users = [
      models::User {
        id: 1, 
        first_name: "FirstName1".to_owned(), 
        last_name: "LastName1".to_owned(), 
        email: "test@1.again".to_owned()
      },
      models::User {
        id: 2,
        first_name: "FirstName2".to_owned(),
        last_name: "LastName2".to_owned(),
        email: "test@2.again".to_owned()
      },
      models::User {
        id: 3,
        first_name: "FirstName3".to_owned(),
        last_name: "LastName3".to_owned(),
        email: "test@3.again".to_owned()
      }
    ];

    let posts = [
      models::Post {
        id: 1,
        author: users[0].clone(),
        title: "Title #1".to_owned(),
        text: Some("Text #1".to_owned()),
        description: Some("Description #1".to_owned())
      },
      models::Post {
        id: 2,
        author: users[1].clone(),
        title: "Title #2".to_owned(),
        text: Some("Text #2".to_owned()),
        description: Some("Description #2".to_owned())
      },
      models::Post {
        id: 3,
        author: users[2].clone(),
        title: "Title #3".to_owned(),
        text: Some("Text #3".to_owned()),
        description: Some("Description #3".to_owned())
      },
      models::Post {
        id: 4,
        author: users[2].clone(),
        title: "Title #4".to_owned(),
        text: Some("Text #4".to_owned()),
        description: Some("Description #4".to_owned())
      }
    ];

    Self {
      likes: vec![
        models::Like {id: 1, user: users[2].clone(), post: posts[1].clone()},
        models::Like {id: 2, user: users[1].clone(), post: posts[3].clone()},
        models::Like {id: 3, user: users[0].clone(), post: posts[0].clone()},
        models::Like {id: 4, user: users[0].clone(), post: posts[2].clone()}
      ]
    }
  }

  pub fn list(&self) -> Vec<models::Like> {
    self.likes.clone()
  }
}

#[cfg(test)]
mod tests {
  use crate::repository;

  #[test]
  fn test_author() {
    let like_repository = repository::Like::new();

    for like in like_repository.likes {
      assert!(like.post.author.id > 0);
    }
  }
}

