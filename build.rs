use gray_matter::engine::YAML;
use gray_matter::Matter;
use include_dir::*;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;

fn optimize_images(image_dir: &str, target_dir: &str) {
    fs::read_dir(Path::new(image_dir))
        .expect("Failed to read image directory")
        .for_each(|image| {
            let input_file = image.expect("Cannot read imgae");
            let output_file = Path::new(target_dir).join(input_file.file_name());
            Command::new("convert")
                .arg(input_file.path())
                .arg("-resize")
                .arg("x256")
                .arg(output_file.with_extension("webp"))
                .spawn()
                .expect("Failed to convert image");
        });
}

fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

fn write_all_posts() {
    static POST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("all_posts.rs");
    fs::write(
        &dest_path,
        format!(
            "pub fn get_all_posts() -> HashMap<String, PostData> {{
                HashMap::from([
                  {}
                ])
            }}",
            POST_DIR
                .files()
                .map(|post| {
                    let id = post.path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    let matter = Matter::<YAML>::new();
                    let markdown = matter.parse(post.contents_utf8().unwrap());
                    let front_matter = markdown.data.unwrap();
                    let content = markdown_to_html(markdown.content);
                    // 
                    // {chrono::DateTime::parse_from_rfc3339(&post.metadata.date)
                    //     .unwrap()
                    //     .format("%e %b %Y")
                    //     .to_string()}

                    format!(
                        "(
                            {id},
                            PostData {{
                                metadata: PostMetadata {{
                                    date: \"{TODO chrono}\",
                                    draft: {TODO},
                                    summary: \"{TODO}\",
                                    tags: vec![{TODO}],
                                    title: \"{TODO}\",
                                    cover: \"{TODO}\",
                                }},
                                content: \"{TODO safe content}\"
                            }}
                            
                        ).filter(|(_id, post)| !post.metadata.draft)
                        "
                    )
                }).collect::<Vec<_>>().join("\n")
        )
    ).unwrap();
}

fn main() {
    let image_dir = "public/images";
    let target_dir = "target/site/images";
    // println!("cargo:rerun-if-changed={image_dir}");
    optimize_images(image_dir, target_dir);
    write_all_posts();
}
