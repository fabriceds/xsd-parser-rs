use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::types::{RsEntity, Struct};
use crate::parser::utils::{elements_to_fields, get_documentation, get_parent_name};

pub fn parse_sequence(sequence: &Node, parent: &Node) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(Struct {
        name: name.into(),
        comment: get_documentation(parent),
        subtypes: vec![],
        fields: RefCell::new(elements_to_fields(sequence, name)),
        ..Default::default()
    })
}
