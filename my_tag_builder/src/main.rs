// cargo run --release

use std::time;
use std::collections::HashMap;
use chrono::Local;

mod tag_builder;

const X_NUM: i32 = 800;
const Y_NUM: i32 = 800;

fn main() {
    // let text = render();
    // println!("{}", text);

    let now = time::Instant::now();
    render();
    println!("{:?}", now.elapsed());
}

fn render() -> String {
    let content = format!("{}{}", render_thead(), render_tbody());
    let mut data = HashMap::new();
    let timestamp = Local::now().to_string();
    data.insert("timestamp", timestamp.as_str());
    return tag_builder::build("table", &content, None, Some(data), false);
}

fn render_thead() -> String {
    let content = tag_builder::build("tr", &th_tags(), None, None, false);
    return tag_builder::build("thead", &content, None, None, false);
}

fn render_tbody() -> String {
    let mut content = String::new();
    for i in 0..Y_NUM {
        content += &tag_builder::build("tr", &td_tags(i), None, None, false);
    }
    return tag_builder::build("tbody", &content, None, None, false);
}

fn th_tags() -> String {
    let mut tags = String::new();
    for i in 0..X_NUM {
        let x_text = &i.to_string();
        let mut data = HashMap::new();
        data.insert("mdt-x", x_text.as_str());
        tags += &tag_builder::build("th", x_text, Some(&[&format!("mdt-x{}", x_text)]), Some(data), true);
    }
    return tags;
}

fn td_tags(num: i32) -> String {
    let mut tags = String::new();
    let y_text = &num.to_string();
    for i in 0..X_NUM {
        let x_text = &i.to_string();
        let mut data = HashMap::new();
        data.insert("mdt-x", x_text.as_str());
        data.insert("mdt-y", y_text.as_str());
        tags += &tag_builder::build("td", x_text, Some(&[&format!("mdt-x{}", x_text), &format!("mdt-y{}", y_text)]), Some(data), true);
    }
    return tags;
}
