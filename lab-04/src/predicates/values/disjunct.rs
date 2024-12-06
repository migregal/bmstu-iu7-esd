use std::fmt;

use crate::predicates::solvers;

use crate::predicates::values::atom;

#[derive(Clone, Debug)]
pub struct Disjunct {
    atoms: Vec<atom::Atom>,
}

impl Disjunct {
    pub fn new(atoms: Vec<atom::Atom>) -> Disjunct {
        return Disjunct { atoms };
    }

    pub fn atoms(&self) -> Vec<atom::Atom> {
        self.atoms.clone()
    }
}

pub fn unify(
    storage: &mut dyn solvers::TermsStorage,
    this: Disjunct,
    other: Disjunct,
) -> Option<Disjunct> {
    let mut result: Vec<Once> = this
        .atoms()
        .into_iter()
        .map(|a| Once { v: a, flag: false })
        .collect();
    result.extend(
        other
            .atoms()
            .into_iter()
            .map(|a| Once { v: a, flag: false })
            .collect::<Vec<Once>>(),
    );

    let (mut unified, mut local_unifications) = (false, 1);
    while local_unifications > 0 {
        local_unifications = 0;

        for i in 0..result.len() {
            if result[i].flag {
                continue;
            }

            for j in (i + 1)..result.len() {
                if result[j].flag {
                    continue;
                }

                // println!("new iter: {}[{}], {}[{}]", result[i].v, i, result[j].v, j);
                // println!("\tatoms [{}, {}]: {} {}", i, j, result[i].v, result[j].v);

                if result[i].v.name() != result[j].v.name() {
                    // println!("\t\tskip");
                    continue;
                }

                if result[i].v.is_neg() == result[j].v.is_neg() {
                    // println!("\t\tsame sign");
                    if atom::unify(storage, result[i].v.clone(), result[j].v.clone()) {
                        // println!("\t\t\tdeleting {}", result[j].v);
                        result[j].flag = true;
                        local_unifications += 1;
                    }

                    continue;
                }

                if !atom::unify(storage, result[i].v.clone(), result[j].v.clone()) {
                    // println!("\t\tnot unifiable");
                    continue;
                }

                // println!("\t\tdeleting\n\t\t{}\n\t\t{}", result[i].v, result[j].v);
                result[j].flag = true;
                result[i].flag = true;
                local_unifications += 1;
            }
        }

        if local_unifications > 0 {
            unified = true;
        }
    }

    if unified {
        return Some(Disjunct::new(
            result
                .iter()
                .filter(|o| !o.flag)
                .map(|a| a.clone().v)
                .collect(),
        ));
    }

    return None;
}

#[derive(Clone)]
struct Once {
    v: atom::Atom,
    flag: bool,
}

impl fmt::Display for Disjunct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", AtomVec(self.atoms()))
    }
}

struct AtomVec(Vec<atom::Atom>);

impl fmt::Display for AtomVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, v) in (&self.0).into_iter().enumerate() {
            if i > 0 {
                write!(f, " | ")?;
            }

            write!(f, "{}", v)?;
        }
        Ok(())
    }
}
