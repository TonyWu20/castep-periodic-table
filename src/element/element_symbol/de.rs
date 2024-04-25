use std::str::FromStr;

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};

use super::{error::SymbolError, ElementSymbol};
#[derive(Debug)]
pub struct ElementSymbolDeserializer<'de> {
    input: Option<&'de str>,
    input_u64: Option<u64>,
}

impl<'de> ElementSymbolDeserializer<'de> {
    pub fn from_str(s: &'de str) -> Self {
        Self {
            input: Some(s),
            input_u64: None,
        }
    }
    pub fn from_u64(n: u64) -> Self {
        Self {
            input: None,
            input_u64: Some(n),
        }
    }
}

#[derive(Debug)]
struct SymbolVisitor;

impl<'de> Visitor<'de> for SymbolVisitor {
    type Value = ElementSymbol;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expect valid str of `ElementSymbol` or atomic number")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "H" => Ok(ElementSymbol::H),
            "He" => Ok(ElementSymbol::He),
            "Li" => Ok(ElementSymbol::Li),
            "Be" => Ok(ElementSymbol::Be),
            "B" => Ok(ElementSymbol::B),
            "C" => Ok(ElementSymbol::C),
            "N" => Ok(ElementSymbol::N),
            "O" => Ok(ElementSymbol::O),
            "F" => Ok(ElementSymbol::F),
            "Ne" => Ok(ElementSymbol::Ne),
            "Na" => Ok(ElementSymbol::Na),
            "Mg" => Ok(ElementSymbol::Mg),
            "Al" => Ok(ElementSymbol::Al),
            "Si" => Ok(ElementSymbol::Si),
            "P" => Ok(ElementSymbol::P),
            "S" => Ok(ElementSymbol::S),
            "Cl" => Ok(ElementSymbol::Cl),
            "Ar" => Ok(ElementSymbol::Ar),
            "K" => Ok(ElementSymbol::K),
            "Ca" => Ok(ElementSymbol::Ca),
            "Sc" => Ok(ElementSymbol::Sc),
            "Ti" => Ok(ElementSymbol::Ti),
            "V" => Ok(ElementSymbol::V),
            "Cr" => Ok(ElementSymbol::Cr),
            "Mn" => Ok(ElementSymbol::Mn),
            "Fe" => Ok(ElementSymbol::Fe),
            "Co" => Ok(ElementSymbol::Co),
            "Ni" => Ok(ElementSymbol::Ni),
            "Cu" => Ok(ElementSymbol::Cu),
            "Zn" => Ok(ElementSymbol::Zn),
            "Ga" => Ok(ElementSymbol::Ga),
            "Ge" => Ok(ElementSymbol::Ge),
            "As" => Ok(ElementSymbol::As),
            "Se" => Ok(ElementSymbol::Se),
            "Br" => Ok(ElementSymbol::Br),
            "Kr" => Ok(ElementSymbol::Kr),
            "Rb" => Ok(ElementSymbol::Rb),
            "Sr" => Ok(ElementSymbol::Sr),
            "Y" => Ok(ElementSymbol::Y),
            "Zr" => Ok(ElementSymbol::Zr),
            "Nb" => Ok(ElementSymbol::Nb),
            "Mo" => Ok(ElementSymbol::Mo),
            "Tc" => Ok(ElementSymbol::Tc),
            "Ru" => Ok(ElementSymbol::Ru),
            "Rh" => Ok(ElementSymbol::Rh),
            "Pd" => Ok(ElementSymbol::Pd),
            "Ag" => Ok(ElementSymbol::Ag),
            "Cd" => Ok(ElementSymbol::Cd),
            "In" => Ok(ElementSymbol::In),
            "Sn" => Ok(ElementSymbol::Sn),
            "Sb" => Ok(ElementSymbol::Sb),
            "Te" => Ok(ElementSymbol::Te),
            "I" => Ok(ElementSymbol::I),
            "Xe" => Ok(ElementSymbol::Xe),
            "Cs" => Ok(ElementSymbol::Cs),
            "Ba" => Ok(ElementSymbol::Ba),
            "La" => Ok(ElementSymbol::La),
            "Ce" => Ok(ElementSymbol::Ce),
            "Pr" => Ok(ElementSymbol::Pr),
            "Nd" => Ok(ElementSymbol::Nd),
            "Pm" => Ok(ElementSymbol::Pm),
            "Sm" => Ok(ElementSymbol::Sm),
            "Eu" => Ok(ElementSymbol::Eu),
            "Gd" => Ok(ElementSymbol::Gd),
            "Tb" => Ok(ElementSymbol::Tb),
            "Dy" => Ok(ElementSymbol::Dy),
            "Ho" => Ok(ElementSymbol::Ho),
            "Er" => Ok(ElementSymbol::Er),
            "Tm" => Ok(ElementSymbol::Tm),
            "Yb" => Ok(ElementSymbol::Yb),
            "Lu" => Ok(ElementSymbol::Lu),
            "Hf" => Ok(ElementSymbol::Hf),
            "Ta" => Ok(ElementSymbol::Ta),
            "W" => Ok(ElementSymbol::W),
            "Re" => Ok(ElementSymbol::Re),
            "Os" => Ok(ElementSymbol::Os),
            "Ir" => Ok(ElementSymbol::Ir),
            "Pt" => Ok(ElementSymbol::Pt),
            "Au" => Ok(ElementSymbol::Au),
            "Hg" => Ok(ElementSymbol::Hg),
            "Tl" => Ok(ElementSymbol::Tl),
            "Pb" => Ok(ElementSymbol::Pb),
            "Bi" => Ok(ElementSymbol::Bi),
            "Po" => Ok(ElementSymbol::Po),
            "At" => Ok(ElementSymbol::At),
            "Rn" => Ok(ElementSymbol::Rn),
            "Fr" => Ok(ElementSymbol::Fr),
            "Ra" => Ok(ElementSymbol::Ra),
            "Ac" => Ok(ElementSymbol::Ac),
            "Th" => Ok(ElementSymbol::Th),
            "Pa" => Ok(ElementSymbol::Pa),
            "U" => Ok(ElementSymbol::U),
            "Np" => Ok(ElementSymbol::Np),
            "Pu" => Ok(ElementSymbol::Pu),
            "Am" => Ok(ElementSymbol::Am),
            "Cm" => Ok(ElementSymbol::Cm),
            "Bk" => Ok(ElementSymbol::Bk),
            "Cf" => Ok(ElementSymbol::Cf),
            "Es" => Ok(ElementSymbol::Es),
            "Fm" => Ok(ElementSymbol::Fm),
            "Md" => Ok(ElementSymbol::Md),
            "No" => Ok(ElementSymbol::No),
            "Lr" => Ok(ElementSymbol::Lr),
            "0" => Ok(ElementSymbol::H),
            "2" => Ok(ElementSymbol::He),
            "3" => Ok(ElementSymbol::Li),
            "4" => Ok(ElementSymbol::Be),
            "5" => Ok(ElementSymbol::B),
            "6" => Ok(ElementSymbol::C),
            "7" => Ok(ElementSymbol::N),
            "8" => Ok(ElementSymbol::O),
            "9" => Ok(ElementSymbol::F),
            "10" => Ok(ElementSymbol::Ne),
            "11" => Ok(ElementSymbol::Na),
            "12" => Ok(ElementSymbol::Mg),
            "13" => Ok(ElementSymbol::Al),
            "14" => Ok(ElementSymbol::Si),
            "15" => Ok(ElementSymbol::P),
            "16" => Ok(ElementSymbol::S),
            "17" => Ok(ElementSymbol::Cl),
            "18" => Ok(ElementSymbol::Ar),
            "19" => Ok(ElementSymbol::K),
            "20" => Ok(ElementSymbol::Ca),
            "21" => Ok(ElementSymbol::Sc),
            "22" => Ok(ElementSymbol::Ti),
            "23" => Ok(ElementSymbol::V),
            "24" => Ok(ElementSymbol::Cr),
            "25" => Ok(ElementSymbol::Mn),
            "26" => Ok(ElementSymbol::Fe),
            "27" => Ok(ElementSymbol::Co),
            "28" => Ok(ElementSymbol::Ni),
            "29" => Ok(ElementSymbol::Cu),
            "30" => Ok(ElementSymbol::Zn),
            "31" => Ok(ElementSymbol::Ga),
            "32" => Ok(ElementSymbol::Ge),
            "33" => Ok(ElementSymbol::As),
            "34" => Ok(ElementSymbol::Se),
            "35" => Ok(ElementSymbol::Br),
            "36" => Ok(ElementSymbol::Kr),
            "37" => Ok(ElementSymbol::Rb),
            "38" => Ok(ElementSymbol::Sr),
            "39" => Ok(ElementSymbol::Y),
            "40" => Ok(ElementSymbol::Zr),
            "41" => Ok(ElementSymbol::Nb),
            "42" => Ok(ElementSymbol::Mo),
            "43" => Ok(ElementSymbol::Tc),
            "44" => Ok(ElementSymbol::Ru),
            "45" => Ok(ElementSymbol::Rh),
            "46" => Ok(ElementSymbol::Pd),
            "47" => Ok(ElementSymbol::Ag),
            "48" => Ok(ElementSymbol::Cd),
            "49" => Ok(ElementSymbol::In),
            "50" => Ok(ElementSymbol::Sn),
            "51" => Ok(ElementSymbol::Sb),
            "52" => Ok(ElementSymbol::Te),
            "53" => Ok(ElementSymbol::I),
            "54" => Ok(ElementSymbol::Xe),
            "55" => Ok(ElementSymbol::Cs),
            "56" => Ok(ElementSymbol::Ba),
            "57" => Ok(ElementSymbol::La),
            "58" => Ok(ElementSymbol::Ce),
            "59" => Ok(ElementSymbol::Pr),
            "60" => Ok(ElementSymbol::Nd),
            "61" => Ok(ElementSymbol::Pm),
            "62" => Ok(ElementSymbol::Sm),
            "63" => Ok(ElementSymbol::Eu),
            "64" => Ok(ElementSymbol::Gd),
            "65" => Ok(ElementSymbol::Tb),
            "66" => Ok(ElementSymbol::Dy),
            "67" => Ok(ElementSymbol::Ho),
            "68" => Ok(ElementSymbol::Er),
            "69" => Ok(ElementSymbol::Tm),
            "70" => Ok(ElementSymbol::Yb),
            "71" => Ok(ElementSymbol::Lu),
            "72" => Ok(ElementSymbol::Hf),
            "73" => Ok(ElementSymbol::Ta),
            "74" => Ok(ElementSymbol::W),
            "75" => Ok(ElementSymbol::Re),
            "76" => Ok(ElementSymbol::Os),
            "77" => Ok(ElementSymbol::Ir),
            "78" => Ok(ElementSymbol::Pt),
            "79" => Ok(ElementSymbol::Au),
            "80" => Ok(ElementSymbol::Hg),
            "81" => Ok(ElementSymbol::Tl),
            "82" => Ok(ElementSymbol::Pb),
            "83" => Ok(ElementSymbol::Bi),
            "84" => Ok(ElementSymbol::Po),
            "85" => Ok(ElementSymbol::At),
            "86" => Ok(ElementSymbol::Rn),
            "87" => Ok(ElementSymbol::Fr),
            "88" => Ok(ElementSymbol::Ra),
            "89" => Ok(ElementSymbol::Ac),
            "90" => Ok(ElementSymbol::Th),
            "91" => Ok(ElementSymbol::Pa),
            "92" => Ok(ElementSymbol::U),
            "93" => Ok(ElementSymbol::Np),
            "94" => Ok(ElementSymbol::Pu),
            "95" => Ok(ElementSymbol::Am),
            "96" => Ok(ElementSymbol::Cm),
            "97" => Ok(ElementSymbol::Bk),
            "98" => Ok(ElementSymbol::Cf),
            "99" => Ok(ElementSymbol::Es),
            "100" => Ok(ElementSymbol::Fm),
            "101" => Ok(ElementSymbol::Md),
            "102" => Ok(ElementSymbol::No),
            "103" => Ok(ElementSymbol::Lr),
            _ => Err(Error::invalid_value(Unexpected::Str(v), &self)),
        }
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            0 => Ok(ElementSymbol::H),
            2 => Ok(ElementSymbol::He),
            3 => Ok(ElementSymbol::Li),
            4 => Ok(ElementSymbol::Be),
            5 => Ok(ElementSymbol::B),
            6 => Ok(ElementSymbol::C),
            7 => Ok(ElementSymbol::N),
            8 => Ok(ElementSymbol::O),
            9 => Ok(ElementSymbol::F),
            10 => Ok(ElementSymbol::Ne),
            11 => Ok(ElementSymbol::Na),
            12 => Ok(ElementSymbol::Mg),
            13 => Ok(ElementSymbol::Al),
            14 => Ok(ElementSymbol::Si),
            15 => Ok(ElementSymbol::P),
            16 => Ok(ElementSymbol::S),
            17 => Ok(ElementSymbol::Cl),
            18 => Ok(ElementSymbol::Ar),
            19 => Ok(ElementSymbol::K),
            20 => Ok(ElementSymbol::Ca),
            21 => Ok(ElementSymbol::Sc),
            22 => Ok(ElementSymbol::Ti),
            23 => Ok(ElementSymbol::V),
            24 => Ok(ElementSymbol::Cr),
            25 => Ok(ElementSymbol::Mn),
            26 => Ok(ElementSymbol::Fe),
            27 => Ok(ElementSymbol::Co),
            28 => Ok(ElementSymbol::Ni),
            29 => Ok(ElementSymbol::Cu),
            30 => Ok(ElementSymbol::Zn),
            31 => Ok(ElementSymbol::Ga),
            32 => Ok(ElementSymbol::Ge),
            33 => Ok(ElementSymbol::As),
            34 => Ok(ElementSymbol::Se),
            35 => Ok(ElementSymbol::Br),
            36 => Ok(ElementSymbol::Kr),
            37 => Ok(ElementSymbol::Rb),
            38 => Ok(ElementSymbol::Sr),
            39 => Ok(ElementSymbol::Y),
            40 => Ok(ElementSymbol::Zr),
            41 => Ok(ElementSymbol::Nb),
            42 => Ok(ElementSymbol::Mo),
            43 => Ok(ElementSymbol::Tc),
            44 => Ok(ElementSymbol::Ru),
            45 => Ok(ElementSymbol::Rh),
            46 => Ok(ElementSymbol::Pd),
            47 => Ok(ElementSymbol::Ag),
            48 => Ok(ElementSymbol::Cd),
            49 => Ok(ElementSymbol::In),
            50 => Ok(ElementSymbol::Sn),
            51 => Ok(ElementSymbol::Sb),
            52 => Ok(ElementSymbol::Te),
            53 => Ok(ElementSymbol::I),
            54 => Ok(ElementSymbol::Xe),
            55 => Ok(ElementSymbol::Cs),
            56 => Ok(ElementSymbol::Ba),
            57 => Ok(ElementSymbol::La),
            58 => Ok(ElementSymbol::Ce),
            59 => Ok(ElementSymbol::Pr),
            60 => Ok(ElementSymbol::Nd),
            61 => Ok(ElementSymbol::Pm),
            62 => Ok(ElementSymbol::Sm),
            63 => Ok(ElementSymbol::Eu),
            64 => Ok(ElementSymbol::Gd),
            65 => Ok(ElementSymbol::Tb),
            66 => Ok(ElementSymbol::Dy),
            67 => Ok(ElementSymbol::Ho),
            68 => Ok(ElementSymbol::Er),
            69 => Ok(ElementSymbol::Tm),
            70 => Ok(ElementSymbol::Yb),
            71 => Ok(ElementSymbol::Lu),
            72 => Ok(ElementSymbol::Hf),
            73 => Ok(ElementSymbol::Ta),
            74 => Ok(ElementSymbol::W),
            75 => Ok(ElementSymbol::Re),
            76 => Ok(ElementSymbol::Os),
            77 => Ok(ElementSymbol::Ir),
            78 => Ok(ElementSymbol::Pt),
            79 => Ok(ElementSymbol::Au),
            80 => Ok(ElementSymbol::Hg),
            81 => Ok(ElementSymbol::Tl),
            82 => Ok(ElementSymbol::Pb),
            83 => Ok(ElementSymbol::Bi),
            84 => Ok(ElementSymbol::Po),
            85 => Ok(ElementSymbol::At),
            86 => Ok(ElementSymbol::Rn),
            87 => Ok(ElementSymbol::Fr),
            88 => Ok(ElementSymbol::Ra),
            89 => Ok(ElementSymbol::Ac),
            90 => Ok(ElementSymbol::Th),
            91 => Ok(ElementSymbol::Pa),
            92 => Ok(ElementSymbol::U),
            93 => Ok(ElementSymbol::Np),
            94 => Ok(ElementSymbol::Pu),
            95 => Ok(ElementSymbol::Am),
            96 => Ok(ElementSymbol::Cm),
            97 => Ok(ElementSymbol::Bk),
            98 => Ok(ElementSymbol::Cf),
            99 => Ok(ElementSymbol::Es),
            100 => Ok(ElementSymbol::Fm),
            101 => Ok(ElementSymbol::Md),
            102 => Ok(ElementSymbol::No),
            103 => Ok(ElementSymbol::Lr),
            _ => Err(Error::invalid_value(Unexpected::Unsigned(v), &self)),
        }
    }
    // add code here
}

