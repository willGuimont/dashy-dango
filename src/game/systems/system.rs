use crate::Registry;

pub trait System {
    fn execute_system(&mut self, registry: &mut Registry);
}
