use bl0g::FrontMatter;
use std::env;
use std::fs;
use std::io::Write;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<_>>();
    let title = args.get(1).expect("Please provide a title for the post.");

    if title.contains('-') {
        panic!("Please use _ (underscores) for naming posts instead of - (dashes).");
    }

    let file_name = format!("{title}.md");
    let frontmatter = FrontMatter::new(title.to_string());

    let api_token = fs::read_to_string(".env").unwrap();

    match fs::metadata(format!("./data/posts/{file_name}")) {
        Ok(_) => panic!("{file_name} already exsists!"),
        Err(_) => {
            // Create the post in D1.
            let client = reqwest::Client::new();
            let res = client
                .post(format!(
                    "https://worker-rust.austin-e33.workers.dev/new/{}",
                    frontmatter.id
                ))
                .header("API_TOKEN", api_token.trim())
                .send()
                .await
                .expect("Unable to send request to create new post in d1.");

            if res.status().as_u16() == 200 {
                println!("Post successfully added to d1!");
            } else {
                println!("Post was not added to d1.");
            }

            let mut file = fs::File::create(format!("./data/posts/{file_name}"))
                .expect("Unable to create new file.");
            file.write_all(frontmatter.to_string().as_bytes())
                .expect("Unable to write to new file.");

            println!("{title}.md successfully created!");
        }
    };
}
