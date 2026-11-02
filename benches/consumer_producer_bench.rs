use criterion::{Criterion, criterion_group, criterion_main};
use feed_handler::{
    consumer::spsc::SpscConsumer,
    data_generator::{self, ItchOrder},
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
    thread::{self},
};

fn bench_spsc_throughput(c: &mut Criterion) {
    let mut spsc_ring_buffer_group = c.benchmark_group("spsc_ring_buffer");
    const RING_BUFFER_CAPACITY: usize = 131072;
    const BATCH_SIZE: usize = 300_000;
    const RANDOM_SEED: u64 = 20;
    spsc_ring_buffer_group.throughput(criterion::Throughput::Elements(BATCH_SIZE as u64));

    let barrier = Arc::new(Barrier::new(3));
    let stop_flag = Arc::new(AtomicBool::new(false));

    // static FLAG_THREAD_START_ACTIVITY: AtomicBool = AtomicBool::new(false);

    let (producer_handle, consumer_handle) = {
        let (producer, consumer) = RingBuffer::<ItchOrder>::new(RING_BUFFER_CAPACITY);

        let setup_data = data_generator::DataGenerator::bulk_generate_random(
            None,
            BATCH_SIZE,
            Some(RANDOM_SEED),
        );

        let producer_handle = {
            let barrier = Arc::clone(&barrier);
            let stop_flag = Arc::clone(&stop_flag);
            thread::spawn(move || {
                let mut tx = SpscProducer::new(producer);
                loop {
                    barrier.wait();
                    if stop_flag.load(Acquire) {
                        break;
                    }
                    for item in &setup_data {
                        tx.send(black_box(*item));
                    }
                    barrier.wait();
                }
            })
        };

        let consumer_handle = {
            let barrier = Arc::clone(&barrier);
            let stop_flag = Arc::clone(&stop_flag);

            thread::spawn(move || {
                let mut received_count = 0;
                let mut rx = SpscConsumer::new(consumer);
                loop {
                    barrier.wait();
                    if stop_flag.load(Acquire) {
                        break;
                    }
                    while received_count < BATCH_SIZE {
                        black_box(rx.recv());
                        received_count += 1;
                    }
                    barrier.wait();
                }
                // while !stop_flag.load(Acquire) {
                //     std::hint::spin_loop();
                // }
                // while received_count < BATCH_SIZE {
                //     black_box(rx.recv());
                //     received_count += 1;
                // }
            })
        };

        (producer_handle, consumer_handle)
    };

    spsc_ring_buffer_group.bench_function("push_peek_300k_messages", |b| {
        b.iter(|| {
            barrier.wait();
            barrier.wait();
        })
    });

    stop_flag.store(true, Release);
    barrier.wait();
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();

    // let routine = |producer_handle: JoinHandle<()>, consumer_handle: JoinHandle<()>| {
    //     FLAG_THREAD_START_ACTIVITY.store(true, Release);
    //     producer_handle.join().inspect_err(|e| error!("{e:?}")).ok();
    //     consumer_handle.join().inspect_err(|e| error!("{e:?}")).ok();
    //     FLAG_THREAD_START_ACTIVITY.store(false, Release);
    // };

    // spsc_ring_buffer_group.bench_function("push_peek_300k_messages", |b| {
    //     b.iter_batched(
    //         || setup(),
    //         |(producer_handle, consumer_handle)| routine(producer_handle, consumer_handle),
    //         criterion::BatchSize::SmallInput,
    //     )
    // });
}

criterion_group!(benches, bench_spsc_throughput);
criterion_main!(benches);
