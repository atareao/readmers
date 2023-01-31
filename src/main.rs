// The MIT License (MIT)
//
// Copyright (c) 2022 Lorenzo Carbonell <a.k.a atareao>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod lib;

use std::fs;
use std::io::Error;
use html_editor::operation::{Selector, Queryable};
use html_editor::parse;
use html_editor::Node::Text;
use crate::lib::Readme;
use requestty::{prompt, Question};

fn main() {
    let badgets = include_str!("resources/badgets.md");
    println!("Badgets: {}", badgets);
}

fn menu(){
    let questions = vec![
        Question::select("option")
            .message("Select option")
            .choices(vec![
                "Open README file".to_string(),
                "Create README file".to_string(),
            ])
            .build(),
    ];
    let selection = prompt(questions).unwrap();
    println!("{:?}", selection);
    if selection.contains_key("option"){
        if let Some(selected) = selection.get("option"){
            if selected.as_list_item().unwrap().index == 0{
                select_files();
            }
        }
    }
}

fn select_files(){
    let paths = fs::read_dir("./").unwrap();
    let mut filenames: Vec<String> = Vec::new();
    for path in paths{
        filenames.push(path.unwrap().path().display().to_string());
    }
    let questions = vec![
        Question::select("file")
            .message("Select file")
            .choices(filenames)
            .build(),
    ];
    let selection = prompt(questions).unwrap();
}

fn read_file(filename: &str) -> Result<String, Error>{
    fs::read_to_string(filename)
}

fn open_file(filename: &str){
    let content = read_file(filename).unwrap();
    // println!("{:?}", content);
    let dom = parse(&content).unwrap();
    let selector = Selector::from("#project_title");
    let elements = dom.query_all(&selector);
    for mut element in elements {
        element.children = [Text("Nada".to_string())].to_vec();
        // println!("{:?}", element);
    }
    // println!("{:?}", dom);

    let readme = Readme::new(&content);
    println!("{:?}", readme.blocks);
    println!("{:?}", readme.params);

    let mut questions: Vec<Question> = Vec::new();

    for param in readme.params{
        let mut message: String;
        if !param.help.is_empty(){
            message = param.help.to_string();
        }else{
            message = param.key.to_string();
        }
        if !param.value.is_empty(){
            message = format!("{} ({})", &message, &param.value);
        }
        questions.push(Question::input(param.key)
                       .message(&message)
                       .build())
    }
    let result = prompt(questions);
}
