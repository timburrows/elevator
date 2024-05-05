
pub struct CabinController {
    pub buttons: Vec<FloorButton>,
}

impl CabinController {
    pub fn new(available_floors: i16) -> Self {
        CabinController { buttons: Vec::with_capacity(available_floors.try_into().unwrap()) }
    }
}

pub struct FloorButton {
    floor: i16,
    is_pressed: bool,
}