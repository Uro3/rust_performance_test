use std::collections::HashMap;
use chrono::Local;

use crate::tag_builder;

pub struct Renderer {
    x_num: i32,
    y_num: i32,
}

impl Renderer {
    pub fn new(x_num: i32, y_num: i32) -> Self {
        Self { x_num, y_num }
    }

    pub fn render(&self) -> String {
        let content = format!("{}{}", self.render_thead(), self.render_tbody());
        let mut data = HashMap::new();
        let timestamp = Local::now().to_string();
        data.insert("timestamp", timestamp.as_str());
        return tag_builder::build("table", &content, None, Some(data), false);
    }

    fn render_thead(&self) -> String {
        let content = tag_builder::build("tr", &self.th_tags(), None, None, false);
        return tag_builder::build("thead", &content, None, None, false);
    }

    fn render_tbody(&self) -> String {
        let mut content = String::new();
        for i in 0..self.y_num {
            content += &tag_builder::build("tr", &self.td_tags(i), None, None, false);
        }
        return tag_builder::build("tbody", &content, None, None, false);
    }

    fn th_tags(&self) -> String {
        let mut tags = String::new();
        for i in 0..self.x_num {
            let x_text = &i.to_string();
            let mut data = HashMap::new();
            data.insert("mdt-x", x_text.as_str());
            tags += &tag_builder::build("th", x_text, Some(&[&format!("mdt-x{}", x_text)]), Some(data), true);
        }
        return tags;
    }

    fn td_tags(&self, num: i32) -> String {
        let mut tags = String::new();
        let y_text = &num.to_string();
        for i in 0..self.x_num {
            let x_text = &i.to_string();
            let mut data = HashMap::new();
            data.insert("mdt-x", x_text.as_str());
            data.insert("mdt-y", y_text.as_str());
            tags += &tag_builder::build("td", x_text, Some(&[&format!("mdt-x{}", x_text), &format!("mdt-y{}", y_text)]), Some(data), true);
        }
        return tags;
    }
}


