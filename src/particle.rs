extern crate rand;
use rand::distributions::{Normal, IndependentSample};
use parameters::*;

#[derive(Clone,Copy,Default)]
pub struct Particle{
    pub r: [f64;DIM],
    pub v: [f64;DIM],
}

impl Particle{
    fn v2(&self) -> f64 {
        let dist = Normal::new(0.,(K*T/M)*(1.-(-2.*BETA*DT).exp()));
        return dist.ind_sample(&mut rand::thread_rng());
    }

    pub fn adv(&mut self){
        let c1 = 1.-(-BETA*DT).exp();
        for i in 0..DIM {
            self.r[i] += c1*self.v[i]/BETA;
            self.v[i] += -self.v[i]*c1+self.v2();
        }
    }

    pub fn v_norm(&self) -> f64 {
        let mut sum = 0.;
        for val in &self.v {
            sum += val*val;
        }
        sum.sqrt()
    }
}
