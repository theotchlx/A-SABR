use core::panic;
use std::{cell::RefCell, rc::Rc, time::Duration};

use a_sabr::{
    bundle::Bundle, contact_manager::seg::SegmentationManager,
    contact_plan::from_tvgutil_file::TVGUtilContactPlan, route_storage::cache::TreeCache,
    routing::aliases::*, types::NodeID,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn spsn_mpt_benchmark(c: &mut Criterion) {
    let contact_plan =
        TVGUtilContactPlan::parse::<SegmentationManager>("benches/ptvg_files/sample1.json");

    match contact_plan {
        Ok((nodes, contacts)) => {
            let route_storage = Rc::new(RefCell::new(TreeCache::new(false, false, 10)));
            let mut router = SpsnMpt::new(nodes, contacts, route_storage, false);

            let source = 178;
            let bundle = Bundle {
                source: 178,
                destinations: vec![159],
                priority: 0,
                size: 47419533.0,
                expiration: 24060.0,
            };
            let curr_time = 60.0;
            let excluded_nodes: Vec<NodeID> = vec![];

            c.bench_function("route", |b| {
                b.iter(|| {
                    router.route(
                        black_box(source),
                        black_box(&bundle),
                        black_box(curr_time),
                        black_box(&excluded_nodes),
                    )
                })
            });
        }
        Err(err) => panic!("Unable to load contact plan: {}", err),
    }
}

criterion_group! {
    name=benches;
    config=Criterion::default().measurement_time(Duration::from_secs(10));
    targets=spsn_mpt_benchmark
}
criterion_main!(benches);
