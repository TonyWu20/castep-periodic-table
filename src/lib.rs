pub mod data;
pub mod element;

#[cfg(test)]
mod test {
    use crate::{
        data::ELEMENT_TABLE,
        element::{Conventions, Element, ElementSymbol, ElementYamlTable, LookupElement},
    };

    use std::{fs::File, io::Write, path::PathBuf, str::FromStr};
    #[test]
    fn read_from_table() {
        let cwd = env!("CARGO_MANIFEST_DIR");
        let file =
            File::open(PathBuf::from_str(&format!("{cwd}/element_table.yaml")).unwrap()).unwrap();
        let element_table: Result<ElementYamlTable, serde_yaml::Error> =
            serde_yaml::from_reader(file);
        match element_table {
            Ok(table) => println!("{:?}", table),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn write_data_rs() {
        let cwd = env!("CARGO_MANIFEST_DIR");
        let file =
            File::open(PathBuf::from_str(&format!("{cwd}/element_table.yaml")).unwrap()).unwrap();
        let element_table: ElementYamlTable = serde_yaml::from_reader(file).unwrap();
        let export_content = element_table.export_struct();
        let mut output = File::create("./data_backup.rs").unwrap();
        output.write_all(b"use crate::element::Element;\n").unwrap();
        output
            .write_all(b"use crate::element::ElementSymbol;\n")
            .unwrap();
        output.write_all(export_content.as_bytes()).unwrap();
    }
    #[test]
    fn test_table() {
        let mut element_iter = ELEMENT_TABLE.iter();
        let element_co: &Element = element_iter.find(|elm| elm.symbol == "Co").unwrap();
        println!("Co: {:?}", element_co);
        let element_h = ELEMENT_TABLE
            .iter()
            .find(|elm| elm.atomic_number == 0_u8)
            .unwrap();
        println!("H: {:?}", element_h);
        let element_h = ELEMENT_TABLE.get_by_atomic_number(0_u8).unwrap();
        println!("H: {:?}", element_h);
        println!("He: {:?}", ElementSymbol::He as u8);
        let element_n = ELEMENT_TABLE.get_by_symbol(ElementSymbol::from_str("N").unwrap());
        println!("Mass: {:?}", element_n.mass());
        println!("Mass: {:?}", element_co.mass());
        println!("Covalent_radius : {:?}", element_co.covalent_radius());
        let d4 = ELEMENT_TABLE.metals_4d();
        d4.iter().for_each(|elm| println!("{}", elm.symbol()));
        let elm_sc = ElementSymbol::Sc;
        println!("{:?}", elm_sc.family());
    }
}
