extern crate rand;
use std::fs::File;
use std::io::Write;
use rand::distributions::{Normal, IndependentSample};

/* physical constants */
const abs_zero :f64 = -273.15; // absolute zero [degree Celsius]
const k :f64 = 1.380649e-23;   // Boltzmann constant [J/K]
const pi :f64 = 3.141592653589793238462643383279;

/* parameters (user-defined)*/
const t_degc :f64 = 25.;       // temperature [degree Celsius]
const rho :f64 = 2.699e+3;     // dencity of particle [kg/m^3]
const eta :f64 = 0.890e-3;     // viscocity of fluid [Pa*s]
const ra :f64 = 100e-6;        // radius of particle [m]
const dt :f64 = 5.747e-4;      // step size in time [s]
const dim :usize = 3;            // dimension [-]
const ptclnum :usize = 1000;     // particle amount [-]
const process_time :f64 = 0.3; // simulation time [s]

/* parameters (automatically calculated) */
const T :f64 = t_degc-abs_zero;         // temperature in Kelvin [K]
const m :f64 = (4./3.)*pi*ra*ra*ra*rho; // mass of particle [kg]
const beta :f64 = 6.*pi*ra*eta/m;       // constant of drag [-]
const D :f64 = k*T/(m*beta);            // diffusion coefficient [m^2/s]

#[derive(Clone,Copy,Default)]
struct particle{
  pub r: [f64;dim],
  pub v: [f64;dim],
}

impl particle{
  fn v2(&self) -> f64 {
    let dist = Normal::new(0.,(k*T/m)*(1.-(-2.*beta*dt).exp()));
    return dist.ind_sample(&mut rand::thread_rng());
  }

  fn adv(&mut self){
    let c1 = 1.-(-beta*dt).exp();
    for i in 0..dim {
      self.r[i] += c1*self.v[i]/beta;
      self.v[i] += -self.v[i]*c1+self.v2();
    }
  }
}


fn main(){
  let mut ps:[particle;ptclnum] = [Default::default();ptclnum];
  let mut f = File::create("coord.dat").unwrap();
  for _ in 0..100{
    for p in ps.iter_mut(){
      p.adv();
      write!(&mut f, "{:e} {:e} {:e}\n",p.r[0],p.r[1],p.r[2]).unwrap();
    }
    write!(&mut f, "\n\n");
  }
}
