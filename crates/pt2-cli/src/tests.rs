use pt2_core::{Constraints, Sign};

pub struct Lpp<'a> {
    pub objective_function: Vec<f64>,
    pub constraints: Constraints<'a>,
    pub initial_point: Vec<f64>,
}

pub fn generators<'a>() -> &'a [fn() -> Lpp<'a>] {
    &[lab_6_problem_1, lab_6_problem_2, lecture_6_problem_1]
}

fn lab_6_problem_1<'a>() -> Lpp<'a> {
    Lpp {
        objective_function: vec![1., 1.],
        constraints: &[(&[2., 4.], Sign::Le, 16.), (&[1., 3.], Sign::Ge, 9.)],
        initial_point: vec![0.5, 3.5, 1., 2.],
    }
}

fn lab_6_problem_2<'a>() -> Lpp<'a> {
    Lpp {
        objective_function: vec![9., 10., 16.],
        constraints: &[
            (&[18., 15., 12.], Sign::Le, 360.),
            (&[6., 4., 8.], Sign::Le, 192.),
            (&[5., 3., 3.], Sign::Le, 180.),
        ],
        initial_point: vec![1., 1., 1., 315., 174., 169.],
    }
}

fn lecture_6_problem_1<'a>() -> Lpp<'a> {
    Lpp {
        objective_function: vec![1., 2., 0.],
        constraints: &[(&[1., 1., 1.], Sign::Eq, 8.)],
        initial_point: vec![2., 2., 4.],
    }
}
