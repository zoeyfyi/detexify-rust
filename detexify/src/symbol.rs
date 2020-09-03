extern crate phf;

include!(concat!(env!("OUT_DIR"), "/symbol_table.rs"));

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub command: &'static str,
    pub package: &'static str,
    pub font_encoding: &'static str,
    pub text_mode: bool,
    pub math_mode: bool,
}

impl Symbol {
    pub fn from_id(id: &str) -> Option<Self> {
        SYMBOL_TABLE.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {

    use super::Symbol;

    #[test]
    fn test_from_id() {
        let symbol = Symbol::from_id("bGF0ZXgyZS1PVDEtX3RleHRhc2NpaWNpcmN1bQ==");

        assert_eq!(
            symbol,
            Some(Symbol {
                command: "\\textasciicircum",
                package: "latex2e",
                font_encoding: "OT1",
                text_mode: true,
                math_mode: false
            })
        );
    }
}
