use crate::{boarding::BoardingController, cabin::CabinController, ElevatorRequest};
use itertools::Itertools;
use std::{cmp, error::Error, fmt, thread, time};

pub struct Elevator {
    pub current_floor: i16,
    pub state: ElevatorState,
    pub direction: ElevatorDirection,
    is_door_open: bool,
    pub cabin_ctrl: CabinController,

    // ideally an Elevator does not 'have a' BoardingController,
    // instead receiving instructions via a pub/sub or similar
    pub boarding_ctrls: Vec<BoardingController>,
}

impl Elevator {
    fn open_door(&mut self, should_open: bool) {
        // todo: in IRL consider if the door is obstructed?
        let can_open = self.state == ElevatorState::Boarding;
        match should_open {
            true => {
                if can_open {
                    self.is_door_open = true
                }
            }
            false => self.is_door_open = false,
        };
    }

    pub fn run(&mut self, mut request_queue: Vec<ElevatorRequest>) -> Result<bool, ElevatorError> {
        while !request_queue.is_empty() {
            if self.direction == ElevatorDirection::None {
                self.direction = request_queue[0].direction;
            }

            // only consider floors going up/down per initial direction
            let mut floor_sequence_ord: Vec<i16> = request_queue
                .iter()
                .filter(|f| f.direction == self.direction)
                .flat_map(|m| [m.desired_floor, m.boarding_floor])
                .sorted()
                .unique()
                .collect_vec();

            if self.direction == ElevatorDirection::Down {
                floor_sequence_ord.reverse();
            }

            self.traverse_route(floor_sequence_ord, &mut request_queue)?;
        }

        Ok(true)
    }

    fn traverse_route(
        &mut self,
        floor_sequence_ord: Vec<i16>,
        request_queue: &mut Vec<ElevatorRequest>,
    ) -> Result<(), ElevatorError> {

        self.state = ElevatorState::Enroute;
        for target_floor in floor_sequence_ord.iter() {
            self.set_direction(*target_floor);

            while self.current_floor != *target_floor {
                self.open_door(false);

                self.next_floor()?;

                if self.current_floor == *target_floor {
                    println!(
                        "Elevator reaches floor {}. Boarding/Departing",
                        self.current_floor
                    );

                    request_queue.retain(|f| {
                        self.current_floor != f.desired_floor && self.direction != f.direction
                    });
                    
                    self.board_cabin();
                } else {
                    println!("Elevator passes floor {}", self.current_floor);
                }
            }
        }

        self.direction = ElevatorDirection::None;
        self.state = ElevatorState::Idle;

        Ok(())
    }

    fn next_floor(&mut self) -> Result<(), ElevatorError> {
        if self.is_door_open {
            return Err(ElevatorError::MoveRequestWithDoorOpen);
        }

        self.current_floor += self.direction as i16;

        // crudely simulate an elevator taking time between floors
        thread::sleep(time::Duration::from_millis(500));

        Ok(())
    }

    fn board_cabin(&mut self) {
        self.state = ElevatorState::Boarding;
        self.open_door(true);

        // simulate time taken to open doors/board
        thread::sleep(time::Duration::from_millis(250));

        // ===================================
        // this would be the point at which someone could 'enter' the cabin
        // and input their desired floor, however instructions are preconceived at this stage
        // note: last_desired_floor would need to be updated
        // ===================================

        self.open_door(false);
    }

    fn set_direction(&mut self, dest: i16) {
        self.direction = Elevator::get_direction(self.current_floor, dest);
    }

    fn get_direction(current: i16, target: i16) -> ElevatorDirection {
        match current.cmp(&target) {
            cmp::Ordering::Less => ElevatorDirection::Up,
            cmp::Ordering::Greater => ElevatorDirection::Down,
            cmp::Ordering::Equal => ElevatorDirection::None,
        }
    }

    pub fn with_floors(mut self, num_floors: i16) -> Self {
        if self.current_floor > num_floors {
            panic!("Cannot set num_floors to be lower than current_floor")
        }

        self.cabin_ctrl = CabinController::new(num_floors);
        self.boarding_ctrls = BoardingController::new(num_floors);
        self
    }

    pub fn set_current_floor(mut self, current_floor: i16) -> Self {
        self.current_floor = current_floor;
        self
    }
}

impl Default for Elevator {
    fn default() -> Self {
        let num_floors = 2;
        Self {
            current_floor: Default::default(),
            state: Default::default(),
            direction: ElevatorDirection::None,
            is_door_open: false,
            cabin_ctrl: CabinController::new(num_floors),
            boarding_ctrls: BoardingController::new(num_floors),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ElevatorError {
    MoveRequestWithDoorOpen,
}

impl fmt::Display for ElevatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            ElevatorError::MoveRequestWithDoorOpen => "door open while attempting to move elevator",
        };
        f.write_str(description)
    }
}

impl Error for ElevatorError {}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum ElevatorDirection {
    Up = 1,
    Down = -1,

    #[default]
    None = 0,
}

#[derive(Default, PartialEq)]
pub enum ElevatorState {
    Enroute,
    Boarding,

