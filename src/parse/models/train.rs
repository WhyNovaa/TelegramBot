use crate::parse::models::ticket::Ticket;

#[derive(Debug)]
pub struct Train {
    pub number: String,
    pub route: String,
    pub start_station: String,
    pub end_station: String,
    pub start_time: String,
    pub end_time: String,
    pub tickets: Vec<Ticket>,
}

impl Train {
    pub fn with_default() -> Self{
        Self {
            number: String::new(),
            route: String::new(),
            start_station: String::new(),
            end_station: String::new(),
            start_time: String::new(),
            end_time: String::new(),
            tickets: Vec::new(),
        }
    }
    pub fn new(number: String, route: String, start_station: String,
           end_station: String, start_time: String, end_time: String, tickets: Vec<Ticket>) -> Self {
        Self {
            number,
            route,
            start_station,
            end_station,
            start_time,
            end_time,
            tickets,
        }
    }
}