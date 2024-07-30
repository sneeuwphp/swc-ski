use swc_core::ecma::visit::Visit;

#[derive(Debug, Default)]
struct Inspector {
    template: String,
}

impl Visit for Inspector {
    //
}

pub fn visitor() -> Inspector {
    Inspector::default()
}
