use std::borrow::Cow;
use syntect::parsing::{SyntaxSet,ParseState};
use syntect::html::{ClassStyle,tokens_to_classed_html};
use pulldown_cmark::html::push_html;
use pulldown_cmark::{Event,Parser,Tag};

pub fn highlighted_markdown(text: String) -> String {
    let mut in_block = false;
    let mut code_buffer = String::from("");
    let prs = Parser::new(text.as_str())
        .filter_map(|event| match event {
            Event::Start(Tag::CodeBlock(_)) => {
                in_block = true;
                None
            },
            Event::End(Tag::CodeBlock(lang)) => {
                in_block = false;
                let l_str: String = lang.into_owned();
                let ss = SyntaxSet::load_defaults_newlines();
                let mys = ss.find_syntax_by_extension(l_str.as_str()).unwrap_or(ss.find_syntax_plain_text());
                let mut ps = ParseState::new(&mys);
                let mut html_str = String::from("");
                let mut line_numbers = String::from("");
                let mut i:u8 = 1;
                for line in code_buffer.as_str().lines() {
                    let next = ps.parse_line(line);
                    html_str.push_str(tokens_to_classed_html(line, next.as_slice(), ClassStyle::Spaced).as_str());
                    html_str.push('\n');
                    line_numbers.push_str(format!("{}\n", i).as_str());
                    i += 1;
                }
                code_buffer.clear();
                Some(Event::Html(Cow::from(format!(r#"
                    <figure>
                        <table class="highlight-table">
                            <tr>
                                <td class="linenos">
                                    <div class="lineno">
                                        <pre>{}</pre>
                                    </div>
                                </td>
                                <td class="code">
                                    <div class="highlight">
                                        <pre>{}</pre>
                                    </div>
                                </td>
                            </tr>
                        </table>
                    </figure>"#, line_numbers, html_str))))
            },
            Event::Text(mut text) => {
                if in_block {
                    code_buffer.push_str(text.to_mut());
                    None
                } else {
                    Some(Event::Text(Cow::from(text)))
                }
            },
            _ => Some(event)
        });
    let mut html_str = String::from("");
    push_html(&mut html_str, prs);
    html_str
}
