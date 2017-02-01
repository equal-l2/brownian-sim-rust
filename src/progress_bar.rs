use std::io::{stderr,Write};

pub struct ProgressBar{
    max :u64,
    width :u64,
    precision :i32,
    count :u64,
    next_show_count :u64
}

impl ProgressBar{
    pub fn new(max_in:u64, width_in:u64) -> ProgressBar {
        // max       : max count (count value on 100% progress)
        // width     : width of progress bar in character
        // precision : digits after decimal point

        // Caution
        // Too much precision can cause great performance decrease
        ProgressBar{max:max_in, width:width_in, precision:0, count:0, next_show_count:0}
    }

    pub fn adv(&mut self){
        self.adv_by(1);
    }

    pub fn adv_by(&mut self,step:u64){
        self.count += step;
        if self.count >= self.next_show_count {self.show();}
    }

    pub fn set_presicion(&mut self,precision_arg:i32){
        self.precision=precision_arg;
    }

    fn show(&mut self){
        let mut s = "".to_string();
        let count = self.count as f64;
        let max   = self.max   as f64;
        let width = self.width as f64;

        /* draw progress bar */
        let tics = ((count/max)*width) as u64;
        s += "\r[";
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

        write!(stderr(),"{}",s);
        stderr().flush();
    }
}
