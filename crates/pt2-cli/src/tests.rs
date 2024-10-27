pub struct Lpp<'a> {
    pub objective_function: Vec<f64>,
    pub constraints: &'a [&'a [f64]],
    pub initial_point: Vec<f64>,
    pub rhs_numbers: &'a [f64],
}

pub fn all<'a>() -> &'a [fn() -> Lpp<'a>] {
    &[lab_6_problem_2]
}

fn lab_6_problem_2<'a>() -> Lpp<'a> {
    Lpp {
        objective_function: vec![9., 10., 16.],
        constraints: &[&[18., 15., 12.], &[6., 4., 8.], &[5., 3., 3.]],
        initial_point: vec![1., 1., 1., 315., 174., 169.],
        rhs_numbers: &[360., 192., 180.],
    }
}
