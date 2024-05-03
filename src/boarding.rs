use crate::elevator::ElevatorDirection;

pub struct BoardingController {
    pub boarding_floor: i16,
    pub desired_direction: ElevatorDirection,
}

impl BoardingController {
    pub fn new(boarding_floor: i16, desired_direction: ElevatorDirection) -> Self {
        BoardingController {
            boarding_floor,
            desired_direction,
        }
    }
}