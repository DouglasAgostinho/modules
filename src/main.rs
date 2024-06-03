
pub mod modules;
//use std::io;

use modules::components::{Vessel, PID};
//use crate::ves

fn main() {
    println!("Hello, world!");

    
    let mut m_01 = Vessel{
        shape:          "cilinder",
        range:          [0.0, 2000.0],
        h_hh:           [1600.0, 1800.0],
        l_ll:           [400.0, 200.0],
        in_valve:       10.0,
        in_pressure:    10.0,
        out_valve:      0.0,
        level:          0.0,
    };

    let mut pid = PID{
        mv: 0.0,
        sp: 500.0,
        kp: 0.002,
        ki: 0.0001,
        kd: 0.002,
    };

    let mut i = 0.0;
    let mut prev_time = 0.0;
    let mut integral = 0.0;    
    let mut e_prev = 0.0;

    loop{

        i += 1.0;

        (prev_time, integral, e_prev ) = pid.control(i, prev_time, integral, e_prev, m_01.level, true);

        m_01.level = m_01.level + m_01.in_flow() - m_01.out_flow();

        m_01.out_valve = pid.mv;


        println!(" Level is {}, input flow is {}, output flow is {}, outuput valve is {}", m_01.level, m_01.in_flow(), m_01.out_flow(), m_01.out_valve);
    }
    
}
