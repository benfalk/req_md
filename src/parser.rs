use crate::req::{Request, Meta};
use comrak::arena_tree::Node;
use comrak::nodes::{Ast, NodeValue::*};
use comrak::{parse_document, Arena, ComrakOptions};
use std::cell::RefCell;
use std::ops::Range;
use url::{Position, Url};

type MarkDown<'a> = Node<'a, RefCell<Ast>>;

const VALID_METHODS: &'static [&'static str] = &["GET", "HEAD", "POST", "PUT", "DELETE", "PATCH"];

pub fn parse_requests(input: &str) -> Vec<Request> {
    let arena = Arena::new();

    parse_document(&arena, &input, &ComrakOptions::default())
    .children()
    .filter(|node| node.is_req_block())
    .map(|node| node.to_request())
    .flatten()
    .collect()
}

trait ReqBlock {
    fn to_request(&self) -> Option<Request> {
        let meta = Meta {
            line_range: self.line_range().unwrap_or(0..0),
            // TODO: Come up with a way to set timeout in
            // the markdown dock
            timeout: None
        };

        Some(Request {
            method: self.request_method()?,
            uri: self.request_uri()?,
            host: self.host()?,
            headers: self.headers(),
            body: self.request_body(),
            meta,
        })
    }

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
    fn line_range(&self) -> Option<Range<u32>>;
}

trait SourceRange {
    fn source_range(&self) -> Option<Range<u32>>;
}

trait NodeInterrogation {
    fn is_a_code_block(&self) -> bool;
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
            let lines = lines_for_req_line(&block);
            let mut req_line = String::new();

            block
                .lines()
                .take(lines)
                .for_each(|line| {
                    req_line += line.trim();
                });

            return Some(req_line);
        }

        None
    }

    fn headers(&self) -> Vec<String> {
        if let CodeBlock(code) = &self.data.borrow().value {
            let block = String::from_utf8_lossy(&code.literal);
            let lines = lines_for_req_line(&block);

            return block
                .lines()
                .skip(lines)
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

    fn line_range(&self) -> Option<Range<u32>> {
        let range = self.source_range()?;

        match self.next_sibling() {
            None => Some(range),
            Some(node) => {
                if node.is_a_code_block() {
                    Some(range.start..(node.source_range()?.end))
                } else {
                    Some(range)
                }
            }
        }
    }
}

fn lines_for_req_line(body: &str) -> usize {
    body
        .lines()
        .skip(1)
        .take_while(|line| {
            let trimed = line.trim();
            trimed.starts_with("?") || trimed.starts_with("&")
        })
        .count() + 1
}

impl<'a> SourceRange for &'a MarkDown<'a> {
    fn source_range(&self) -> Option<Range<u32>> {
        let start = self.data.borrow().start_line;

        let lines =
            match &self.data.borrow().value {
                CodeBlock(code) =>
                    String::from_utf8_lossy(&code.literal).lines().count() as u32 + 1,
                _ => return None,
            };

        Some(start..(start+lines+1))
    }
}

impl<'a> NodeInterrogation for &'a MarkDown<'a> {
    fn is_a_code_block(&self) -> bool {
        match self.data.borrow().value {
            CodeBlock(_) => true,
            _ => false,
        }
    }
}
