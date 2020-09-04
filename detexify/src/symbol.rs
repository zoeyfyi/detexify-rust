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

    pub fn id(&self) -> &'static str {
        let id = format!(
            "{}-{}-{}",
            self.package,
            self.font_encoding,
            self.command.replace("\\", "_")
        );

        // TODO: remove this once https://github.com/sfackler/rust-phf/pull/185 is merged
        Box::leak(base64::encode(id).into_boxed_str())
    }
}

pub fn iter_symbols() -> impl Iterator<Item = Symbol> {
    SYMBOL_TABLE.values().cloned()
}

#[cfg(test)]
mod tests {

    use super::Symbol;
    use crate::iter_symbols;

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

    #[test]
    fn test_iterate_symbols() {
        assert_eq!(iter_symbols().count(), 1098);
    }

    #[test]
    fn test_id_get_id() {
        for symbol in iter_symbols() {
            assert_eq!(Symbol::from_id(symbol.id()).unwrap(), symbol);
        }
    }

    #[test]
    fn missing_ids() {
        // these where missing due to a bug in the build script
        // if a entry in symbols.yaml had a bothmoth and textmode,
        // one of the sets of symbols would not be processed
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX2xpZ2h0bmluZw==").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX0xFRlRhcnJvdw==").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX1VQYXJyb3c=").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX29wcG9zaXRpb24=").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX2Nvbmp1bmN0aW9u").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX3BvaW50ZXI=").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX0RPV05hcnJvdw==").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX3JlY29yZGVy").is_some());
        assert!(Symbol::from_id("d2FzeXN5bS1PVDEtX3Bob25l").is_some());
    }
}
