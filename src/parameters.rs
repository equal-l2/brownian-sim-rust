use std::f64::consts::PI;

/* physical pub constants */
pub const ABS_ZERO :f64 = -273.15; // absolute zero [degree Celsius]
pub const K :f64 = 1.380649e-23;   // Boltzmann constant [J/K]

/* parameters (user-defined)*/
pub const T_DEGC :f64 = 25.;     // temperature [degree Celsius]
pub const RHO :f64 = 2.699e+3;   // dencity of particle [kg/m^3]
pub const ETA :f64 = 0.890e-3;   // viscocity of fluid [Pa*s]
pub const RA :f64 = 100e-6;      // radius of particle [m]
pub const DT :f64 = 5.747e-4;    // step size in time [s]
pub const DIM :usize = 3;        // dimension [-]
pub const PTCLNUM :usize = 1000; // particle amount [-]
pub const SIM_TIME :f64 = 0.1;   // simulation time [s]
/* parameters (automatically calculated) */
pub const T :f64 = T_DEGC-ABS_ZERO;             // temperature in Kelvin [K]
pub const M :f64 = (4./3.)*PI*RA*RA*RA*RHO;     // mass of particle [kg]
pub const BETA :f64 = 6.*PI*RA*ETA/M;           // constant of drag [-]
pub const D :f64 = K*T/(M*BETA);                // diffusion coefficient [m^2/s]
pub const REP_N:usize = (SIM_TIME/DT) as usize; // number of repetition

