pub struct Context<'ctx> {
    pub rodeo: &'ctx lasso::Rodeo,
    pub source_id: usize
}

pub struct ContextMut<'ctx> {
    pub rodeo: &'ctx mut lasso::Rodeo,
    pub source_id: usize
}

impl<'ctx> ContextMut<'ctx> {
    pub fn as_ctx(&'ctx self) -> Context<'ctx> {
        Context {
            rodeo: &*self.rodeo,
            source_id: self.source_id
        }
    }
}