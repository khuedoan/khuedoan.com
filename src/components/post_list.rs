use crate::content::PostData;
use itertools::Itertools;
use leptos::*;
use std::collections::HashMap;

#[component]
fn PostCover(href: String, src: String) -> impl IntoView {
    view! {
        <a href=href>
            <img
                class="absolute inset-0 h-full w-full rounded-2xl bg-gray-50 object-cover"
                src=src
                // TODO
                alt=""
                width="720"
                height="1280"
            />
            <div class="absolute inset-0 rounded-2xl ring-1 ring-inset ring-gray-900/10"></div>

        </a>
    }
}

#[component]
fn TagLabel(tag: String) -> impl IntoView {
    view! {
        <a
            class="inline-flex items-center gap-x-1.5 rounded-full px-2 py-1 text-xs font-medium text-gray-900 ring-1 ring-inset ring-gray-200"
            href=format!("/tags/{tag}")
        >
            {tag}
        </a>
    }
}

#[component]
fn PostPreview(id: String, post: PostData) -> impl IntoView {
    view! {
        <article key=id.clone() class="relative isolate flex flex-col gap-8 lg:flex-row">
            <div class="relative aspect-[16/9] lg:w-64 lg:shrink-0">
                <PostCover href=format!("/posts/{}", id.clone()) src=post.metadata.cover/>
            </div>
            <div>
                <div class="flex items-center gap-x-4 text-xs">
                    <p>
                        {&post.metadata.date}
                    </p>
                    {post
                        .metadata
                        .tags
                        .iter()
                        .map(|tag| view! { <TagLabel tag=tag.clone()/> })
                        .collect_view()}
                </div>
                <div class="group relative max-w-xl">
                    <h2 class="mt-3 text-lg font-semibold leading-6 text-gray-900 group-hover:text-gray-600">
                        <a href=format!("/posts/{}", id.clone())>
                            <span class="absolute inset-0"></span>
                            {post.metadata.title}
                        </a>
                    </h2>
                    <div
                        class="prose mt-5 text-sm leading-6 text-gray-600"
                        inner_html=post.metadata.summary
                    ></div>
                </div>
            </div>
        </article>
    }
}

#[component]
pub fn PostList(posts: HashMap<String, PostData>) -> impl IntoView {
    view! {
        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <div class="mt-16 space-y-10 lg:mt-10 lg:space-y-10">
                {posts
                    .iter()
                    .sorted_by(|(_, a), (_, b)| b.metadata.date.cmp(&a.metadata.date))
                    .map(|(id, post)| {
                        view! { <PostPreview id=id.to_string() post=post.clone()/> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
