mod lib;

use std::fs;
use std::io::Error;
use html_editor::operation::{Selector, Queryable};
use html_editor::parse;
use html_editor::Node::Text;
use crate::lib::Readme;

use html_editor;


fn main() {
    let content = read_file("template.md").unwrap();
    // println!("{:?}", content);
    let mut dom = parse(&content).unwrap();
    let selector = Selector::from("#project_title");
    let elements = dom.query_all(&selector);
    for mut element in elements {
        element.children = [Text("Nada".to_string())].to_vec();
        // println!("{:?}", element);
    }
    // println!("{:?}", dom);

    let readme = Readme::new(&content);
    println!("{:?}", readme.blocks)



}

fn read_file(filename: &str) -> Result<String, Error>{
    fs::read_to_string(filename)
}
