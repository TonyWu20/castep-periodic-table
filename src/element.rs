use serde::{Deserialize, Serialize};

extern crate serde;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementYAML {
    pub symbol: String,
    pub atomic_number: u8,
    #[serde(rename = "LCAO")]
    pub lcao: u8,
    pub mass: f64,
    pub potential: String,
    pub spin: u8,
    pub covalent_radius: Option<f64>,
}

impl Eq for ElementYAML {}

impl PartialEq for ElementYAML {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_number == other.atomic_number
    }
}

impl Ord for ElementYAML {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.atomic_number.cmp(&other.atomic_number)
    }
}

impl PartialOrd for ElementYAML {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub symbol: &'static str,
    pub atomic_number: u8,
    pub lcao: u8,
    pub mass: f64,
    pub potential: &'static str,
    pub spin: u8,
    pub covalent_radius: Option<f64>,
}

impl Element {
    pub fn symbol(&self) -> &str {
        self.symbol
    }

    pub fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub fn lcao(&self) -> u8 {
        self.lcao
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn potential(&self) -> &str {
        self.potential
    }

    pub fn spin(&self) -> u8 {
        self.spin
    }

    pub fn covalent_radius(&self) -> Option<f64> {
        self.covalent_radius
    }
}
impl Eq for Element {}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_number == other.atomic_number
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.atomic_number.cmp(&other.atomic_number)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

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
            format!("Element{{ symbol: \"{}\", atomic_number: {}_u8, lcao: {}_u8, mass: {:?}, potential: \"{}\", spin:{}_u8, covalent_radius: {}\n}}", &elm.symbol, elm.atomic_number, elm.lcao, elm.mass, &elm.potential, elm.spin, if let Some(r) = elm.covalent_radius {format!("Some({})", r)} else {"None".into()})
            // Debug formatter is used for mass to avoid making f64 numbers like `147.0` to `147`
        };
        let var_name = "ELEMENT_TABLE";
        let var_type = self.new_array_type("Element");
        let array_content = self.new_array_content(init_element);
        Self::new_const_array(var_name, &var_type, &array_content)
    }
}

pub trait LookupElement {
    fn get_by_symbol(&self, symbol: &str) -> Option<&Element>;
    fn get_by_atomic_number(&self, atomic_number: u8) -> Option<&Element>;
}

impl LookupElement for [Element] {
    fn get_by_symbol(&self, symbol: &str) -> Option<&Element> {
        self.iter().find(|item| item.symbol() == symbol)
    }

    fn get_by_atomic_number(&self, atomic_number: u8) -> Option<&Element> {
        self.iter()
            .find(|item| item.atomic_number() == atomic_number)
    }
}
