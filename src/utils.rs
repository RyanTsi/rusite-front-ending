use std::collections::HashMap;

use chrono::{DateTime, Utc};
use chrono_tz::Asia::Shanghai;
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::models::markdown::{FrontMatter, Heading, MarkdownContent};

pub fn format_date_cn(date: DateTime<Utc>) -> String {
    date.with_timezone(&Shanghai).format("%Y-%m-%d %H:%M").to_string()
}

// 静态正则用于提取 Front Matter
static FRONT_MATTER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap()
});

// 解析 Markdown 内容
pub fn parse_markdown(content: &str) -> Option<MarkdownContent> {
    // 解析 Front Matter
    let (front_matter, markdown_content) = parse_front_matter(content)?;
    
    // 生成 TOC 并渲染 HTML
    let (html_content, toc) = render_markdown_with_toc(&markdown_content);
    
    Some(MarkdownContent {
        front_matter,
        html_content,
        toc,
    })
}

// 解析 Front Matter
fn parse_front_matter(content: &str) -> Option<(FrontMatter, String)> {
    let captures = FRONT_MATTER_REGEX.captures(content)?;
    
    let yaml_str = captures.get(1)?.as_str();
    let markdown_content = captures.get(2)?.as_str().to_string();
    
    // 反序列化 YAML
    let front_matter: FrontMatter = serde_yaml::from_str(yaml_str)
        .ok()?;
    
    Some((front_matter, markdown_content))
}

// 渲染 Markdown 并生成 TOC
pub fn render_markdown_with_toc(markdown: &str) -> (String, Vec<Heading>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    
    let parser = Parser::new_ext(markdown, options);
    let mut toc = Vec::new();
    let mut html_output = String::new();
    let mut heading_id_counter: HashMap<String, u32> = HashMap::new();
    
    // 处理事件并生成 TOC
    let events = parser.map(|event| {
        match &event {
            Event::Start(Tag::Heading{level, ..}) => {
                // 开始处理标题
                toc.push(Heading {
                    id: String::new(),
                    text: String::new(),
                    level: *level as u32,
                });
            }
            Event::Text(text) if !toc.is_empty() => {
                // 收集标题文本
                if let Some(last) = toc.last_mut() {
                    last.text.push_str(text);
                }
            }
            Event::End(TagEnd::Heading(level)) if !toc.is_empty() => {
                // 结束标题处理，生成 ID
                if let Some(heading) = toc.last_mut() {
                    let base_id = heading.text.to_lowercase()
                        .replace(' ', "-")
                        .replace(|c: char| !c.is_alphanumeric(), "");
                    
                    let count = heading_id_counter.entry(base_id.clone()).or_insert(0);
                    *count += 1;
                    
                    heading.id = if *count > 1 {
                        format!("{}-{}", base_id, count)
                    } else {
                        base_id
                    };
                    
                    // 替换原始事件以添加 ID
                    return Event::Html(
                        format!("<h{} id=\"{}\">", level, heading.id).into()
                    );
                }
            }
            _ => {}
        }
        event
    });
    
    // 渲染 HTML
    html::push_html(&mut html_output, events);
    (html_output, toc)
}

// // 代码高亮初始化
// #[wasm_bindgen]
// pub fn init_code_highlighting() {
//     let window = web_sys::window().expect("no global window exists");
//     let document = window.document().expect("should have a document");
    
//     document.query_selector_all("pre code").unwrap().for_each(|element| {
//         let element = element.unwrap();
//         hljs_highlight_element(&element);
//     });
// }

// // 数学公式渲染
// #[wasm_bindgen]
// pub fn render_math_formulas() {
//     let window = web_sys::window().expect("no global window exists");
//     let document = window.document().expect("should have a document");
    
//     // 渲染块级公式
//     document.query_selector_all(".math-display").unwrap().for_each(|element| {
//         let element = element.unwrap();
//         render_math_element(&element, true);
//     });
    
//     // 渲染行内公式
//     document.query_selector_all(".math-inline").unwrap().for_each(|element| {
//         let element = element.unwrap();
//         render_math_element(&element, false);
//     });
// }

// // 内部函数 - 高亮代码元素
// fn hljs_highlight_element(element: &web_sys::Element) {
//     let _ = js_sys::Reflect::apply(
//         &js_sys::Function::from(js_sys::eval(
//             "return hljs.highlightElement"
//         ).unwrap()),
//         &js_sys::global(),
//         &[element.into()],
//     );
// }

// // 内部函数 - 渲染数学元素
// fn render_math_element(element: &web_sys::Element, display_mode: bool) {
//     let text = element.text_content().unwrap();
//     let options = js_sys::Object::new();
//     let _ = js_sys::Reflect::set(
//         &options,
//         &"throwOnError".into(),
//         &false.into(),
//     );
//     let _ = js_sys::Reflect::set(
//         &options,
//         &"displayMode".into(),
//         &display_mode.into(),
//     );
    
//     let katex = js_sys::eval("return katex").unwrap();
//     let _ = js_sys::Reflect::apply(
//         &js_sys::Function::from(js_sys::Reflect::get(&katex, &"render".into()).unwrap()),
//         &katex,
//         &[
//             &text.into(),
//             element.into(),
//             &options.into(),
//         ],
//     );
// }