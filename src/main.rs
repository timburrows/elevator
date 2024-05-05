mod boarding;
mod cabin;
mod elevator;

use std::error::Error;
use elevator::ElevatorDirection;

use crate::elevator::Elevator;

fn main() -> Result<(), Box<dyn Error>> {
    const ELEVATOR_CURRENT_FLOOR: i16 = 5;
    const NUM_FLOORS: i16 = 10;

    // let floor_requests = VecDeque::from(vec![
    //     ElevatorRequest::new(1, 2),
    //     ElevatorRequest::new(2, 3),

    //     //elevator doesnt go down afterward
    //     ElevatorRequest::new(3, 1),
    // ]);


    // let floor_requests = VecDeque::from(vec![
    //     ElevatorRequest::new(1, 2),
    // ]);

    // multiple floors
    // let floor_requests = vec![
    //     ElevatorRequest::new(1, 2),
    //     ElevatorRequest::new(2, 3),
    // ];

    // multiple floors with direction change at top
    let floor_requests = vec![
        ElevatorRequest::new(1, 2, ElevatorDirection::Up),
        ElevatorRequest::new(2, 3, ElevatorDirection::Up),
        ElevatorRequest::new(3, 1, ElevatorDirection::Down),
    ];

    println!("{:?}", floor_requests);


    let mut elevator = Elevator::default()
        .with_floors(NUM_FLOORS)
        .set_current_floor(ELEVATOR_CURRENT_FLOOR);

    println!("Elevator currently on floor {}", elevator.current_floor);

    elevator.run(floor_requests)?;

    Ok(())
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct ElevatorRequest {
    boarding_floor: i16,
    desired_floor: i16,
    direction: ElevatorDirection,
    is_complete: bool
}

impl Ord for ElevatorRequest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.desired_floor.cmp(&other.desired_floor)
    }
}

impl PartialOrd for ElevatorRequest {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.desired_floor.cmp(&other.desired_floor))
    }
}

impl ElevatorRequest {
    fn new(boarding_floor: i16, desired_floor: i16, direction: ElevatorDirection) -> Self {
        Self {
            boarding_floor,
            desired_floor,
            direction,
            is_complete: false,
        }
    }
}
