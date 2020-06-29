use csv;
use std::fs::File;

use is_sorted::IsSorted;
use itertools_num::linspace;
use ordered_float::OrderedFloat;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::iter::{self, FromIterator};

use crate::helper::*;

pub struct DemandCurve {
    p: Vec<u64>,
    q: Vec<u64>,
    finv_vec: Vec<u64>,
    rng: ThreadRng,
}

impl DemandCurve {
    pub fn new(p: Vec<u64>, q: Vec<u64>, interp_resolution: u64) -> DemandCurve {
        if !IsSorted::is_sorted(&mut p.iter()) {
            panic!("Input price vector must be sorted in increasing order");
        }

        if q.last().unwrap().clone() != 0 {
            panic!("Input quantity vector must have 0 as the last element");
        }

        let p_int = Vec::from_iter(linspace(
            p.iter().min().unwrap().clone() as f64,
            p.iter().max().unwrap().clone() as f64,
            interp_resolution as usize,
        ));

        // let P_int = Vec::from_iter(P_int);

        let p_f64 = p.iter().map(|&x| x as f64).collect();
        let q_f64 = q.iter().map(|&x| x as f64).collect();

        let q_val_interp = LinearInterpolator::new(&p_f64, &q_f64);

        let mut q_val: Vec<f64> = p_int.iter().map(|x| q_val_interp.interpolate(*x)).collect();

        let q_val_max = *q_val.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
        q_val = q_val.iter().map(|x| x / q_val_max).collect();

        let x = linspace(0., 1., interp_resolution as usize);

        let finv_interp = LinearInterpolator::new(&q_val, &p_int);

        let finv_vec_f64: Vec<f64> = x.map(|x| finv_interp.interpolate(x)).collect();

        let finv_vec = finv_vec_f64.iter().map(|&x| x as u64).collect();
        // println!("{:?}", Q_val);
        // println!("{:?}", P_int);
        // println!("{:?}", Finv_vec);

        // let mut rng = &mut rand::thread_rng();

        DemandCurve {
            p: p,
            q: q,
            finv_vec,
            rng: rand::thread_rng(),
        }
    }

    pub fn from_csv(path: &str, interp_resolution: u64) -> DemandCurve {
        let file = File::open(path).expect("Couldn't open input CSV file");
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut p: Vec<u64> = Vec::new();
        let mut q: Vec<u64> = Vec::new();

        for record in reader.records() {
            let record = record.unwrap();
            p.push(record[0].parse().unwrap());
            q.push(record[1].parse().unwrap());
        }

        DemandCurve::new(p, q, interp_resolution)
    }

    pub fn sample_price(&mut self, size: usize) -> Vec<u64> {
        // self.Finv_vec
        //     .choose_multiple(&mut self.rng, size)
        //     .cloned()
        //     .collect()
        iter::repeat_with(|| *(self.finv_vec.choose(&mut self.rng).unwrap()))
            .take(size)
            .collect()
    }
}
