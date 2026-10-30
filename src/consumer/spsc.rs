use crate::data_generator::ItchOrder;
use crate::feed::FeedConsumer;
use rtrb::Consumer;

pub struct SpscConsumer {
    data_rx: Consumer<ItchOrder>,
}

impl SpscConsumer {
    pub fn new(data_rx: Consumer<ItchOrder>) -> Self {
        Self { data_rx }
    }
}

impl FeedConsumer for SpscConsumer {
    fn recv(&mut self) {
        self.data_rx.peek();
    }
}
// ToDo: Implement criterion yourself so that proper checking can be done.
// ToDo: Evalute all the criteria. Start with tail latency. Thing to measure is
// time taken from just before the message is pushed to the queue to the point right
// after receiver reads it.