    #[default]
    Idle,
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    use crate::elevator::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn next_floor_when_door_is_open_error() {
        let mut sup = Elevator {
            state: ElevatorState::Boarding,
            is_door_open: true,
            ..Default::default()
        };

        let actual = sup.next_floor();

        assert_eq!(actual, Err(ElevatorError::MoveRequestWithDoorOpen));
    }

    #[test]
    fn run_elevator_travels_down() {
        let mut sup = Elevator {
            current_floor: 3,
            ..Default::default()
        }
        .with_floors(3);

        let _ = sup.run(vec![ElevatorRequest::new(2, 1, ElevatorDirection::Down)]);

        assert_eq!(sup.current_floor, 1)
    }

    #[test]
    fn run_elevator_travels_up() {
        let mut sup = Elevator {
            current_floor: 3,
            ..Default::default()
        };

        let _ = sup.run(vec![ElevatorRequest::new(1, 2, ElevatorDirection::Up)]);

        assert_eq!(sup.current_floor, 2)
    }

    // Passenger summons lift on the ground floor. Once in, chooses to go to level 5.
    #[test]
    fn scenario_1() {
        let boarding_floor = 1;
        let desired_floor = 5;

        let mut sup = Elevator {
            current_floor: 3,
            ..Default::default()
        }
        .with_floors(5);

        let _ = sup.run(vec![ElevatorRequest::new(
            boarding_floor,
            desired_floor,
            ElevatorDirection::Up,
        )]);

        assert_eq!(sup.current_floor, desired_floor)
    }

    // todo: implement ElevatorError when requests are made beyond storey boundaries
    // Passenger summons lift on level 6 to go down. 
    // Passenger on level 4 summons the lift to go down. They both choose L1.
    #[test]
    fn scenario_2() {
        let current_floor = 7;
        let p1_boarding_floor = 6;
        let p2_boarding_floor = 4;
        let desired_floor = 1;

        let mut sup = Elevator {
            current_floor,
            ..Default::default()
        }
        .with_floors(7);

        let _ = sup.run(vec![
            ElevatorRequest::new(p1_boarding_floor, desired_floor, ElevatorDirection::Down),
            ElevatorRequest::new(p2_boarding_floor, desired_floor, ElevatorDirection::Down),
        ]);

        assert_eq!(sup.current_floor, desired_floor)
    }

    // Passenger summons lift on level 6 to go down. 
    // Passenger on level 4 summons the lift to go down. They both choose L1.
    #[test]
    fn scenario_2_opposite() {
        let current_floor = 3;
        let p1_boarding_floor = 4;
        let p2_boarding_floor = 6;
        let desired_floor = 8;

        let mut sup = Elevator {
            current_floor,
            ..Default::default()
        }
        .with_floors(8);

        let _ = sup.run(vec![
            ElevatorRequest::new(p1_boarding_floor, desired_floor, ElevatorDirection::Up),
            ElevatorRequest::new(p2_boarding_floor, desired_floor, ElevatorDirection::Up),
        ]);

        assert_eq!(sup.current_floor, desired_floor)
    }

    // Passenger 1 summons lift to go up from L2
    // Passenger 1 chooses to go to L6
    // Passenger 2 summons lift to go down from L4
    // Passenger 2 chooses to go to Ground Floor
    #[test]
    fn scenario_3() {
        let current_floor = 3;
        let p1_boarding_floor = 2;
        let p1_desired_floor_1 = 6;

        let p2_boarding_floor = 4;
        let p2_desired_floor = 1;

        let mut sup = Elevator {
            current_floor,
            ..Default::default()
        }
        .with_floors(8);

        let _ = sup.run(vec![
            ElevatorRequest::new(p1_boarding_floor, p1_desired_floor_1, ElevatorDirection::Up),
            ElevatorRequest::new(p2_boarding_floor, p2_desired_floor, ElevatorDirection::Down),
        ]);

        assert_eq!(sup.current_floor, p2_desired_floor)
    }

    // Passenger 1 summons lift to go up from Ground. They choose L5. 
    // Passenger 2 summons lift to go down from L4. 
    // Passenger 3 summons lift to go down from L10. 
    // Passengers 2 and 3 choose to travel to Ground.
    #[test]
    fn scenario_4() {
        let current_floor = 1;
        let p1_boarding_floor = 2;
        let p1_desired_floor_1 = 5;

        let p2_boarding_floor = 4;
        let p2_desired_floor = 1;

        let p3_boarding_floor = 10;
        let p3_desired_floor = 1;

        let mut sup = Elevator {
            current_floor,
            ..Default::default()
        }
        .with_floors(10);

        let _ = sup.run(vec![
            ElevatorRequest::new(p1_boarding_floor, p1_desired_floor_1, ElevatorDirection::Up),
            ElevatorRequest::new(p2_boarding_floor, p2_desired_floor, ElevatorDirection::Down),
            ElevatorRequest::new(p3_boarding_floor, p3_desired_floor, ElevatorDirection::Down),

        ]);

        assert_eq!(sup.current_floor, p2_desired_floor)
    }

    // bug: elevator fails to pickup passengers going down when lift is summoned from below, and first passenger is going up
    // resolved when cabin controller, floor requests would only become active once lift is summoned
    #[test]
    fn scenario_5() {
        
    }
}
