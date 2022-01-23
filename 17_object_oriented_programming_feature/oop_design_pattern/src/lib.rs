//State pattern in Rust

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            //Whenever generating new Post, it will start out as a draft
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    //if the state is Published, we want to return the value in the post’s content field; otherwise, we want to return an empty string slice
    pub fn content(&self) -> &str {
        //as_ref() -> a reference to the value inside Option
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        //take method takes the Some value out of state field and leave a None in it
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

//State trait defines behavior shared by different post states.
//Draft, PendingReview, Published states will implement State trait
trait State {
    //all types that implement the trait will now need to implement the request_review method.
    //This method is only valid when called on a Box holding type
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    //it will have no effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        //it returns itself
        self
    }
    //returns a new, boxed instance of Published struct
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    //it will have no effect
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// With the state pattern, the Post methods and the places we use Post don’t need match expressions,
// and to add a new state, we would only need to add a new struct and implement the trait methods on that one struct.
