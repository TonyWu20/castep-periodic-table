# castep-periodic-table
A periodic table for use in CASTEP, written in rust.

## Properties
Example `Element`:
```
Element {
    symbol: "C",
    atomic_number: 6_u8,
    lcao: 2_u8,
    mass: 12.0109996796,
    potential: "C_00PBE.usp",
    spin: 0_u8,
}

```

## Usage
```
extern crate castep_periodic_table as cpt;
use cpt::element::Element;
use cpt::data::ELEMENT_TABLE;
// Lookup element "Co" by symbol
let cobalt: &Element = ELEMENT_TABLE.get_by_symbol("Co").unwrap();
// Lookup element by atomic_number
let oxygen = ELEMENT_TABLE.get_by_atomic_number(8_u8).unwrap();
// Get fields
let lcao_cobalt: u8 = cobalt.lcao();
let mass_cobalt: f64 = cobalt.mass();
let spin: u8 = cobalt.spin();
let potential: &str = cobalt.potential();
// Iterator
// ELEMENT_TABLE.iter()...
```

## Maintainance
Currently the crate only include C, H, O, transition metals and Lanthanides due to personal interest. It can be updated by modifying the `element_table.yaml` and generate a new `data.rs` with the provided method in `ElementYamlTable`.
