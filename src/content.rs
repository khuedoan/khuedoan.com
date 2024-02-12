use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PostMetadata {
    pub date: String,
    pub draft: bool,
    pub summary: String,
    pub tags: Vec<String>,
    pub title: String,
    pub cover: String,
}

#[derive(Clone, Debug)]
pub struct PostData {
    pub metadata: PostMetadata,
    pub content: String,
}

include!(concat!(env!("OUT_DIR"), "/all_posts.rs"));

pub fn get_all_tags() -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    get_all_posts().values().for_each(|post| {
        post.metadata.tags.iter().for_each(|tag| {
            if !tags.contains(tag) {
                tags.push(tag.clone());
            }
        })
    });

    tags.sort();

    tags
}

pub fn get_posts_by_tag(tag: String) -> HashMap<String, PostData> {
    get_all_posts()
        .into_iter()
        .filter(|(_id, post)| post.metadata.tags.contains(&tag))
        .collect()
}

pub fn get_post(id: String) -> PostData {
    get_all_posts().get(&id).cloned().unwrap()
}
