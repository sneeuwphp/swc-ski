use swc_core::ecma::{
    ast::Module,
    visit::{Visit, VisitWith},
};

use crate::{inspector, Config};

struct Transformator {}

impl Visit for Transformator {
    fn visit_module(&mut self, node: &Module) {
        let mut inspector = inspector::visitor();
        node.visit_with(&mut inspector);

        println!("{:#?}", inspector);

        node
    }
}

pub fn visitor(config: Config) -> Transformator {
    Tranformator {}
}
