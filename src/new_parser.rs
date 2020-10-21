use comrak::{Arena, parse_document, ComrakOptions};
use comrak::nodes::{Ast, NodeValue::*};
use comrak::arena_tree::Node;
use std::cell::RefCell;

type MarkDown<'a> = Node<'a, RefCell<Ast>>;

const VALID_METHODS: &'static [&'static str] = &[
    "GET",
    "HEAD",
    "POST",
    "PUT",
    "DELETE",
    "PATCH",
];

pub fn parse_request(input: &str) {
    let arena = Arena::new();
    let root = parse_document(&arena, &input, &ComrakOptions::default());
    for node in root.children() {
        if node.is_req_block() {
            println!("{}", node.request_line().unwrap());
            println!("{}", node.headers().join("\n"));
            if let Some(body) = node.request_body() {
                println!("\n{}", body);
            }
        }
    }
}

trait ReqBlock {
    fn is_req_block(&self) -> bool;
    fn request_line(&self) -> Option<String>;
    fn headers(&self) -> Vec<String>;
    fn request_body(&self) -> Option<String>;
}

impl <'a> ReqBlock for &'a MarkDown<'a> {
    fn is_req_block(&self) -> bool {
        if let CodeBlock(code) = &self.data.borrow().value {
            let string = String::from_utf8_lossy(&code.literal);
            return VALID_METHODS.iter().any(|method| string.starts_with(method));
        }

        false
    }

    fn request_line(&self) -> Option<String> {
        if let CodeBlock(code) = &self.data.borrow().value {
            let block = String::from_utf8_lossy(&code.literal);
            return block.lines().nth(0).map(|s| s.to_string());
        }

        None
    }

    fn headers(&self) -> Vec<String> {
        if let CodeBlock(code) = &self.data.borrow().value {
            let block = String::from_utf8_lossy(&code.literal);
            return block
                .lines()
                .skip(1)
                .take_while(|line| line.trim().len() > 0)
                .map(|line| line.to_string())
                .collect();
        }

        vec![]
    }

    fn request_body(&self) -> Option<String> {
        match self.next_sibling() {
            None => None,
            Some(node) => {
                if let CodeBlock(code) = &node.data.borrow().value {
                    return Some(String::from_utf8_lossy(&code.literal).to_string());
                }

                None
            }
        }
    }
}
