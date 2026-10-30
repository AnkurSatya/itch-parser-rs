use crate::data_generator::ItchOrder;
use crate::feed::FeedProducer;
use rtrb::Producer;

pub struct SpscProducer {
    data_tx: Producer<ItchOrder>,
}

impl SpscProducer {
    pub fn new(data_tx: Producer<ItchOrder>) -> Self {
        Self { data_tx }
    }
}

impl FeedProducer for SpscProducer {
    type Item = ItchOrder;

    fn send(&mut self, item: Self::Item) {
        self.data_tx.push(item);
    }
}
