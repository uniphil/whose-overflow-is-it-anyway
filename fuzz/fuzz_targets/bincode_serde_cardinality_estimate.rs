#![no_main]

use bincode::config::{Configuration, BigEndian, Fixint, Limit, standard};
use bincode::serde::decode_from_slice;
use cardinality_estimator::CardinalityEstimator;
use libfuzzer_sys::fuzz_target;

type C = Configuration<BigEndian, Fixint, Limit<1048576>>;
static BINCODE_CONF: C = standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .with_limit::<1048576>();

fuzz_target!(|data: &[u8]| {
    if let Ok((mut est, _n)) = decode_from_slice::<CardinalityEstimator<usize>, C>(
        data,
        BINCODE_CONF,
    ) {
        // crash happens *much* faster if we just do kinda anything with the estimator
        est.insert(&1);
        assert!(est.estimate() > 0);
    }
});
