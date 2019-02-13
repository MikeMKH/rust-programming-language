#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_item() {
        let mut col = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        AveragedCollection::add(&mut col, 1);
        assert_eq!(AveragedCollection::average(&col), 1.0);
    }

    #[test]
    fn it_remove_item() {
        let mut col = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        AveragedCollection::add(&mut col, 1);
        let item = AveragedCollection::remove(&mut col);

        assert_eq!(item.unwrap(), 1);
    }

    #[test]
    fn happy_path_blog_post() {
        let mut post = Post::new();

        post.add_text("I ate breakfast for dinner last night.");
        let post = post.request_review();
        let mut post = post.approve();
        let mut post = post.approve();

        assert_eq!("I ate breakfast for dinner last night.", post.content());
    }

    #[test]
    fn rejected_blog_post() {
        // hidden vegetarian propaganda :)
        let mut post = Post::new();

        post.add_text("I ate an animal for dinner last night.");
        let post = post.request_review();
        let mut post = post.reject();

        post.add_text("I ate salad for dinner last night.");
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();

        assert_eq!("I ate salad for dinner last night.", post.content());
    }

    #[test]
    fn rejected_before_final_approved_blog_post() {
        let mut post = Post::new();

        post.add_text("I ate ice cream for dinner last night.");
        let post = post.request_review();
        let post = post.approve();
        let mut post = post.reject();

        post.add_text("I ate custard for dinner last night.");
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();

        assert_eq!("I ate custard for dinner last night.", post.content());
    }
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

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
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> PendingFinalReviewPost {
        PendingFinalReviewPost {
            content: self.content,
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}

pub struct PendingFinalReviewPost {
    content: String,
}

impl PendingFinalReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}
