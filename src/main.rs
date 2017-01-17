extern crate rand;
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;
use rand::distributions::{Normal, IndependentSample};

/* physical constants */
const ABS_ZERO :f64 = -273.15; // absolute zero [degree Celsius]
const K :f64 = 1.380649e-23;   // Boltzmann constant [J/K]

/* parameters (user-defined)*/
const T_DEGC :f64 = 25.;     // temperature [degree Celsius]
const RHO :f64 = 2.699e+3;   // dencity of particle [kg/m^3]
const ETA :f64 = 0.890e-3;   // viscocity of fluid [Pa*s]
const RA :f64 = 100e-6;      // radius of particle [m]
const DT :f64 = 5.747e-4;    // step size in time [s]
const DIM :usize = 3;        // dimension [-]
const PTCLNUM :usize = 1000; // particle amount [-]
const SIM_TIME :f64 = 0.3;   // simulation time [s]
/* parameters (automatically calculated) */
const T :f64 = T_DEGC-ABS_ZERO;             // temperature in Kelvin [K]
const M :f64 = (4./3.)*PI*RA*RA*RA*RHO;     // mass of particle [kg]
const BETA :f64 = 6.*PI*RA*ETA/M;           // constant of drag [-]
const D :f64 = K*T/(M*BETA);                // diffusion coefficient [m^2/s]
const REP_N:usize = (SIM_TIME/DT) as usize; // number of repetition

#[derive(Clone,Copy,Default)]
struct Particle{
    pub r: [f64;DIM],
    pub v: [f64;DIM],
}

impl Particle{
    fn v2(&self) -> f64 {
        let dist = Normal::new(0.,(K*T/M)*(1.-(-2.*BETA*DT).exp()));
        return dist.ind_sample(&mut rand::thread_rng());
    }

    fn adv(&mut self){
        let c1 = 1.-(-BETA*DT).exp();
        for i in 0..DIM {
            self.r[i] += c1*self.v[i]/BETA;
            self.v[i] += -self.v[i]*c1+self.v2();
        }
    }

    fn v_norm(&self) -> f64 {
        let mut sum = 0.;
        for val in &self.v {
            sum += val*val;
        }
        sum.sqrt()
    }
}


fn main(){
    let mut ps:[Particle;PTCLNUM] = [Default::default();PTCLNUM];

    let mut f = File::create("coord.dat").unwrap();

    for _ in 0..REP_N {
        for p in ps.iter_mut() {
            p.adv();
            write!(&mut f, "{:e} {:e} {:e}\n",p.r[0],p.r[1],p.r[2]);
        }
        write!(&mut f, "\n\n");
    }
}

