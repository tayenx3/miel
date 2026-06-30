pub struct Context<'ctx> {
    pub rodeo: &'ctx lasso::Rodeo,
    pub source_id: usize
}

impl Clone for Context<'_> {
    fn clone(&self) -> Self {
        Self {
            rodeo: self.rodeo,
            source_id: self.source_id,
        }
    }
}