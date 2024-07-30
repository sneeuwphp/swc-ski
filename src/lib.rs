use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::proxies::TransformPluginProgramMetadata,
};
use transform;

#[plugin_transform]
fn plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.fold_with(visitor(Config {}))
}
