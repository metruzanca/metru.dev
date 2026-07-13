use dioxus::prelude::*;

use crate::blog;

#[post("/api/blog/posts")]
pub async fn get_blog_posts() -> Result<Vec<blog::BlogPost>, ServerFnError> {
    Ok(blog::get_merged_posts().await)
}
