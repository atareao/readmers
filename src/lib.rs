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

use regex::Regex;

pub struct Param{
    pub key: String,
    pub value: String,
    pub help: String,
}

impl Param{
    pub fn new(key: &str, value: &str, help: &str) -> Self{
        Self{
            key: key.to_string(),
            value: value.to_string(),
            help: help.to_string(),
        }
    }
}

pub struct Readme{
    pub params: Vec<Param>,
    pub blocks: Vec<String>,

}

impl Readme {
    pub fn new(content: &str) -> Self{
        Self {
            params: Self::get_params(content),
            blocks: Self::get_blocks(content),
        }
    }

    fn get_params(content: &str) -> Vec<Param>{
        let mut params: Vec<Param> = Vec::new();
        let param_block = Self::get_block_content("params", content);
        println!("{}", &param_block);
        let re = Regex::new(r"(.*)\s*\((.*)\)\s*:\s*(.*)\n").unwrap();
        for cap in re.captures_iter(&param_block){
            params.push(Param::new(&cap[1], &cap[2], &cap[3]));
        }
        params
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

    fn get_block_content(tag: &str, content: &str) -> String{
        let pattern = format!(r"<!--\sstart\s{c}\s-->\n<!--\n([\S\s]*)-->\n<!--\send\s{c}\s-->", c = tag);
        let re = Regex::new(&pattern).unwrap();
        re.captures(content).unwrap()[1].to_string()
    }

}
