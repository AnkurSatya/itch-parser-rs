use rand::prelude::*;
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use zerocopy::{FromBytes, IntoBytes};

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum ItchOrder {
    Add(ItchAddOrderMsg),
    Executed(ItchOrderExecutedMsg),
    Cancel(ItchOrderCancelMsg),
    Delete(ItchOrderDeleteMsg),
}
#[repr(C, packed)]
#[derive(Default, Debug, IntoBytes, FromBytes, Clone, Copy)]
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

#[repr(C, packed)]
#[derive(Default, Debug, IntoBytes, FromBytes, Clone, Copy)]
pub struct ItchOrderExecutedMsg {
    pub msg_type: u8,         // 1 byte: 'E'
    pub stock_locate: u16,    // 2 bytes
    pub tracking_number: u16, // 2 bytes
    pub timestamp: [u8; 6],   // 6 bytes
    pub order_reference: u64, // 8 bytes: References the original Add Order
    pub executed_shares: u32, // 4 bytes: Number of shares executed
    pub match_number: u64,    // 8 bytes: Unique match/trade ID
} // Total 31 bytes

#[repr(C, packed)]
#[derive(Default, Debug, IntoBytes, FromBytes, Clone, Copy)]
pub struct ItchOrderCancelMsg {
    pub msg_type: u8,         // 1 byte: 'X'
    pub stock_locate: u16,    // 2 bytes
    pub tracking_number: u16, // 2 bytes
    pub timestamp: [u8; 6],   // 6 bytes
    pub order_reference: u64, // 8 bytes: References the original Add Order
    pub canceled_shares: u32, // 4 bytes: Number of shares removed
} // Total: 22 bytes

#[repr(C, packed)]
#[derive(Default, Debug, IntoBytes, FromBytes, Clone, Copy)]
pub struct ItchOrderDeleteMsg {
    pub msg_type: u8,         // 1 byte: 'D'
    pub stock_locate: u16,    // 2 bytes
    pub tracking_number: u16, // 2 bytes
    pub timestamp: [u8; 6],   // 6 bytes
    pub order_reference: u64, // 8 bytes: The order being wiped out
} // Total: 18 bytes

pub struct DataGenerator;

impl DataGenerator {
    pub fn bulk_generate_random(
        order_type: Option<ItchOrder>,
        n: usize,
        seed: Option<u64>,
    ) -> Vec<ItchOrder> {
        // For now, we are just using the default value for all the variants on ItchOrder,
        // so all the orders of the same variant will be exactly the same.
        if let Some(order_type) = order_type {
            vec![order_type; n]
        } else {
            let mut orders = Vec::with_capacity(n);
            let seed = seed.unwrap_or_default();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            for _ in 0..n {
                orders.push(ItchOrder::iter().choose(&mut rng).unwrap());
            }
            orders
        }
    }

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
