extern crate rand;
use std::fs::File;
use std::io::{stdout,Write};
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

struct ProgressBar{
    max :u64,
    width :u64,
    precision :i32,
    count :u64,
    next_show_count :u64
}

impl ProgressBar{
    fn new(max_in:u64, width_in:u64) -> ProgressBar {
        ProgressBar{max:max_in, width:width_in, precision:0, count:0, next_show_count:0}
    }

    fn adv(&mut self){
        self.adv_by(1);
    }

    fn adv_by(&mut self,step:u64){
        self.count += step;
        if self.count >= self.next_show_count {self.show();}
    }

    fn set_presicion(&mut self,precision_arg:i32){
        self.precision=precision_arg;
    }

    fn show(&mut self){
        let mut s = "".to_string();
        let count = self.count as f64;
        let max   = self.max   as f64;
        let width = self.width as f64;

        /* draw progress bar */
        let tics = ((count/max)*width) as u64;
        s += "\n[";
        if tics != 0 {
            let mut i = tics-1;
            while i > 0 {
                s += "=";
                i -= 1;
            }
            s += ">";
        }
        if tics < self.width {
            let mut i = self.width-tics;
            while i > 0 {
                s += " ";
                i -= 1;
            }
        }
        s += "] [";

        /* draw progress in percentage */
        // `precision` > 0 : increased digits after decimal point
        // `precision` <= 0: print just before decimal point
        // if `precision` is negative, screen updating will happen less
        let next_tic_count = (((tics+1) as f64)/width*max) as u64;
        let next_ratio_count = if self.precision > 0 {self.count + (max/10_f64.powi(2+self.precision)) as u64} else { self.count + self.max/100 };
        let pcent_width = if self.precision > 0 {4+self.precision} else {3} as usize;
        let pcent = count*(100./max);

        s += &format!("{2:0$.1$}%]",pcent_width,self.precision as usize,pcent);
        self.next_show_count = *[next_tic_count,next_ratio_count,self.max-1].iter().min().unwrap();

        // actual flushing of buffer will happen only when either progress in percentage or tics changes
        // `next_show_count` is capped with `max` to ensure last value is 100%

        print!("{}",s);
        stdout().flush();
    }
}

fn main(){
    let mut ps:[Particle;PTCLNUM] = [Default::default();PTCLNUM];
    let mut prog = ProgressBar::new((REP_N*PTCLNUM) as u64,50);

    let mut f = File::create("coord.dat").unwrap();

    for _ in 0..REP_N {
        for p in ps.iter_mut() {
            p.adv();
            prog.adv();
            write!(&mut f, "{:e} {:e} {:e}\n",p.r[0],p.r[1],p.r[2]);
        }
        write!(&mut f, "\n\n");
    }
}

