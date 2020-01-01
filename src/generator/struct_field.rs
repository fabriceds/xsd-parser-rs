use core::fmt;

use crate::generator::complex_type::{element_type, yaserde_for_attribute, yaserde_for_element};
use crate::generator::utils::{get_field_comment, get_field_name, get_type_name, match_type};
use crate::xsd2::attribute::{Attribute, attribute_type};
use crate::xsd2::extension::Extension;
use crate::xsd2::sequence::{Element, Sequence};

pub struct StructField {
    pub name: String,
    pub typename: String,
    pub comment: String,
    pub macros: String,
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{macros}  pub {name}: {typename},  {comment}",
               macros=self.macros,
               name=self.name,
               typename=self.typename,
               comment=self.comment
        )
    }
}

pub fn any_attribute_field() -> StructField {
    StructField{
        name: "any_attribute".to_string(),
        typename: "AnyAttribute".to_string(),
        comment: String::new(),
        macros: "//TODO: yaserde macros for any attribute\n".to_string()
    }
}

fn any_element_field() -> StructField {
    StructField{
        name: "any_element".to_string(),
        typename: "AnyElement".to_string(),
        macros: "//TODO: yaserde macros for any element\n".to_string(),
        comment: String::new()
    }
}

pub fn field_from_attribute(attr: &Attribute, target_namespace: Option<&str>) -> StructField {
    let name = attr.name();
    StructField{
        name: get_field_name(&name),
        typename: attribute_type(attr, match_type(attr.typename(), target_namespace)),
        macros: yaserde_for_attribute(name),
        comment: get_field_comment(attr.documentation())
    }
}

pub  fn field_from_element(elem: &Element, target_namespace: Option<&str>) -> StructField {
    let name = elem.name();
    StructField{
        name: get_field_name(&name),
        typename: element_type(elem, match_type(elem.typename(), target_namespace)),
        macros: yaserde_for_element(name),
        comment: get_field_comment(elem.documentation())
    }
}

pub fn get_fields_from_sequence(s: &Sequence, target_namespace: Option<&str>) -> Vec<StructField> {
    let mut fields = s.elements().
        iter().
        map(|el| field_from_element(el, target_namespace)).
        collect::<Vec<StructField>>();

    let any = s.any_element();
    match any {
        Some(_) => fields.push(any_element_field()),
        None => ()
    }
    fields
}

pub fn get_fields_from_extension(ext: &Extension, target_namespace: Option<&str>) -> Vec<StructField> {
    let mut fields = match ext.sequence() {
        Some(s) => get_fields_from_sequence(&s, target_namespace),
        None => vec!()
    };

    fields.append(&mut ext.
        attributes().
        iter().
        map(|a| field_from_attribute(a, target_namespace)).
        collect::<Vec<StructField>>()
    );

    let ty = ext.base();
    fields.push(StructField {
        name: "base".to_string(),
        typename: get_type_name(ty, target_namespace),
        macros: yaserde_for_element("base"), //TODO: yaserde for base element
        comment: String::new(),
    });

    if ext.has_any_attribute() {
        fields.push(any_attribute_field());
    };

    fields
}