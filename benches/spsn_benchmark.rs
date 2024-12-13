use std::{cell::RefCell, rc::Rc};

use a_sabr::{
    bundle::Bundle, contact_manager::seg::SegmentationManager,
    contact_plan::from_tvgutil_file::TVGUtilContactPlan, route_storage::cache::TreeCache,
    routing::aliases::*, types::NodeID,
};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

macro_rules! make_spsn_router {
    ($router_type:ident, $ptvg_filepath:expr) => {{
        let (nodes, contacts) =
            TVGUtilContactPlan::parse::<SegmentationManager>($ptvg_filepath).unwrap();

        let route_storage = Rc::new(RefCell::new(TreeCache::new(false, false, 10)));

        $router_type::new(nodes, contacts, route_storage, false)
    }};
}

macro_rules! blackbox_route_bundle {
    ($router:ident, $source:expr, $bundle:expr, $curr_time:expr, $excluded_nodes:expr) => {
        $router.route(
            black_box($source),
            black_box($bundle),
            black_box($curr_time),
            black_box($excluded_nodes),
        )
    };
}

pub fn spsn_mpt_benchmark(c: &mut Criterion) {
    let ptvg_filepath = "benches/ptvg_files/sample1.json";

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

    let mut group = c.benchmark_group("SpsnMpt");
    group.bench_function("SABR", |b| {
        b.iter_batched(
            || make_spsn_router!(SpsnMpt, ptvg_filepath),
            |mut router| {
                blackbox_route_bundle!(router, source, &bundle, curr_time, &excluded_nodes);
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("Hop", |b| {
        b.iter_batched(
            || make_spsn_router!(SpsnHopMpt, ptvg_filepath),
            |mut router| {
                blackbox_route_bundle!(router, source, &bundle, curr_time, &excluded_nodes);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name=benches;
    config=Criterion::default().sample_size(50);
    targets=spsn_mpt_benchmark
}
criterion_main!(benches);
