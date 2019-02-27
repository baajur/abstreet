use crate::plugins::sim::new_des_model::ParkingSpot;
use map_model::{BuildingID, BusStopID, Traversable, TurnID};
use sim::{AgentID, CarID, PedestrianID};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    // TODO CarFinishedParking
    // TODO and the pedestrian / trip associated with it?
    CarReachedParkingSpot(CarID, ParkingSpot),
    // TODO and the car / trip?
    PedReachedParkingSpot(PedestrianID, ParkingSpot),
    // TODO CarFinishedUnparking
    BusArrivedAtStop(CarID, BusStopID),
    BusDepartedFromStop(CarID, BusStopID),

    PedReachedBuilding(PedestrianID, BuildingID),
    PedReachedBusStop(PedestrianID, BusStopID),
    PedEntersBus(PedestrianID, CarID),
    PedLeavesBus(PedestrianID, CarID),

    // TODO split up into cases or not?
    AgentEntersTraversable(AgentID, Traversable),
    AgentLeavesTraversable(AgentID, Traversable),

    // TODO maybe AgentRequestsTurn?
    IntersectionAcceptsRequest(AgentID, TurnID),
}