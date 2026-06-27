pub struct Context<'ctx> {
    pub rodeo: &'ctx lasso::Rodeo,
    pub source_id: usize
}

impl<'ctx> Clone for Context<'ctx> {
    fn clone(&self) -> Self {
        Self {
            rodeo: self.rodeo,
            source_id: self.source_id,
        }
    }
}