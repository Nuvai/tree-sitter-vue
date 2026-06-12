//! Vue language support for tree-sitter.

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_vue() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for Vue.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_vue) };

/// The content of the [`node-types.json`] file for this grammar.
pub const NODE_TYPES: &str = include_str!("../src/node-types.json");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Vue parser");
    }
}
