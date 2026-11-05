use crate::data_generator::{
    ItchAddOrderMsg, ItchFrame, ItchOrder, ItchOrderCancelMsg, ItchOrderDeleteMsg,
    ItchOrderExecutedMsg,
};
use crate::feed::FeedConsumer;
use rtrb::Consumer;
use zerocopy::FromBytes;

pub struct SpscConsumer<T> {
    data_rx: Consumer<T>,
}

impl<T> SpscConsumer<T> {
    pub fn new(data_rx: Consumer<T>) -> Self {
        Self { data_rx }
    }
}

impl FeedConsumer for SpscConsumer<ItchOrder> {
    fn recv(&mut self) {
        let _order = loop {
            match self.data_rx.pop() {
                Ok(o) => break o,
                Err(_) => std::hint::spin_loop(),
            }
        };
    }
}

impl FeedConsumer for SpscConsumer<ItchFrame> {
    fn recv(&mut self) {
        let frame = loop {
            match self.data_rx.pop() {
                Ok(f) => break f,
                Err(_) => std::hint::spin_loop(),
            }
        };
        match frame[0] {
            b'A' => {
                let _order = ItchAddOrderMsg::ref_from_bytes(
                    &frame[..std::mem::size_of::<ItchAddOrderMsg>()],
                )
                .unwrap();
            }
            b'E' => {
                let _order = ItchOrderExecutedMsg::ref_from_bytes(
                    &frame[..std::mem::size_of::<ItchOrderExecutedMsg>()],
                )
                .unwrap();
            }
            b'X' => {
                let _order = ItchOrderCancelMsg::ref_from_bytes(
                    &frame[..std::mem::size_of::<ItchOrderCancelMsg>()],
                )
                .unwrap();
            }
            b'D' => {
                let _order = ItchOrderDeleteMsg::ref_from_bytes(
                    &frame[..std::mem::size_of::<ItchOrderDeleteMsg>()],
                )
                .unwrap();
            }
            _ => {}
        };
    }
}
