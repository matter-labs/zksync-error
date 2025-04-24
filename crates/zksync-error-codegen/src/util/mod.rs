pub mod io;
pub mod printing;

pub trait LooseEq<I> {
    fn loosely_match(&self, template: &I) -> bool;
}
