use serde::{Deserialize, Serialize};

use super::element_symbol::ElementSymbol;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementYAML {
    pub symbol: ElementSymbol,
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
