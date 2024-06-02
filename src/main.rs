
pub mod modules;
//use std::io;

use modules::components::Vessel;
//use crate::ves

fn main() {
    println!("Hello, world!");

    
    let m_01 = Vessel{
        shape:          "cilinder",
        range:          [0.0, 2000.0],
        h_hh:           [1600.0, 1800.0],
        l_ll:           [400.0, 200.0],
        in_valve:       0.0,
        in_pressure:    0.0,
        out_valve:      0.0,
        level:          0.0,
    };

    let x = m_01.in_flow();
    
}
