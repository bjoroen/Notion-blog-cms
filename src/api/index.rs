use actix::Arbiter;
use actix_files as afs;
use actix_web::get;
use actix_web::{Error, HttpRequest};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::{
    html::{create_index_html::IndexHtml, html_parser::HtmlParser, lexer::Lexer},
    notion::{notion_db::get_notion_db, page_request::get_notion_page_data},
    utils::compare_timestamps::has_blog_post_changed,
};

#[get("/")]
pub async fn get_index() -> Result<afs::NamedFile, Error> {
    let execution = async {
        if has_blog_post_changed().await {
            create_files().await;
        }
    };

    let thread = Arbiter::new();

    thread.spawn(execution);

    println!("{}", "Sending html done");
    Ok(afs::NamedFile::open("./static/index.html")?)
}

pub async fn create_files() {
    let page_data = match get_notion_db().await {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let index = IndexHtml::new();
    index.create_index_file().await;

    let mut timestamps: Vec<String> = Vec::new();

    for page in &page_data {
        let temp_val = get_notion_page_data(page.id.clone()).await.unwrap();
        let mut lex = Lexer::new(&temp_val, page.title.clone());
        lex.collect_tokens();
        let mut html = HtmlParser::new(lex);
        timestamps.push(page.last_edited_time.clone());
        html.generate_html();
        html.generte_html_page();
    }

    let mut file = File::create("./static/timestamp.json").unwrap();
    let last_time_db_edit = timestamps.join("\n");

    println!("{}", "Creating files done");

    file.write_all(last_time_db_edit.as_bytes());
}

#[get("/style.css")]
pub async fn get_style() -> Result<afs::NamedFile, Error> {
    let path = Path::new("./templates/style.css");
    Ok(afs::NamedFile::open(path)?)
}

#[get("/style-post.css")]
pub async fn get_style_post() -> Result<afs::NamedFile, Error> {
    let path = Path::new("./templates/style-post.css");
    Ok(afs::NamedFile::open(path)?)
}

#[get("/{filename:.*}")]
pub async fn get_blog_html_page(req: HttpRequest) -> Result<afs::NamedFile, Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut folder_path = PathBuf::from("./static/");
    folder_path.push(path);
    folder_path.set_extension("html");
    let file = afs::NamedFile::open(folder_path)?;
    Ok(file)
}
