use antlr_rust::tree::{ParseTreeVisitorCompat, ParseTree};

use crate::parser::{cpp14parser::{CPP14ParserContextType, TranslationUnitContext}, cpp14parservisitor::CPP14ParserVisitorCompat};

pub struct DebugVisitor {

    // traversal
    pub return_val: Vec<String>,

    // debug
    pub debug_output: bool,
    pub indent: u16,

}

impl Default for DebugVisitor {
    fn default() -> Self {
        Self {

            // traversal
            return_val: Vec::new(),

            // debug
            debug_output: false,
            indent: 0u16,

        }
    }
}

impl<'i> DebugVisitor {

    pub fn ascend_ident(&mut self) {
        if !self.debug_output {
            return;
        }
        self.indent = self.indent - 1;
    }

    pub fn descend_ident(&mut self, label: &str) {
        if !self.debug_output {
            return;
        }
        self.indent = self.indent + 1;
        for _n in 0..self.indent {
            print!("  ");
        }
        println!("{}", label);
    }

    // reset internal state
    pub fn reset_self(&mut self) {
        //self.record.clear();
    }

}

impl<'i> ParseTreeVisitorCompat<'i> for DebugVisitor {

    type Node = CPP14ParserContextType;
    type Return = Vec<String>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.return_val
    }

    fn visit_terminal(
        &mut self,
        node: &antlr_rust::tree::TerminalNode<'i, Self::Node>,
    ) -> Self::Return {
        let terminal_text = node.get_text();
        log::trace!("'{}'", terminal_text);

        if terminal_text.eq(",") {
            return vec![];
        }

        return vec![terminal_text];
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'i, Self::Node>,
    ) -> Self::Return {
        Self::Return::default()
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        // https://stackoverflow.com/questions/40792801/what-is-the-best-way-to-concatenate-vectors-in-rust
        let c: Vec<String> = aggregate
            .iter()
            .cloned()
            .chain(next.iter().cloned())
            .collect(); // Cloned
        c
    }

    #[allow(dead_code, unused)]
    fn should_visit_next_child(
        &self,
        node: &<Self::Node as antlr_rust::parser::ParserNodeType<'i>>::Type,
        current: &Self::Return,
    ) -> bool {
        true
    }

}

impl<'i> CPP14ParserVisitorCompat<'i> for DebugVisitor {

    fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'i>) -> Self::Return {
        self.descend_ident("visit_asm_intrinsic_usage");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        log::info!("cr: {:?}\n", visit_children_result);

        //self.process_asm_intrinsic_usage(visit_children_result)

        visit_children_result
    }

}