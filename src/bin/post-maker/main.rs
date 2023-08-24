use bl0g::FrontMatter;
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let title = args.get(1).expect("Please provide a title for the post.");

    if title.contains("-") {
        panic!("Please use _ (underscores) for naming posts instead of - (dashes).");
    }

    let file_name = format!("{title}.md");
    let frontmatter = FrontMatter::new(title.to_string()).to_string();

    match fs::metadata(format!("./posts/{file_name}")) {
        Ok(_) => panic!("{file_name} already exsists!"),
        Err(_) => {
            let mut file = fs::File::create(format!("./posts/{file_name}"))
                .expect("Unable to create new file.");
            file.write_all(frontmatter.as_bytes())
                .expect("Unable to write to new file.");

            println!("{title}.md successfully created!");
        }
    };
}
