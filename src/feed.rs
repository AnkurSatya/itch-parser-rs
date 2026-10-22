use crate::data_generator::ItchOrder;
use rtrb::Producer;

pub struct FeedHandler {
    data_tx: Producer<ItchOrder>,
}

impl FeedHandler {
    pub fn new(data_tx: Producer<ItchOrder>) -> Self {
        Self { data_tx }
    }

    pub fn send_order(&mut self, order: ItchOrder) {
        // ToDo: Handle the push error gracefully.
        self.data_tx.push(order).ok();
    }
}
