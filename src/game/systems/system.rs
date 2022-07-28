use crate::Registry;

pub trait System {
    fn execute_system(&self, registry: &mut Registry) -> ();
}
