use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use capitalize::Capitalize;

pub mod predicates;

use predicates::solvers::{resolution, storage};
use predicates::values::atom;
use predicates::values::disjunct::Disjunct;
use predicates::values::{atom::Atom, disjunct, term};

fn main() {
    let mut storage = get_knowledge("./input.txt");

    let disj = parse_disjunct(
        &mut storage,
        "~L(Лена, Снег) | L(Лена, Дождь)".to_string(),
        false,
    )
    .unwrap();

    let mut solver = resolution::Solver::new(storage);

    solver.solve(disj);
}

fn get_knowledge(fin: &str) -> storage::Storage {
    let mut storage = storage::Storage::new();

    if let Ok(lines) = read_lines(fin) {
        for line in lines.flatten().into_iter() {
            let disj = parse_disjunct(&mut storage, line, true);
            if disj.is_some() {
                storage.add_disjunct(disj.unwrap());
            }
        }
    }

    storage
}

fn parse_disjunct(
    storage: &mut storage::Storage,
    line: String,
    add_atoms_to_store: bool,
) -> Option<Disjunct> {
    let exprs = line
        .split("|")
        .map(|s| s.to_string().trim().to_string())
        .into_iter();

    let mut atoms = Vec::<atom::Atom>::new();

    for mut expr in exprs {
        let is_neg = expr.starts_with("~");
        if is_neg {
            expr.remove(0);
        }

        let parts = expr.split(&['(', ')'][..]).collect::<Vec<&str>>();
        let name = parts[0].to_string();

        let mut terms = Vec::new();
        if parts[1].len() > 0 {
            terms.extend(
                parts[1]
                    .split(",")
                    .map(|s| s.to_string().trim().to_string())
                    .collect::<Vec<String>>(),
            );
        }

        for term in terms.clone().into_iter() {
            if term.capitalize() == term {
                storage.add_term(term::Term::new_const(term));
            } else {
                storage.add_term(term::Term::new_var(term));
            }
        }

        let atom = Atom::new(name, is_neg, terms);
        if add_atoms_to_store {
            storage.add_atom(atom.clone());
        }
        atoms.push(atom);
    }

    Some(disjunct::Disjunct::new(atoms))
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
