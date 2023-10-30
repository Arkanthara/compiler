pub struct Context {
    pub variable: String,
    pub value: f64,
}

impl Context {
    pub fn create_context(var: String, val: f64) -> Context {
        Context {
            variable: var.clone(),
            value: val,
        }
    }
}
