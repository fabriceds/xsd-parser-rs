
use roxmltree::*;

pub fn match_type(s: &str) -> &str {
    match s {
        "xs:string" => "String",
        "xs:NCName" => "String",
        "xs:unsignedInt" => "usize",
        "xs:int" => "i64",
        "xs:float" => "f64",
        "xs:boolean" => "bool",
        x => x
    }
}


pub fn lowercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().chain(c).collect(),
    }
}

pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn find_child<'a>(node: &'a Node, tag_name: &str) -> Option<Node<'a, 'a>> {
    node.children().find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn find_element<'a>(node: &'a Node, tag_name: &str) -> Option<Node<'a, 'a>> {
    match node.
        traverse().
        find(|e| match e {
            Edge::Open(x) => x.is_element() && x.tag_name().name() == tag_name,
            _ => false
        })
    {
        Some(Edge::Open(node)) => Some(node.clone()),
        _ => None
    }
}

pub fn get_documentation<'a>(node: &'a Node) -> Option<&'a str> {
    match find_element(node, "documentation") {
        Some(node) => node.text(),
        None => None
    }
}

pub fn get_struct_comment(doc: Option<&str>) -> String {
    doc.
        unwrap_or("").
        lines().
        map(|s| s.trim()).
        filter(|s| s.len() > 2).
        map(|s| format!("// {}\n", s)).
        fold(String::new(), |x , y| (x+&y))
}

pub fn get_field_comment(doc: &Option<String>) -> String {
    doc.as_ref().
        unwrap_or(&"".to_string()).
        lines().
        map(|s| s.trim()).
        filter(|s| s.len() > 1).
        map(|s| format!("{}  ", s)).
        fold(String::from(""), |x , y| (x+&y))
}