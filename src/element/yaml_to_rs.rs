use serde::{Deserialize, Serialize};

use super::yaml_parser::ElementYAML;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ElementYamlTable {
    #[serde(rename = "Element_info")]
    pub elements: Vec<ElementYAML>,
}

impl ElementYamlTable {
    /// Helper function to write the type annotation for an array in rust.
    /// # Arguments:
    /// - type_annotation - type of the array item
    /// The size is determined internally with `self.elements.len()`
    /// # Returns:
    /// `"[{type_annotation}; size]"`
    pub fn new_array_type(&self, type_annotation: &str) -> String {
        format!(
            "[{type_annotation}; {length}]",
            length = self.elements.len()
        )
    }
    /// Helper function to write the content of an array in rust.
    /// # Arguments:
    /// - format_closure: `fn(&Element)->String` - The closure which produces desired string with each `&Elm`
    pub fn new_array_content(&self, format_closure: fn(&ElementYAML) -> String) -> String {
        let contents_vec: Vec<String> = self.elements.iter().map(format_closure).collect();
        contents_vec.join(", ")
    }
    /// Helper function to write a line of constant array in rust.
    /// # Arguments:
    /// - var_name: `&str` - Given variable name, all capitalized
    /// - var_type: `&str` - Given variable type annotation
    /// - array_content: '&str' - A string that joined by ", " from `Vec<String>`
    /// ```
    pub fn new_const_array(var_name: &str, var_type: &str, array_content: &str) -> String {
        format!("pub const {var_name}: {var_type} = [{array_content}];")
    }
    pub fn export_struct(&self) -> String {
        let init_element = |elm: &ElementYAML| -> String {
            format!("Element{{ symbol: ElementSymbol::{:?}, atomic_number: {}_u8, lcao: {}_u8, mass: {:?}, potential: \"{}\", spin:{}_u8, covalent_radius: {}\n}}", &elm.symbol, elm.atomic_number, elm.lcao, elm.mass, &elm.potential, elm.spin, if let Some(r) = elm.covalent_radius {format!("Some({})", r)} else {"None".into()})
            // Debug formatter is used for mass to avoid making f64 numbers like `147.0` to `147`
        };
        let var_name = "ELEMENT_TABLE";
        let var_type = self.new_array_type("Element");
        let array_content = self.new_array_content(init_element);
        Self::new_const_array(var_name, &var_type, &array_content)
    }
}
