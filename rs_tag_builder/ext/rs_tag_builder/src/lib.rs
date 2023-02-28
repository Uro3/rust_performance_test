use magnus::{define_module, function, Error};

use std::collections::HashMap;
use html_escape;

fn build_tag(name: String, content: String, classes: Option<Vec<String>>, data: Option<HashMap<String, String>>, encode_content: bool) -> String {
    let content_text = match encode_content {
        true => encode_safe(&content),
        false => content,
    };

    let class_attr = match classes {
        None => String::from(""),
        Some(c) => format!(" class=\"{}\"", encode_safe(&c.join(" "))),
    };

    let data_attrs = match data {
        None => String::from(""),
        Some(d) => {
            let mut buf = String::new();
            for (key, value) in &d {
                buf += &format!(" data-{}=\"{}\"", encode_safe(&key), encode_safe(&value));
            }
            buf
        },
    };

    return format!("<{name}{class_attr}{data_attrs}>{content}</{name}>", name=name, content=content_text, class_attr=class_attr, data_attrs=data_attrs);
}

fn encode_safe(text: &str) -> String {
    return html_escape::encode_safe(text).to_string();
}


#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RsTagBuilder")?;
    module.define_module_function("build_tag", function!(build_tag, 5))?;
    Ok(())
}
