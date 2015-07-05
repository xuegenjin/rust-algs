extern crate rand;

//use self::rand::Rng;

use super::percolation::Percolation;

pub struct PercolationStats {
    n : usize,
    t : usize,
    measurements: Vec<f64>,

    mean: f64,
    stddev: f64,
    confidence_lo: f64,
    confidence_hi: f64
}

impl PercolationStats {
    pub fn new(n:usize, t: usize) -> PercolationStats {
        assert!(n > 1 && t > 1);

        let mut ps = PercolationStats {n: n,
                          t: t,
                          measurements:vec![0.0; t],
                          mean: 0.0,
                          stddev: 0.0,
                          confidence_lo: 0.0,
                          confidence_hi: 0.0};

       ps.performe_runs();
       ps.calc_stats();
       ps
    }

    fn performe_runs(&mut self) {
        let size = (self.n * self.n) as f64;
        for i in 0..self.t {
            self.measurements[i] = self.run() / size;
        }
    }

    fn run(&self) -> f64 {
        let mut count = 0.0;
        let mut p = Percolation::new(self.n);
        let mut sites = (0..(self.n * self.n)).collect::<Vec<_>>();

        let mut available_size = sites.len();

        //let mut rng = rand::thread_rng();
        //et mut rng = rand::thread_rng();
        while !p.percolates() {

            let index = rand::random::<usize>() % available_size; //rng.get_range(0, available_size);

            //get site
            let site = sites[index];

            let i = site / self.n;
            let j = site % self.n;

            //open site
            p.open(i + 1, j + 1);

            //move the opened site to end
            sites.swap(index, available_size - 1);

            available_size -= 1;
            count += 1.0;
        }

        count
    }

    fn calc_stats(&mut self) {
        let t = self.t as f64;
        let t1 = (self.t - 1) as f64;

        let sum: f64 = self.measurements.iter().fold(0.0, |acc, x| acc + x);
        let mean = sum /(self.t as f64);

        let delta: f64 = self.measurements.iter()
                            .fold(0.0, |acc, x| acc + (x - mean) * (x - mean));

        let stddev = (delta / t1).sqrt();

        self.mean = mean;
        self.stddev = stddev;

        let sqrt_t = t.sqrt();

        self.confidence_lo = mean - 1.96 * stddev / sqrt_t;
        self.confidence_hi = mean + 1.96 * stddev / sqrt_t
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn stddev(&self) -> f64 {
        self.stddev
    }

    pub fn confidence_lo(&self) -> f64 {
        self.confidence_lo
    }

    pub fn confidence_hi(&self) -> f64 {
        self.confidence_hi
    }
}
