use ordered_float::OrderedFloat;
// use sorted_list::SortedList;

pub struct LinearInterpolator {
    x: Vec<f64>,
    y: Vec<f64>,
    xmax: f64,
    xmin: f64,
}

impl LinearInterpolator {
    pub fn new(x: &Vec<f64>, y: &Vec<f64>) -> LinearInterpolator {
        assert!(x.len() == y.len());

        let xmax = *x.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
        let xmin = *x.iter().min_by_key(|n| OrderedFloat(n.abs())).unwrap();

        // X needs to be sorted, so we zip X & Y and sort the tuples
        let mut both: Vec<(&f64, &f64)> = x.iter().zip(y.iter()).collect();
        both.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        let x_ = both.iter().map(|x| *x.0).collect();
        let y_ = both.iter().map(|x| *x.1).collect();

        LinearInterpolator {
            x: x_,
            y: y_,
            xmax,
            xmin,
        }
    }

    pub fn interpolate(&self, a: f64) -> f64 {
        assert!(self.xmin <= a && a <= self.xmax);

        let mut idx: usize = 0;
        for i in 0..self.x.len() - 1 {
            if self.x[i] <= a && a <= self.x[i + 1] {
                idx = i;
                break;
            }
        }

        self.y[idx]
            + (self.y[idx + 1] - self.y[idx]) / (self.x[idx + 1] - self.x[idx]) * (a - self.x[idx])
    }
}

// pub fn linear_interpolation(X: &Vec<f64>, Y: &Vec<f64>, a: f64) -> f64 {
//     assert!(X.len() == Y.len());
//     let xmax = *X.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
//     let xmin = *X.iter().min_by_key(|n| OrderedFloat(n.abs())).unwrap();
//     assert!(xmin <= a && a <= xmax);
//     let result: f64 = 0.;
//     let mut idx: usize = 0;
//     for i in 0..X.len() - 1 {
//         if X[i] <= a && a <= X[i + 1] {
//             idx = i;
//             break;
//         }
//     }
//     Y[idx] + (Y[idx + 1] - Y[idx]) / (X[idx + 1] - X[idx]) * (a - X[idx])
// }
