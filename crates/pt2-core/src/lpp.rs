use na::{DMatrix, DVector};

use crate::{interfaces::NotApplicableError, Constraints, Sign};

#[derive(Clone)]
pub struct Lpp {
    pub x: DVector<f64>,
    pub big_a: DMatrix<f64>,
    pub c: DVector<f64>,
    pub eps: f64,
}

impl Lpp {
    pub fn try_new(
        objective_function: Vec<f64>,
        constraints: &Constraints,
        initial_point: Vec<f64>,
        eps: usize,
    ) -> Result<Self, NotApplicableError> {
        let (n, m) = get_n_and_m(constraints).ok_or(NotApplicableError)?;

        if constraints
            .iter()
            .any(|row| row.0.len() != objective_function.len())
        {
            return Err(NotApplicableError);
        }

        let initial_point_is_feasible = constraints.iter().all(|(coefficients, sign, rhs)| {
            let constraint_sum: f64 = coefficients
                .iter()
                .zip(&initial_point)
                .map(|(coeff, x)| coeff * x)
                .sum();

            sign.compare(&constraint_sum, rhs)
        });

        if !initial_point_is_feasible {
            return Err(NotApplicableError);
        }

        let no_slack_rows = constraints
            .iter()
            .enumerate()
            .filter_map(|(i, (_, sign, _))| matches!(sign, Sign::Eq).then_some(i));
        let no_slack_cols = no_slack_rows.map(|j| m + j).collect::<Box<[_]>>();
        let slack_cols_count = n - no_slack_cols.len();

        if initial_point.len() != m + slack_cols_count {
            return Err(NotApplicableError);
        }

        let x = DVector::from_vec(initial_point);
        let big_a = {
            let left_part_row_elements = constraints
                .iter()
                .flat_map(|(coefficients, _, _)| coefficients)
                .copied();

            let right_part_diagonal_elements = &DVector::from_vec(
                constraints
                    .iter()
                    .map(|(_, sign, _)| match sign {
                        Sign::Le => 1.,
                        Sign::Eq => 0.,
                        Sign::Ge => -1.,
                    })
                    .collect(),
            );

            let mut big_a = DMatrix::from_row_iterator(n, m, left_part_row_elements)
                .resize_horizontally(m + n, 0.);

            big_a
                .view_mut((0, m), (n, n))
                .set_diagonal(right_part_diagonal_elements);

            big_a.remove_columns_at(&no_slack_cols)
        };
        let c = DVector::from_vec(objective_function).resize_vertically(m + slack_cols_count, 0.);
        let eps = up_to_n_dec_places(i32::try_from(eps).map_err(|_| NotApplicableError)?);

        Ok(Self { x, big_a, c, eps })
    }
}

fn get_n_and_m(constraints: &Constraints) -> Option<(usize, usize)> {
    Some((constraints.len(), constraints.first()?.0.len()))
}

fn up_to_n_dec_places(n: i32) -> f64 {
    0.1_f64.powi(n) / 2.
}
