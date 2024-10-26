use na::{DMatrix, DVector};

mod error;

pub struct Solution {
    // /// A vector of the decision variables.
    // x: Vec<f64>,
    // /// The maximum value of the objective function.
    // z: f64,
    x: DVector<f64>,
    objective_fn: DMatrix<f64>,
}

pub fn run() {
    const EPS: usize = 2;
    let solution = solve(
        vec![2., 2., 4., 3.],
        vec![vec![2., -2., 8., 0.], vec![-6., -1., 0., -1.]],
        vec![-2., 3., 0., 0.],
        0.5,
        EPS + 1,
    )
    .unwrap();

    for (i, iteration) in solution.enumerate().map(|(i, it)| (i + 1, it)) {
        let it = iteration.unwrap();
        println!("Iteration {i}");
        println!("x:{:.EPS$}", it.x);
        println!("fn:{:.EPS$}", it.objective_fn);
    }
}

pub struct Ipa {
    x: DVector<f64>,
    a: DMatrix<f64>,
    c: DVector<f64>,
    alpha: f64,
    eps: usize,
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

        if norm < 0.1_f64.powi(self.eps as i32) {
            self.done = true;
        }

        Some(Ok(Solution {
            x: self.x.clone_owned(),
            objective_fn: &self.c * self.x.transpose(),
        }))
    }
}

pub fn solve(
    x: Vec<f64>,
    a: Vec<Vec<f64>>,
    c: Vec<f64>,
    alpha: f64,
    eps: usize,
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

// pub fn main(c: Vec<f64>, a: Vec<Vec<f64>>, initial_point: Vec<f64>, b: Vec<f64>, eps: usize) {
//     assert_eq!(a.len(), b.len());
//     a.iter().for_each(|row| assert_eq!(row.len(), c.len()));
//     todo!();
// }
