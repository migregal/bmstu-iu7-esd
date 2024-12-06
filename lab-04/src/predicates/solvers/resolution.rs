use crate::predicates::values::disjunct;

use super::storage;

pub struct Solver {
    storage: storage::Storage,
}

impl Solver {
    pub fn new(storage: storage::Storage) -> Solver {
        return Solver { storage };
    }

    pub fn solve(&mut self, mut a: disjunct::Disjunct) -> bool {
        let (mut should_continue, mut max_tries) = (true, 100);

        let disjuncts = self.storage.get_disjuncts();
        let mut used_disjuncts: Vec<bool> = Vec::with_capacity(disjuncts.len());
        for _ in 0..disjuncts.len() {
            used_disjuncts.push(false);
        }

        while should_continue && max_tries > 0 {
            for (idx, b) in disjuncts
                .iter()
                .enumerate()
                .filter(|t| !used_disjuncts[t.0])
            {
                let res = disjunct::unify(&mut self.storage, a.clone(), b.clone());

                if res.is_none() {
                    continue;
                }

                let tmp = res.unwrap();
                should_continue = true;
                max_tries -= 1;

                println!("{}", a);
                println!("{}", b);
                println!("\t==> {}\n", tmp);

                a = tmp;
                used_disjuncts[idx] = true;
                break;
            }

            max_tries -= 1;
        }

        false
    }
}
