
#[derive(Clone, Debug)]
pub struct Ticket {
    pub seat_type: String,
    pub amount: u8,
    pub cost: f32,
}
impl Ticket {
    pub fn with_default() -> Self {
        Self {
            seat_type: String::new(),
            amount: 0,
            cost: 0.0,
        }
    }
    pub fn new(seat_type: String, amount: u8, cost: f32) -> Self {
        Self{
            seat_type,
            amount,
            cost
        }
    }

}