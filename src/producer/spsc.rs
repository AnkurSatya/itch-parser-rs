use crate::data_generator::{
    ItchAddOrderMsg, ItchFrame, ItchOrder, ItchOrderCancelMsg, ItchOrderDeleteMsg,
    ItchOrderExecutedMsg,
};
use crate::feed::FeedProducer;
use rtrb::Producer;
use zerocopy::FromBytes;

pub struct SpscProducer<T> {
    data_tx: Producer<T>,
}

impl<T> SpscProducer<T> {
    pub fn new(data_tx: Producer<T>) -> Self {
        Self { data_tx }
    }
}

// Deferred parsing. Producer pushes raw bytes.
// Consumer parses just be pointer re-casting

impl FeedProducer for SpscProducer<ItchFrame> {
    type Item = ItchFrame;
    fn send(&mut self, item: Self::Item) {
        while self.data_tx.push(item).is_err() {
            std::hint::spin_loop();
        }
    }
}

// Eager parsing. Producer parses, then pushes the typed value.
// Consumer pops an already built type.
impl FeedProducer for SpscProducer<ItchOrder> {
    type Item = ItchFrame;

    fn send(&mut self, item: Self::Item) {
        let order = match item[0] {
            b'A' => ItchOrder::Add(
                *ItchAddOrderMsg::ref_from_bytes(&item[..std::mem::size_of::<ItchAddOrderMsg>()])
                    .unwrap(),
            ),
            b'E' => ItchOrder::Executed(
                *ItchOrderExecutedMsg::ref_from_bytes(
                    &item[..std::mem::size_of::<ItchOrderExecutedMsg>()],
                )
                .unwrap(),
            ),
            b'X' => ItchOrder::Cancel(
                *ItchOrderCancelMsg::ref_from_bytes(
                    &item[..std::mem::size_of::<ItchOrderCancelMsg>()],
                )
                .unwrap(),
            ),
            b'D' => ItchOrder::Delete(
                *ItchOrderDeleteMsg::ref_from_bytes(
                    &item[..std::mem::size_of::<ItchOrderDeleteMsg>()],
                )
                .unwrap(),
            ),
            _ => return,
        };
        while self.data_tx.push(order).is_err() {
            std::hint::spin_loop();
        }
    }
}
