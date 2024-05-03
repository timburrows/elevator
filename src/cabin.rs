pub struct CabinController {
    pub desired_floor: i16,
}

impl CabinController {
    pub fn new(desired_floor: i16) -> Self {
        CabinController { desired_floor }
    }
}