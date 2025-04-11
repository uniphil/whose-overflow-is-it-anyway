# Rust buffer overflow from bincode2 (+serde) + cardinality-estimator

See [`./fuzz`](./fuzz) (and specifically [`fuzz_targets/bincode_serde_cardinality_estimate.rs`](./fuzz/fuzz_targets/bincode_serde_cardinality_estimate.rs)) for a small example with no unsafe that finds a buffer flow very quickly on my machine.

The specific dependencies can be verified in [`./fuzz/Cargo.toml`](./fuzz/Cargo.toml):

- cloudflare/**cardinality-estimator** `1.0.2` (features = ["with_serde"])
- **bincode** `2.0.1` (features = ["serde"])


The artifact is committed to the repo, so it might be possible to repro directly by running:

```
cargo +nightly fuzz tmin bincode_serde_cardinality_estimate fuzz/artifacts/bincode_serde_cardinality_estimate/crash-3c8f3f5345ec0e853bed074a3c63d074be14f814
```

Following is the output I see when I fuzz:

```
rust-whose-overflow-is-it-anyway » cargo +nightly fuzz run bincode_serde_cardinality_estimate fuzz/artifacts/bincode_serde_cardinality_estimate/crash-3c8f3f5345ec0e853bed074a3c63d074be14f814
    Finished `release` profile [optimized + debuginfo] target(s) in 0.01s
    Finished `release` profile [optimized + debuginfo] target(s) in 0.00s
     Running `fuzz/target/aarch64-apple-darwin/release/bincode_serde_cardinality_estimate -artifact_prefix=/Users/phil/code/rust-whose-overflow-is-it-anyway/fuzz/artifacts/bincode_serde_cardinality_estimate/ fuzz/artifacts/bincode_serde_cardinality_estimate/crash-3c8f3f5345ec0e853bed074a3c63d074be14f814`
bincode_serde_cardinality_estimate(11269,0x1fe860840) malloc: nano zone abandoned due to inability to reserve vm space.
INFO: Running with entropic power schedule (0xFF, 100).
INFO: Seed: 4178885261
INFO: Loaded 1 modules   (7202 inline 8-bit counters): 7202 [0x1046123a0, 0x104613fc2),
INFO: Loaded 1 PC tables (7202 PCs): 7202 [0x104613fc8,0x1046301e8),
fuzz/target/aarch64-apple-darwin/release/bincode_serde_cardinality_estimate: Running 1 inputs 1 time(s) each.
Running: fuzz/artifacts/bincode_serde_cardinality_estimate/crash-3c8f3f5345ec0e853bed074a3c63d074be14f814
=================================================================
==11269==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x6020000001df at pc 0x0001044857b8 bp 0x00016b981fc0 sp 0x00016b981fb8
READ of size 16 at 0x6020000001df thread T0
    #0 0x0001044857b4 in cardinality_estimator::array::Array$LT$_$C$_$GT$::insert::hd6c309dbb31209cb array.rs:44
    #1 0x00010447e968 in _$LT$cardinality_estimator..array..Array$LT$_$C$_$GT$$u20$as$u20$cardinality_estimator..representation..RepresentationTrait$GT$::insert_encoded_hash::h4595af115dd427a4 array.rs:94
    #2 0x00010448706c in cardinality_estimator::estimator::CardinalityEstimator$LT$T$C$H$C$_$C$_$GT$::insert::hc8cf099896922bcf estimator.rs:52
    #3 0x0001044a7610 in bincode_serde_cardinality_estimate::_::__libfuzzer_sys_run::hf4c6ae4628fcae63 bincode_serde_cardinality_estimate.rs:17
    #4 0x0001044a6ed4 in rust_fuzzer_test_input lib.rs:256
    #5 0x0001044a870c in std::panicking::try::do_call::hb9fff227eaec0f6b+0xc4 (bincode_serde_cardinality_estimate:arm64+0x10002c70c)
    #6 0x0001044ae780 in __rust_try+0x18 (bincode_serde_cardinality_estimate:arm64+0x100032780)
    #7 0x0001044ade0c in LLVMFuzzerTestOneInput+0x16c (bincode_serde_cardinality_estimate:arm64+0x100031e0c)
    #8 0x0001044b0e18 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long)+0x150 (bincode_serde_cardinality_estimate:arm64+0x100034e18)
    #9 0x0001044ce498 in fuzzer::RunOneTest(fuzzer::Fuzzer*, char const*, unsigned long)+0xe0 (bincode_serde_cardinality_estimate:arm64+0x100052498)
    #10 0x0001044d35fc in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long))+0x1d18 (bincode_serde_cardinality_estimate:arm64+0x1000575fc)
    #11 0x0001044e0ecc in main+0x24 (bincode_serde_cardinality_estimate:arm64+0x100064ecc)
    #12 0x000194b70270  (<unknown module>)
    #13 0xae5e7ffffffffffc  (<unknown module>)

0x6020000001df is located 3 bytes after 12-byte region [0x6020000001d0,0x6020000001dc)
allocated by thread T0 here:
    #0 0x000104bf28c0 in malloc+0x6c (librustc-nightly_rt.asan.dylib:arm64+0x528c0)
    #1 0x000104480eb4 in _$LT$serde..de..impls..$LT$impl$u20$serde..de..Deserialize$u20$for$u20$alloc..vec..Vec$LT$T$GT$$GT$..deserialize..VecVisitor$LT$T$GT$$u20$as$u20$serde..de..Visitor$GT$::visit_seq::h4810ea585556fba0 impls.rs:1173
    #2 0x00010447db98 in _$LT$bincode..features..serde..de_borrowed..SerdeDecoder$LT$DE$GT$$u20$as$u20$serde..de..Deserializer$GT$::deserialize_option::h8732f30f46fcfdb4 de_borrowed.rs:253
    #3 0x000104486424 in cardinality_estimator::serde::_$LT$impl$u20$serde..de..Deserialize$u20$for$u20$cardinality_estimator..estimator..CardinalityEstimator$LT$T$C$H$C$_$C$_$GT$$GT$::deserialize::haba608247f9a7ec5 serde.rs:77
    #4 0x0001044a75a4 in bincode_serde_cardinality_estimate::_::__libfuzzer_sys_run::hf4c6ae4628fcae63 bincode_serde_cardinality_estimate.rs:12
    #5 0x0001044a6ed4 in rust_fuzzer_test_input lib.rs:256
    #6 0x0001044a870c in std::panicking::try::do_call::hb9fff227eaec0f6b+0xc4 (bincode_serde_cardinality_estimate:arm64+0x10002c70c)
    #7 0x0001044ae780 in __rust_try+0x18 (bincode_serde_cardinality_estimate:arm64+0x100032780)
    #8 0x0001044ade0c in LLVMFuzzerTestOneInput+0x16c (bincode_serde_cardinality_estimate:arm64+0x100031e0c)
    #9 0x0001044b0e18 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long)+0x150 (bincode_serde_cardinality_estimate:arm64+0x100034e18)
    #10 0x0001044ce498 in fuzzer::RunOneTest(fuzzer::Fuzzer*, char const*, unsigned long)+0xe0 (bincode_serde_cardinality_estimate:arm64+0x100052498)
    #11 0x0001044d35fc in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long))+0x1d18 (bincode_serde_cardinality_estimate:arm64+0x1000575fc)
    #12 0x0001044e0ecc in main+0x24 (bincode_serde_cardinality_estimate:arm64+0x100064ecc)
    #13 0x000194b70270  (<unknown module>)
    #14 0xae5e7ffffffffffc  (<unknown module>)

SUMMARY: AddressSanitizer: heap-buffer-overflow array.rs:44 in cardinality_estimator::array::Array$LT$_$C$_$GT$::insert::hd6c309dbb31209cb
Shadow bytes around the buggy address:
  0x601fffffff00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x601fffffff80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x602000000000: fa fa fd fa fa fa fd fd fa fa fd fd fa fa 00 04
  0x602000000080: fa fa 00 07 fa fa 00 04 fa fa 00 00 fa fa 00 00
  0x602000000100: fa fa 00 00 fa fa 00 fa fa fa 00 fa fa fa 00 fa
=>0x602000000180: fa fa 06 fa fa fa 06 fa fa fa 00[04]fa fa fa fa
  0x602000000200: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x602000000280: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x602000000300: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x602000000380: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x602000000400: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==11269==ABORTING
────────────────────────────────────────────────────────────────────────────────

Error: Fuzz target exited with signal: 6 (SIGABRT)
```
