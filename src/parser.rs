use crate::req::Request;
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, NodeValue::*};
use comrak::{parse_document, Arena, ComrakOptions};
use std::cell::RefCell;
use url::{Position, Url};

type MarkDown<'a> = Node<'a, RefCell<Ast>>;

const VALID_METHODS: &'static [&'static str] = &["GET", "HEAD", "POST", "PUT", "DELETE", "PATCH"];

pub fn parse_request(input: &str) -> Option<Request> {
    let arena = Arena::new();

    let node = parse_document(&arena, &input, &ComrakOptions::default())
        .children()
        .find(|node| node.is_req_block())?;

    Some(Request {
        method: node.request_method()?,
        uri: node.request_uri()?,
        host: node.host()?,
        headers: node.headers(),
        body: node.request_body(),
    })
}

trait ReqBlock {
    fn request_method(&self) -> Option<String> {
        let req_line = self.request_line()?;

        VALID_METHODS
            .iter()
            .find(|method| req_line.starts_with(*method))
            .map(|method| method.to_string())
    }

    fn request_uri(&self) -> Option<String> {
        self.request_line()?
            .split_whitespace()
            .nth(1)
            .map(|uri| uri.to_string())
    }

    fn host(&self) -> Option<String> {
        let uri = self.request_uri()?;
        if let Ok(url) = Url::parse(&uri) {
            return Some(url[..Position::BeforePath].to_string());
        }

        self.headers()
            .iter()
            .find(|header| header.to_lowercase().starts_with("host: "))
            .map(|header| header[6..].to_string())
    }

    fn is_req_block(&self) -> bool;
    fn request_line(&self) -> Option<String>;
    fn headers(&self) -> Vec<String>;
    fn request_body(&self) -> Option<String>;
}

impl<'a> ReqBlock for &'a MarkDown<'a> {
    fn is_req_block(&self) -> bool {
        if let CodeBlock(code) = &self.data.borrow().value {
            let string = String::from_utf8_lossy(&code.literal);
            return VALID_METHODS
                .iter()
                .any(|method| string.starts_with(method));
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
        match &self.next_sibling()?.data.borrow().value {
            CodeBlock(code) => Some(String::from_utf8_lossy(&code.literal).to_string()),
            _ => None,
        }
    }
}
