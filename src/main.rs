mod elevator;
mod boarding;
mod cabin;

use crate::{boarding::BoardingController, cabin::CabinController};
use std::{error::Error, vec};

use crate::elevator::{Elevator, ElevatorDirection};

fn main() -> Result<(), Box<dyn Error>> {

    const PASSENGER_CURRENT_FLOOR: i16 = 1;
    const ELEVATOR_CURRENT_FLOOR: i16 = 5;
    const PASSENGER_DESIRED_FLOOR: i16 = 2;

    let boarding_ctrl = BoardingController::new(PASSENGER_CURRENT_FLOOR, ElevatorDirection::Up);
    let cabin_ctrl = CabinController::new(PASSENGER_DESIRED_FLOOR);

    let floors = vec![boarding_ctrl.boarding_floor, cabin_ctrl.desired_floor];

    let mut elevator = Elevator::default();
    elevator.current_floor = ELEVATOR_CURRENT_FLOOR;
    // elevator.target_floor = cabin_ctrl.desired_floor;

    println!("Calling elevator to {} floor", boarding_ctrl.boarding_floor);
    println!("Elevator currently on {} floor", elevator.current_floor);

    elevator.run(floors)?;

    Ok(())
}
struct Passenger {
    weight: u16,
}