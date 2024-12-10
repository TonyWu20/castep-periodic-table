use std::fmt::Display;

use serde::Serialize;

mod de;
mod error;

pub use error::SymbolError;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Serialize, Clone, Copy, Hash)]
pub enum ElementSymbol {
    H,
    He = 2,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
}

impl Display for ElementSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq<&str> for ElementSymbol {
    fn eq(&self, other: &&str) -> bool {
        let symbol = format!("{}", self);
        symbol.as_str() == *other
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementFamily {
    TransitionMetal3d,
    TransitionMetal4d,
    TransitionMetal5d,
    RareEarthLa,
    RareEarthAc,
    Else,
}

impl ElementSymbol {
    pub fn family(&self) -> ElementFamily {
        let atomic_number = *self as usize;
        match atomic_number {
            21..=30 => ElementFamily::TransitionMetal3d,
            39..=48 => ElementFamily::TransitionMetal4d,
            72..=80 => ElementFamily::TransitionMetal5d,
            57..=71 => ElementFamily::RareEarthLa,
            89..=103 => ElementFamily::RareEarthAc,
            _ => ElementFamily::Else,
        }
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use super::ElementSymbol;

    #[test]
    fn test_serde() {
        let input = "Pt";
        let symbol = ElementSymbol::from_str(input);
        assert!(symbol.is_ok());
        println!("{:?}", symbol.unwrap());
        let input = 3_u32;
        let symbol = ElementSymbol::try_from(input);
        assert!(symbol.is_ok());
        println!("{:?}", symbol.unwrap());
    }
}
