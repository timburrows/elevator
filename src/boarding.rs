use crate::elevator::ElevatorDirection;

pub struct BoardingController {
    pub boarding_floor: i16,
    pub desired_direction: ElevatorDirection,
}

impl BoardingController {
    pub fn new(num_floors: i16) -> Vec<BoardingController> {
        // todo: remove unwrap(), research more idomatic/safe conversion between usize and i16
        // we know the num_floors are not going to exceed usize::MAX... probably
        let mut boarding_ctrl = Vec::with_capacity((num_floors - 1).try_into().unwrap());
        for floor in 1..=num_floors {
            boarding_ctrl.push(BoardingController {
                boarding_floor: floor,
                desired_direction: ElevatorDirection::Down,
            });
        }
        boarding_ctrl
    }
}
