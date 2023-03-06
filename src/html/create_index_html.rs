use std::{
    fs::{self, File},
    io::Write,
};

use crate::notion::notion_db::{self, NotionPageMetaData};

pub struct IndexHtml;

impl IndexHtml {
    pub fn new() -> IndexHtml {
        IndexHtml {}
    }

    async fn tiltes_to_list_elements(&self) -> Vec<String> {
        let blog_meta_data: Vec<NotionPageMetaData> = notion_db::get_notion_db().await.unwrap();
        let mut title_strings: Vec<String> = Vec::new();

        for page in blog_meta_data {
            let no_space_title = page.title.replace(" ", "");
            let date_of_creation = iso8601::datetime(page.created_time.as_str()).unwrap();
            let link_tag = format!(
                "
                <p id=\"date_text\">{}</p>
                <li><a href=\"/{}\"/>{}</a></li>
                ",
                date_of_creation.date, no_space_title, page.title
            );
            title_strings.push(link_tag)
        }

        title_strings
    }

    pub async fn create_index_file(&self) {
        // Get timestamp from server, for last edit
        // Create HTML-File and open the sample file
        let mut file = File::create("./static/index.html").unwrap();
        let sample_html = fs::read_to_string("./templates/index_template.html").unwrap();

        // Get all links formatted as HTML elements
        let titles = self.tiltes_to_list_elements().await;
        let all_title_elements = titles.join("\n");

        let with_links = sample_html.replace("{links}", all_title_elements.as_str());

        file.write_all(with_links.as_bytes());
    }
}
