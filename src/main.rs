extern crate rand;
use std::fs::File;
use std::io::{stdout,Write};
mod progress_bar;
mod parameters;
mod particle;
use parameters::*;

fn main(){
    let mut ps:[particle::Particle;PTCLNUM] = [Default::default();PTCLNUM];
    let mut prog = progress_bar::ProgressBar::new(REP_N as u64,50);

    let mut f = File::create("coord.dat").unwrap();

    for _ in 0..REP_N {
        for p in ps.iter_mut() {
            p.adv();
            write!(&mut f, "{:e} {:e} {:e}\n",p.r[0],p.r[1],p.r[2]);
        }
        prog.adv();
        write!(&mut f, "\n\n");
    }
}

