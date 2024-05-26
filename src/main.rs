pub mod models;
pub mod repository;
pub mod service;

fn main() {
	let like_service = service::Like::new(
		repository::Like::new()
	);

	println!("Most liked author: {:#?}", like_service.most_liked_author());
}
