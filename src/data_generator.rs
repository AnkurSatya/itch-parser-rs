#[derive(Debug)]
pub enum ItchOrder {
    Add(ItchAddOrderMsg),
    Executed(ItchOrderExecutedMsg),
    Cancel(ItchOrderCancelMsg),
    Delete(ItchOrderDeleteMsg),
}

#[derive(Default, Debug)]
pub struct ItchAddOrderMsg {
    pub msg_type: u8,           // 1 byte:  'A'
    pub stock_locate: u16,      // 2 bytes: Big-endian stock locate code
    pub tracking_number: u16,   // 2 bytes: Big-endian tracking number
    pub timestamp: [u8; 6],     // 6 bytes: 48-bit nanoseconds since midnight
    pub order_reference: u64,   // 8 bytes: Unique order reference number
    pub buy_sell_indicator: u8, // 1 byte:  'B' or 'S'
    pub shares: u32,            // 4 bytes: Number of shares
    pub price: u32,             // 4 bytes: Price (4 decimal places)
} // Total = 36 bytes

#[derive(Default, Debug)]
pub struct ItchOrderExecutedMsg {
    pub msg_type: u8,         // 1 byte: 'E'
    pub stock_locate: u16,    // 2 bytes
    pub tracking_number: u16, // 2 bytes
    pub timestamp: [u8; 6],   // 6 bytes
    pub order_reference: u64, // 8 bytes: References the original Add Order
    pub executed_shares: u32, // 4 bytes: Number of shares executed
    pub match_number: u64,    // 8 bytes: Unique match/trade ID
} // Total 31 bytes

#[derive(Default, Debug)]
pub struct ItchOrderCancelMsg {
    pub msg_type: u8,         // 1 byte: 'X'
    pub stock_locate: u16,    // 2 bytes
    pub tracking_number: u16, // 2 bytes
    pub timestamp: [u8; 6],   // 6 bytes
    pub order_reference: u64, // 8 bytes: References the original Add Order
    pub canceled_shares: u32, // 4 bytes: Number of shares removed
} // Total: 22 bytes

#[derive(Default, Debug)]
pub struct ItchOrderDeleteMsg {
    pub msg_type: u8,         // 1 byte: 'D'
    pub stock_locate: u16,    // 2 bytes
    pub tracking_number: u16, // 2 bytes
    pub timestamp: [u8; 6],   // 6 bytes
    pub order_reference: u64, // 8 bytes: The order being wiped out
} // Total: 18 bytes

pub struct DataGenerator {}

impl DataGenerator {
    pub fn run() {}

    pub fn generate_orders() {}

    pub fn generate_add_order() -> ItchAddOrderMsg {
        ItchAddOrderMsg::default()
    }

    pub fn generate_execute_order() -> ItchOrderExecutedMsg {
        ItchOrderExecutedMsg::default()
    }

    pub fn generate_cancel_order() -> ItchOrderCancelMsg {
        ItchOrderCancelMsg::default()
    }

    pub fn generate_delete_order() -> ItchOrderDeleteMsg {
        ItchOrderDeleteMsg::default()
    }
}
