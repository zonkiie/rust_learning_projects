use super::schema::posts;
use std::fmt; // Import `fmt`

#[derive(Queryable)]
pub struct Post {
    //pub id: i32,
	pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// Implement `Display` for `Post`.
impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Post: ({:?}, {}, {})", self.id, self.title, self.body)
    }
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


// Implement `Display` for `NewPost`.
impl<'a> fmt::Display for NewPost<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.title, self.body)
    }
}

