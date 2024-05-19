use scraper::{Html as ScraperHtml, Selector};

pub fn add_tailwind_classes(html: &str) -> String
{
    let document = ScraperHtml::parse_document(html);
    let mut result = String::new();
    let h1_selector = Selector::parse("h1").unwrap();
    let h2_selector = Selector::parse("h2").unwrap();
    let ul_selector = Selector::parse("ul").unwrap();
    let pre_selector = Selector::parse("pre").unwrap();

    for element in document.select(&h1_selector) {
        result.push_str(&format!(
            r#"<h1 class="text-4xl font-bold mb-4">{}</h1>"#,
            element.inner_html()
        ));
    }
    for element in document.select(&h2_selector) {
        result.push_str(&format!(
            r#"<h2 class="text-2xl font-semibold mb-2">{}</h2>"#,
            element.inner_html()
        ));
    }
    for element in document.select(&ul_selector) {
        result.push_str(&format!(
            r#"<ul class="list-disc pl-5 mb-4">{}</ul>"#,
            element.inner_html()
        ));
    }
    for element in document.select(&pre_selector) {
        result.push_str(&format!(
            r#"<pre class="bg-gray-100 p-4 rounded mb-4">{}</pre>"#,
            element.inner_html()
        ));
    }
    result
}