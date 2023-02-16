// cargo run --release

use std::time;
use chrono::Local;
use html_escape;

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
    return format!("<table data-timestamp=\"{}\">{}{}</table>", encode_safe(Local::now().to_string()), render_thead(), render_tbody());
}

fn render_thead() -> String {
    return format!("<thead><tr>{}</tr></thead>", th_tags())
}

fn render_tbody() -> String {
    let mut body = String::from("<tbody>");
    for i in 0..Y_NUM {
        body += format!("<tr>{}</tr>", td_tags(i)).as_str();
    }
    return body + "</tbody>";
}

fn th_tags() -> String {
    let mut tags = String::new();
    for i in 0..X_NUM {
        tags += format!("<th class=\"mdt-x{x_num}\" data-mdt-x=\"{x_num}\">{x_num}</th>", x_num=encode_safe(i.to_string())).as_str();
    }
    return tags;
}

fn td_tags(num: i32) -> String {
    let mut tags = String::new();
    let encoded_num = encode_safe(num.to_string());
    for i in 0..X_NUM {
        tags += format!("<td class=\"mdt-x{x_num} mdt-y{y_num}\" data-mdt-x=\"{x_num}\" data-mdt-y=\"{y_num}\">{y_num}-{x_num}</td>", x_num=encode_safe(i.to_string()), y_num=encoded_num).as_str();
    }
    return tags;
}

fn encode_safe(text: String) -> String {
    return html_escape::encode_safe(&text).to_string();
}
