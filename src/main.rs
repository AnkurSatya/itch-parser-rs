use feed_handler::data_generator::ItchOrder;
use rtrb::RingBuffer;
fn main() {
    let capacity = 1000;
    let (_tx, _rx) = RingBuffer::<ItchOrder>::new(capacity);
    let (_tx, _rx) = RingBuffer::<&[u8]>::new(capacity);
    // let data_generator = DataGenerator
}
