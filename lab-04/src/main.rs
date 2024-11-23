pub mod predicates;

use predicates::solvers::{storage, unification};
use predicates::values::{atom::Atom, term::Term};

fn main() {
    let mut storage = storage::Storage::new();

    storage.add_term(Term::new_var("x1"));

    storage.add_atom(Atom::new("S", vec!["x1"]));
    storage.add_atom(Atom::new("M", vec!["x1"]));

    storage.add_term(Term::new_var("x2"));
    storage.add_atom(Atom::new_negative("M", vec!["x2"]));

    storage.add_term(Term::new_const("RAIN"));

    storage.add_atom(Atom::new_negative("L", vec!["x2", "RAIN"]));

    storage.add_term(Term::new_var("x3"));

    storage.add_atom(Atom::new_negative("S", vec!["x3"]));

    storage.add_term(Term::new_const("SNOW"));

    storage.add_atom(Atom::new_negative("L", vec!["x3", "SNOW"]));

    storage.add_term(Term::new_var("y1"));
    storage.add_term(Term::new_var("y2"));

    storage.add_term(Term::new_const("LENA"));
    storage.add_term(Term::new_const("PETYA"));

    storage.add_atom(Atom::new_negative("L", vec!["LENA", "y1"]));
    storage.add_atom(Atom::new_negative("L", vec!["PETYA", "y1"]));

    storage.add_atom(Atom::new("L", vec!["LENA", "y2"]));
    storage.add_atom(Atom::new("L", vec!["PETYA", "y2"]));
    storage.add_atom(Atom::new("L", vec!["PETYA", "RAIN"]));
    storage.add_atom(Atom::new("L", vec!["PETYA", "SNOW"]));

    let mut solver = unification::Solver::new(storage);

    let (a, b) = (
        Atom::new("L", vec!["x1", "x2"]),
        Atom::new("L", vec!["PETYA", "y1"]),
    );

    let res = solver.solve(a.clone(), b.clone());
}
