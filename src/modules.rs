
pub mod components{
    
    struct Vessel <'a> {
        shape:          &'a str,
        range:          [f32; 2],
        h_hh:           [f32; 2],
        l_ll:           [f32; 2],
        in_valve:       f32,
        in_pressure:    f32,
        out_valve:      f32,
    }
    impl <'a> Vessel <'a>{

        fn flow(&self) -> f32{

            return self.in_valve * self.in_pressure;
        }

    }

}