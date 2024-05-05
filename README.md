
# Summary
Yo are in charge of writing software for an elevator (lift) company.
Your task is to write a program to control the travel of a lift for a 10 storey building.
A passenger can summon the lift to go up or down from any floor, once in the lift they can choose the floor they’d like to travel to.
Your program needs to plan the optimal set of instructions for the lift to travel, stop, and open its doors.

## Instructions 
    - Install Rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    Visit https://www.rust-lang.org/learn/get-started for further instructions

    - Clone repo

    - cargo run: for default operation

    - cargo test: run all unit tests

## Requirements
    - An Elevator should be capable of traversing each floor of a 10 storey build.
    - The elevator can be summoned by a passenger to go either up or down, from any floor, with the exception of down on the first floor.
    - The passenger can select the destination floor once inside the lift.
    - The lift system should pickup and deliver passengers optimally.

## Assumptions
    - Multiple passengers can engage with the lift system at one time, on the same and different floors.
    - Only one lift operates at a time (single lift track)
    - A passenger may stop at other floors before arriving at their chosen floor according to any other passengers that they may be sharing the lift with.

## Test cases:
    - Scenario 1: Passenger summons lift on the ground floor. Once in, chooses to go to level 5.
    - Scenario 2: Passenger summons lift on level 6 to go down. Passenger on level 4 summons the lift to go down. They both choose L1.

    
    
    - Scenario 3: Passenger 1 summons lift to go up from L2.
                  Passenger 1 chooses to go to L6 
                  Passenger 2 summons lift to go down from L4.
                  Passenger 2 chooses to go to Ground Floor



    - Scenario 4: Passenger 1 summons lift to go up from Ground. They choose L5. 
                  Passenger 2 summons lift to go down from L4. 
                  Passenger 3 summons lift to go down from L10. 
                  Passengers 2 and 3 choose to travel to Ground.