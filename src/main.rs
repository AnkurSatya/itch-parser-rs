use feed_handler::data_generator::DataGenerator;
use feed_handler::data_generator::ItchAddOrderMsg;
use feed_handler::data_generator::ItchOrder;
use rtrb::{self, Consumer, Producer};
fn main() {
    let capacity = 1000;
    let (tx, rx) = rtrb::RingBuffer::<ItchOrder>::new(capacity);
    // let data_generator = DataGenerator
}
