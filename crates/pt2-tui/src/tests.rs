pub struct Lpp<'a> {
    pub c: Vec<f64>,
    pub a: &'a [&'a [f64]],
    pub initial_point: Vec<f64>,
    pub b: &'a [f64],
}

pub fn generate_1<'a>() -> Lpp<'a> {
    Lpp {
        c: vec![9., 10., 16.],
        a: &[&[18., 15., 12.], &[6., 4., 8.], &[5., 3., 3.]],
        initial_point: vec![1., 1., 1., 315., 174., 169.],
        b: &[360., 192., 180.],
    }
}
