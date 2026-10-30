pub trait FeedProducer {
    type Item;
    fn send(&mut self, item: Self::Item);
}

pub trait FeedConsumer {
    fn recv(&mut self);
}
