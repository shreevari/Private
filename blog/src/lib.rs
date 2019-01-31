pub struct Post {
	content: String,
}

pub struct DraftPost {
	content: String,
}

impl Post {
	pub fn new() -> DraftPost {
		DraftPost {
			content: String::new(),
		}
	}
	pub fn content(&self) -> &str {
		&self.content
	}
}

impl DraftPost {
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}
	pub fn request_review(self) -> PendingReviewPost {
		PendingReviewPost {
			content: self.content,
			approvals: 0,
		}
	}
}

pub struct PendingReviewPost {
	content: String,
	approvals: u32,
}

impl PendingReviewPost {
	pub fn consent(&mut self) {
		self.approvals += 1;
	} 
	pub fn approve(self) -> Option<Post> {
		if self.approvals >= 2 {
			Some(Post {
					content: self.content,
				})
		} else {
			None
		}
	}
	pub fn reject(self) -> DraftPost {
		DraftPost {
			content: self.content,
		}
	}
}

// pub struct Post {
// 	state: Option<Box<dyn State>>,
// 	content: String,
// }

// impl Post {
// 	pub fn new() -> Post {
// 		Post {
// 			state: Some(Box::new(Draft {})),
// 			content: String::new(),
// 		}
// 	}

// 	pub fn add_text(&mut self, text: &str) {
// 		if self.state.as_ref().unwrap().is_editable() {
// 			self.content.push_str(text);
// 		}
// 	}

// 	pub fn content(&self) -> &str {
// 		self.state.as_ref().unwrap().content(&self)
// 	}

// 	pub fn request_review(&mut self) {
// 		if let Some(s) = self.state.take() {
// 			self.state = Some(s.request_review())
// 		}
// 	}

// 	pub fn approve(&mut self) {
// 		if let Some(s) = self.state.take() {
// 			self.state = Some(s.approve())
// 		}
// 	}
// 	pub fn reject(&mut self) {
// 		if let Some(s) = self.state.take() {
// 			self.state = Some(s.reject())
// 		}
// 	}
// }

// trait State {
// 	fn request_review(self: Box<Self>) -> Box<dyn State>;
// 	fn approve(self: Box<Self>) -> Box<dyn State>;
// 	fn reject(self: Box<Self>) -> Box<dyn State>;
// 	fn is_editable(&self) -> bool {
// 		false
// 	}
// 	fn content<'a>(&self, post: &'a Post) -> &'a str {
// 		""
// 	}
// }

// struct Draft {}

// impl State for Draft {
// 	fn is_editable(&self) -> bool {
// 		true
// 	}
// 	fn request_review(self: Box<Self>) -> Box<dyn State> {
// 		Box::new(PendingReview {approvals: 0})
// 	}
// 	fn approve(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// 	fn reject(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// }

// struct PendingReview {
// 	approvals: u32,
// }

// impl State for PendingReview {
// 	fn request_review(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// 	fn approve(mut self: Box<Self>) -> Box<dyn State> {
// 		self.approvals += 1;
// 		if self.approvals == 2 {
// 			Box::new(Published {})	
// 		} else {
// 			self
// 		}
// 	}
// 	fn reject(self: Box<Self>) -> Box<dyn State> {
// 		Box::new(Draft {})
// 	}
// }

// struct Published {}

// impl State for Published {
// 	fn request_review(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// 	fn approve(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// 	fn reject(self: Box<Self>) -> Box<dyn State> {
// 		self
// 	}
// 	fn content<'a>(&self, post: &'a Post) -> &'a str {
// 		&post.content
// 	}
// }