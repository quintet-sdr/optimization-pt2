use na::{ComplexField, DMatrix, DVector};

mod error;

pub struct Solution {
    // /// A vector of the decision variables.
    // x: Vec<f64>,
    // /// The maximum value of the objective function.
    // z: f64,
    x: DVector<f64>,
    objective_fn: DMatrix<f64>,
}

pub struct Ipa {
    x: DVector<f64>,
    a: DMatrix<f64>,
    c: DVector<f64>,
    alpha: f64,
    eps: f64,
    done: bool,
}

impl Iterator for Ipa {
    type Item = Result<Solution, error::NotApplicable>;

    #[allow(clippy::many_single_char_names)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let d = DMatrix::from_diagonal(&self.x);

        let aa = &self.a * &d;
        let cc = &d * &self.c;

        let i = DMatrix::<f64>::identity(self.c.nrows(), self.c.nrows());

        let f = &aa * aa.transpose();
        let Some(fi) = f.try_inverse() else {
            return Some(Err(error::NotApplicable));
        };
        let h = aa.tr_mul(&fi);

        let p = i - (h * aa);

        let cp = p * cc;

        let nu = cp.min().abs();
        let y = DVector::from_element(self.c.nrows(), 1.0) + (self.alpha / nu) * cp;

        let yy = d * y;
        let norm = (&yy - &self.x).norm();
        self.x = yy;

        if norm < self.eps {
            self.done = true;
        }

        Some(Ok(Solution {
            x: self.x.clone_owned(),
            objective_fn: &self.c * self.x.transpose(),
        }))
    }
}

pub fn solve_old(
    x: Vec<f64>,
    a: Vec<Vec<f64>>,
    c: Vec<f64>,
    alpha: f64,
    eps: f64,
) -> Result<Ipa, error::NoSolution> {
    // if !todo!() {
    //     return Err(NoSolutionError);
    // }

    Ok(Ipa {
        x: DVector::from_vec(x),
        a: DMatrix::from_row_iterator(a.len(), a.first().unwrap().len(), a.into_iter().flatten()),
        c: DVector::from_vec(c),
        alpha,
        eps,
        done: false,
    })
}

pub fn run() {
    const EPS: usize = 2;

    let new = solve_new(
        vec![9., 10., 16.],
        vec![vec![18., 15., 12.], vec![6., 4., 8.], vec![5., 3., 3.]],
        vec![1., 1., 1., 315., 174., 169.],
        vec![360., 192., 180.],
        0.5,
        EPS,
    );

    // let solution = solve_old(
    //     vec![2., 2., 4., 3.],
    //     vec![vec![2., -2., 8., 0.], vec![-6., -1., 0., -1.]],
    //     vec![-2., 3., 0., 0.],
    //     0.5,
    //     0.01_f64.powi(EPS as i32 + 1),
    // )
    // .unwrap();

    // for (i, iteration) in solution.enumerate().map(|(i, it)| (i + 1, it)) {
    //     let it = iteration.unwrap();
    //     println!("Iteration {i}");
    //     println!("x:{:.EPS$}", it.x);
    //     println!("fn:{:.EPS$}", it.objective_fn);
    // }
}

pub fn solve_new(
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    initial_point: Vec<f64>,
    b: Vec<f64>,
    alpha: f64,
    eps: usize,
) {
    let n = a.len();
    let m = a.first().unwrap().len();

    assert_eq!(b.len(), n);
    a.iter().for_each(|row| assert_eq!(row.len(), c.len()));
    assert_eq!(initial_point.len(), n + m);

    let d = DMatrix::from_diagonal(&DVector::from_vec(initial_point));
    let a = {
        let mut a = DMatrix::from_row_iterator(n, m, a.into_iter().flatten())
            .resize_horizontally(n + m, 0.0);

        a.view_mut((0, n), (n, n)).fill_with_identity();

        a
    };
    let c = DVector::from_vec(c).resize_vertically(n + m, 0.0);

    let a_tilde = &a * &d;
    let c_tilde = &d * c;

    let i = DMatrix::identity(n + m, n + m);
    let p = i - a_tilde.tr_mul(&(&a_tilde * a_tilde.transpose()).try_inverse().unwrap()) * a_tilde;
    let c_p = p * c_tilde;

    let nu = c_p
        .into_iter()
        .filter(|it| it < &&0.0)
        .max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())
        .unwrap();
    let x_tilde = DVector::from_element(n + m, 1.0) + (alpha / nu) * c_p;

    let x = d * x_tilde;

    println!("{a}");
}
