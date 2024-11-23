pub mod predicates;

use predicates::values::{atom::Atom, term::Term};

fn main() {
    let x1 = Term::new_var("x1");

    let (_atom1, _atom2) = (Atom::new("S", vec![x1]), Atom::new("M", vec![x1]));

    let x2 = Term::new_var("x2");
    let _atom3 = Atom::new_negative("M", vec![x2]);

    let rain = Term::new_const("RAIN");
    let _atom4 = Atom::new_negative("L", vec![x2, rain]);

    let x3 = Term::new_var("x3");
    let _atom5 = Atom::new_negative("S", vec![x3]);

    let snow = Term::new_const("SNOW");
    let _atom6 = Atom::new_negative("L", vec![x3, snow]);

    let (y1, y2) = (Term::new_var("y1"), Term::new_var("y2"));

    let (lena, petya) = (Term::new_const("LENA"), Term::new_const("PETYA"));

    let _atom7 = Atom::new_negative("L", vec![lena, y1]);
    let _atom8 = Atom::new_negative("L", vec![petya, y1]);
    let _atom9 = Atom::new("L", vec![lena, y2]);
    let _atom10 = Atom::new("L", vec![petya, y2]);
    let _atom11 = Atom::new("L", vec![petya, rain]);
    let _atom12 = Atom::new("L", vec![petya, snow]);
}
