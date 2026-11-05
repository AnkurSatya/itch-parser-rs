use criterion::{
    BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use feed_handler::{
    consumer::spsc::SpscConsumer,
    data_generator::{DataGenerator, ItchFrame, ItchOrder},
    feed::{FeedConsumer, FeedProducer},
    producer::spsc::SpscProducer,
};
use rtrb::RingBuffer;
use std::{
    hint::black_box,
    sync::{
        Arc, Barrier,
        atomic::{
            AtomicBool,
            Ordering::{Acquire, Release},
        },
    },
    thread,
};

const RING_BUFFER_CAPACITY: usize = 524_288;
const BATCH_SIZE: usize = 300_000;
const RANDOM_SEED: u64 = 20;

fn run_spsc_bench<T>(group: &mut BenchmarkGroup<WallTime>, name: &str, frames: Arc<Vec<ItchFrame>>)
where
    T: Send + 'static,
    SpscProducer<T>: FeedProducer<Item = ItchFrame>,
    SpscConsumer<T>: FeedConsumer,
{
    let barrier = Arc::new(Barrier::new(3));
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (producer, consumer) = RingBuffer::<T>::new(RING_BUFFER_CAPACITY);

    let producer_handle = {
        let barrier = Arc::clone(&barrier);
        let stop_flag = Arc::clone(&stop_flag);
        let frames = Arc::clone(&frames);
        thread::spawn(move || {
            let mut tx = SpscProducer::<T>::new(producer);
            loop {
                barrier.wait();
                if stop_flag.load(Acquire) {
                    break;
                }
                for frame in frames.iter() {
                    tx.send(black_box(*frame));
                }
                barrier.wait();
            }
        })
    };

    let consumer_handle = {
        let barrier = Arc::clone(&barrier);
        let stop_flag = Arc::clone(&stop_flag);
        thread::spawn(move || {
            let mut rx = SpscConsumer::<T>::new(consumer);
            loop {
                barrier.wait();
                if stop_flag.load(Acquire) {
                    break;
                }
                let mut received = 0;
                while received < BATCH_SIZE {
                    black_box(rx.recv());
                    received += 1;
                }
                barrier.wait();
            }
        })
    };

    group.bench_function(name, |b| {
        b.iter(|| {
            barrier.wait();
            barrier.wait();
        })
    });

    stop_flag.store(true, Release);
    barrier.wait();
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}

fn bench_spsc_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("spsc_ring_buffer");
    group.throughput(criterion::Throughput::Elements(BATCH_SIZE as u64));

    let frames = Arc::new(DataGenerator::bulk_generate_random_frames(
        None,
        BATCH_SIZE,
        Some(RANDOM_SEED),
    ));

    run_spsc_bench::<ItchFrame>(&mut group, "option1_pop_byte_frame", Arc::clone(&frames));
    run_spsc_bench::<ItchOrder>(&mut group, "option3_pop_typed_enum", frames);
}

criterion_group!(benches, bench_spsc_throughput);
criterion_main!(benches);
