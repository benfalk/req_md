use comrak::{Arena, parse_document, ComrakOptions};
use comrak::nodes::NodeValue;

pub fn parse_request(input: &str) {
    let arena = Arena::new();
    let root = parse_document(&arena, &input, &ComrakOptions::default());
    for node in root.children() {
        match node.data.borrow().value {
            NodeValue::CodeBlock(ref code) => {
                let string = String::from_utf8_lossy(&code.literal);
                println!("info: {}", String::from_utf8_lossy(&code.info));
                println!("{:?}", code);
                println!("{}", string.trim());
            }
            //ref what => println!("F {:?}", what)
            _ => ()
        }
        //println!("{:?}", node.data.borrow().value);
    }
}
