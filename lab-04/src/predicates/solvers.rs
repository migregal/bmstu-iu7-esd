use super::values::term;

pub mod storage;

pub mod unification;

pub trait TermsStorage {
    fn get_term(&self, name: String) -> term::Term;

    fn link_terms(&mut self, from: String, to: String) -> bool;

    fn get_link_cmd(&self, from: String, to: String) -> Option<Box<dyn LinkTermsCommand>>;
}

pub trait LinkTermsCommand {
    fn run(&self, storage: Box<dyn TermsStorage>);
}
