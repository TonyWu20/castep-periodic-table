use std::{fmt::Display, str::FromStr};

use crate::data::ELEMENT_TABLE;

use super::ElementSymbol;

#[derive(Debug, Clone)]
pub struct Element {
    pub symbol: ElementSymbol,
    pub atomic_number: u8,
    pub lcao: u8,
    pub mass: f64,
    pub potential: &'static str,
    pub spin: u8,
    pub covalent_radius: Option<f64>,
}

impl Element {
    pub fn symbol(&self) -> String {
        format!("{:?}", self.symbol)
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ParseElementErr;

impl Display for ParseElementErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid element symbol!")
    }
}

impl FromStr for Element {
    type Err = ParseElementErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ELEMENT_TABLE
            .get_by_symbol(s)
            .cloned()
            .ok_or(ParseElementErr)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.atomic_number(), self.symbol())
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