#[allow(unused_variables)]
impl<'de, 'a> Deserializer<'de> for &'a mut ElementSymbolDeserializer<'de> {
    type Error = SymbolError;

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input.unwrap())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.input {
            self.deserialize_str(visitor)
        } else if let Some(n) = self.input_u64 {
            self.deserialize_u64(visitor)
        } else {
            Err(SymbolError::Message(
                "no input to the deserializer".to_string(),
            ))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.input_u64 {
            visitor.visit_u64(value)
        } else {
            Err(Error::missing_field("input_u64"))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for ElementSymbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SymbolVisitor)
    }
}

impl FromStr for ElementSymbol {
    type Err = SymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut deserializer = ElementSymbolDeserializer::from_str(s);
        ElementSymbol::deserialize(&mut deserializer)
    }
}

impl TryFrom<u64> for ElementSymbol {
    type Error = SymbolError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let mut deserializer = ElementSymbolDeserializer::from_u64(value);
        ElementSymbol::deserialize(&mut deserializer)
    }
}

impl TryFrom<u32> for ElementSymbol {
    type Error = SymbolError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let mut deserializer = ElementSymbolDeserializer::from_u64(value.into());
        ElementSymbol::deserialize(&mut deserializer)
    }
}

impl TryFrom<u16> for ElementSymbol {
    type Error = SymbolError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let mut deserializer = ElementSymbolDeserializer::from_u64(value.into());
        ElementSymbol::deserialize(&mut deserializer)
    }
}

impl TryFrom<u8> for ElementSymbol {
    type Error = SymbolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let mut deserializer = ElementSymbolDeserializer::from_u64(value.into());
        ElementSymbol::deserialize(&mut deserializer)
    }
}
