#![no_main]
use kata_agent::random::reseed_rng;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    let _ = reseed_rng(bytes);
});
