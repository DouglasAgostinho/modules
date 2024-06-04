
pub mod components{
    //Modules with main components to be used in proccess simulation    

    pub struct Vessel <'a> {
        //Vessel structure to simulate level physics

        pub shape:          &'a str,
        pub range:          [f32; 2],
        pub h_hh:           [f32; 2],
        pub l_ll:           [f32; 2],
        pub in_valve:       f32,
        pub in_pressure:    f32,
        pub out_valve:      f32,
        pub level:      f32,
    }
    impl <'a> Vessel <'a>{

        pub fn in_flow(&self) -> f32{

            self.in_valve * self.in_pressure
        }

        pub fn out_flow(&self) -> f32{

            self.out_valve * (self.level / 100.0)
        }
    }


    pub struct PID {
        //PID structure for proccess control
        
        pub mv: f32,
        pub sp: f32,
        pub kp: f32,
        pub ki: f32,
        pub kd: f32,
    }
    impl PID {
        

        pub fn control(&mut self, i: f32, mut prev_time: f32, integral: f32, mut e_prev: f32, pv: f32, debug: bool) -> (f32, f32, f32) {
            use std::{thread, time};

            //PID control logic        
            let time = i * 0.1;
                        
            //calculations start        
            let e: f32 = self.sp - pv;        
            
            let p: f32 = self.kp * e;        
            
            let integral = integral + (self.ki * e * (time - prev_time));

            
            let d: f32 = if i < 2.0 {
                0.0
            }
            else{
                self.kd * (e - e_prev) / (time - prev_time)
            };        
            
            self.mv = (p + integral + d) * -1.0;  

            if debug == true{
                println!("\n -----{}-----", i);
                println!("e = {}", e);
                println!("p = {}, kp= {}", p, self.kp);
                println!("integral = {}, ki = {}, time = {}, prev_time = {}", integral, self.ki, time, prev_time);
                println!("d = {}, kd = {}", d, self.kd);
                println!("Out valve = {}", self.mv);            
                println!("Level is {:.2} & Setpoint is {:.2}", pv, self.sp);    
                println!("\n -----{}-----", i);
                thread::sleep(time::Duration::from_millis(250));
            }      
            
            e_prev = e;
            prev_time = time;                

            if self.mv > 99.9 {
                self.mv = 100.0
            }
            else if self.mv < 0.01 {
                self.mv = 0.0
                
            }

            (prev_time, integral, e_prev)
            

        }
        
    }

}