use na::{DMatrix, DVector};

mod error;

pub struct Auxilliary {
    big_d: DMatrix<f64>,
    big_a_tilde: DMatrix<f64>,
    c_tilde: DVector<f64>,
    big_p: DMatrix<f64>,
    c_p: DVector<f64>,
    nu: f64,
    x_tilde: DVector<f64>,
}

pub struct Iteration {
    auxilliary: Auxilliary,
    x: DVector<f64>,
}

pub struct InteriorPoint {
    done: bool,
    x: DVector<f64>,
    big_a: DMatrix<f64>,
    c: DVector<f64>,
    alpha: f64,
    eps: f64,
}

impl Iterator for InteriorPoint {
    type Item = Result<Iteration, error::NotApplicable>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let size = self.x.len();

        let big_d = DMatrix::from_diagonal(&self.x);

        let big_a_tilde = &self.big_a * &big_d;
        let c_tilde = &big_d * &self.c;

        let big_p = {
            let big_i = DMatrix::identity(size, size);
            let big_a_tilde_tr = big_a_tilde.transpose();
            let Some(inverse) = (&big_a_tilde * &big_a_tilde_tr).try_inverse() else {
                return Some(Err(error::NotApplicable));
            };
            big_i - big_a_tilde_tr * inverse * &big_a_tilde
        };
        let c_p = &big_p * &c_tilde;

        let Some(nu) = c_p
            .iter()
            .filter(|it| it < &&0.0)
            .map(|it| it.abs())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
        else {
            return Some(Err(error::NotApplicable));
        };
        let x_tilde = DVector::from_element(size, 1.0) + (self.alpha / nu) * &c_p;

        let new_x = &big_d * &x_tilde;
        if (&new_x - &self.x).norm() < self.eps {
            self.done = true;
        }

        self.x = new_x;

        Some(Ok(Iteration {
            auxilliary: Auxilliary {
                big_d,
                big_a_tilde,
                c_tilde,
                big_p,
                c_p,
                nu,
                x_tilde,
            },
            x: self.x.clone_owned(),
        }))
    }
}

pub fn solve(
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    initial_point: Vec<f64>,
    b: Vec<f64>,
    alpha: f64,
    eps: usize,
) -> Result<InteriorPoint, error::NoSolution> {
    let n = a.len();
    let m = a.first().unwrap().len();
    let new_eps = 0.1_f64.powi(eps as i32 + 1);

    assert_eq!(b.len(), n);
    a.iter().for_each(|row| assert_eq!(row.len(), c.len()));
    assert_eq!(initial_point.len(), n + m);

    let x = DVector::from_vec(initial_point);

    let big_a = {
        let a = a.into_iter().flatten();
        let mut big_a = DMatrix::from_row_iterator(n, m, a).resize_horizontally(n + m, 0.0);
        big_a.view_mut((0, n), (n, n)).fill_with_identity();
        big_a
    };
    let c = DVector::from_vec(c).resize_vertically(n + m, 0.0);

    Ok(InteriorPoint {
        done: false,
        x,
        big_a,
        c,
        alpha,
        eps: new_eps,
    })
}

pub fn run() {
    const EPS: usize = 2;

    let _ = solve(
        vec![9., 10., 16.],
        vec![vec![18., 15., 12.], vec![6., 4., 8.], vec![5., 3., 3.]],
        vec![1., 1., 1., 315., 174., 169.],
        vec![360., 192., 180.],
        0.5,
        EPS,
    );
}
