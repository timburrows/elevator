use std::{cmp, collections::VecDeque, error::Error, fmt, thread, time};

use crate::cabin::CabinController;

pub struct Elevator {
    pub current_floor: i16,
    pub target_floor: i16,
    pub state: ElevatorState,
    pub direction: ElevatorDirection,
    is_door_open: bool,
    pub cabin_ctrl: CabinController,
    // pub request_queue: Vec<i16>,
}

impl Elevator {
    fn open_door(&mut self, should_open: bool) {
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

    pub fn run(&mut self, request_queue: Vec<i16>) -> Result<bool, ElevatorError> {
        self.set_direction(request_queue.first().expect("Request queue was empty"));

        if self.is_door_open {
            return Err(ElevatorError::MoveRequestWithDoorOpen);
        }

        for (i, requested_floor) in request_queue.iter().enumerate() {
            while self.current_floor != *requested_floor {
                self.set_moving();

                // crudely simulate an elevator taking time between floors
                thread::sleep(time::Duration::from_millis(500));

                self.current_floor += self.direction as i16;

                let last_floor = request_queue
                    .last()
                    .expect("Why would there not be a last floor?");

                if self.current_floor == *requested_floor {
                    if requested_floor == last_floor {
                        println!(
                            "Elevator reaches floor {}. Standing by for further instructions",
                            self.current_floor
                        );
                        break;
                    }

                    println!(
                        "Elevator reaches floor {}. Now boarding",
                        self.current_floor
                    );

                    self.set_boarding();

                    // sort request_queue by desired_floor, except for floors < current_floor

                    // elevator recalculates direction per desired floor
                    self.set_direction(&request_queue[i + 1]);

                    // Doors close
                    // Elevator state changes to Moving
                    self.set_moving();

                    // todo: this would be the point at which someone could 'enter' the cabin
                    // and input their desired floor, requires inserting
                    // in the middle of the queue if their desired floor is not
                    // the highest floor in the queue
                } else {
                    println!("Elevator passes {} floor", self.current_floor);
                }
            }
        }

        Ok(true)
    }

    fn set_moving(&mut self) {
        // todo: is the door obstructed?
        self.open_door(false);
        self.state = ElevatorState::Moving;

        // simulate time taken to close doors/start moving
        thread::sleep(time::Duration::from_millis(250));
    }

    fn set_boarding(&mut self) {
        self.state = ElevatorState::Boarding;
        self.open_door(true);

        // simulate time taken to open doors/board
        thread::sleep(time::Duration::from_millis(250));
    }

    fn set_direction(&mut self, dest: &i16) {
        self.direction = match self.current_floor.cmp(dest) {
            cmp::Ordering::Less => ElevatorDirection::Up,
            cmp::Ordering::Greater => ElevatorDirection::Down,
            cmp::Ordering::Equal => ElevatorDirection::None,
        };
    }
}

impl Default for Elevator {
    fn default() -> Self {
        Self {
            current_floor: Default::default(),
            target_floor: Default::default(),
            state: Default::default(),
            direction: ElevatorDirection::Up,
            is_door_open: false,
            cabin_ctrl: CabinController::new(0),
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

#[derive(Default, PartialEq, Copy, Clone)]
pub enum ElevatorDirection {
    Up = 1,
    Down = -1,

    #[default]
    None = 0,
}

#[derive(Default, PartialEq)]
pub enum ElevatorState {
    Moving,
    Boarding,

    #[default]
    Idle,
}

#[cfg(test)]
mod tests {
    use crate::elevator::{Elevator, ElevatorError};
    use pretty_assertions::assert_eq;

    #[test]
    fn run_when_door_is_open_error() {
        let mut elevator = Elevator {
            is_door_open: true,
            ..Default::default()
        };

        let actual = elevator.run(1);
        assert_eq!(actual, Err(ElevatorError::MoveRequestWithDoorOpen));
    }

    #[test]
    fn run_elevator_travels_down() {
        let mut elevator = Elevator {
            is_door_open: false,
            current_floor: 2,
            target_floor: 0,
            ..Default::default()
        };

        let boarding_floor = 1;
        let _ = elevator.run(boarding_floor);

        assert_eq!(elevator.current_floor, elevator.target_floor)
    }

    #[test]
    fn run_elevator_travels_up() {
        let mut elevator = Elevator {
            is_door_open: false,
            current_floor: 0,
            target_floor: 2,
            ..Default::default()
        };

        let boarding_floor = 1;
        let _ = elevator.run(boarding_floor);

        assert_eq!(elevator.current_floor, elevator.target_floor)
    }
}
