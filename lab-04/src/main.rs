use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use capitalize::Capitalize;

pub mod predicates;

use predicates::solvers::{resolution, storage};
use predicates::values::atom;
use predicates::values::{atom::Atom, disjunct, term};

fn main() {
    let storage = get_knowledge("./input.txt");

    let mut solver = resolution::Solver::new(storage);

    solver.solve(disjunct::Disjunct::new(vec![
        atom::Atom::new(
            "L".to_string(),
            true,
            vec!["Лена".to_string(), "Снег".to_string()],
        ),
        atom::Atom::new(
            "L".to_string(),
            false,
            vec!["Лена".to_string(), "Дождь".to_string()],
        ),
    ]));
}

fn get_knowledge(fin: &str) -> storage::Storage {
    let mut storage = storage::Storage::new();

    if let Ok(lines) = read_lines(fin) {
        for line in lines.flatten().into_iter() {
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
                storage.add_atom(atom.clone());
                atoms.push(atom);
            }

            storage.add_disjunct(disjunct::Disjunct::new(atoms));
        }
    }

    storage
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
