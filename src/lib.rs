use std::collections::HashMap;

use regex::Regex;


pub struct Readme{
    params: HashMap<String, String>,
    pub blocks: Vec<String>,

}

impl Readme {
    pub fn new(content: &str) -> Self{
        Self {
            params: HashMap::new(),
            blocks: Self::get_blocks(content),
        }
    }

    fn get_blocks(content: &str) -> Vec<String>{
        let mut blocks: Vec<String> = Vec::new();
        let mut blocks_start = Vec::new();
        let re_start = Regex::new(r"<!--\sstart\s(.*)\s-->").unwrap();
        let re_end = Regex::new(r"<!--\send\s(.*)\s-->").unwrap();
        for cap in re_start.captures_iter(content){
            blocks_start.push(cap[1].to_string());
        }
        for cap in re_end.captures_iter(content){
            if blocks_start.contains(&cap[1].to_string()) &&
                    !blocks.contains(&cap[1].to_string()){
                blocks.push(cap[1].to_string());
            }
        }
        blocks
    }

    fn get_block_content(&self, tag: &str, content: &str) -> String{
        let pattern = format!(r"<!--\sstart\s{c}\s-->\n<!--\n([\S\s]*)-->\n<!--\send\s{c}\s-->", c = content);
        let re = Regex::new(&pattern).unwrap();
        re.captures(content).unwrap()[1].to_string()
    }

}
