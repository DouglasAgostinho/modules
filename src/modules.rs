
//#[warn(dead_code)]
pub mod components{
    
    pub struct Vessel <'a> {
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

}