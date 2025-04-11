#![no_main]

use postcard::from_bytes;
use cardinality_estimator::CardinalityEstimator;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(mut est) = from_bytes::<CardinalityEstimator<usize>>(
        data,
    ) {
        // crash happens *much* faster if we just do kinda anything with the estimator
        est.insert(&1);
        assert!(est.estimate() > 0);
    }
});
