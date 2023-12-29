use std::fs;
use crate::strategy::Strategy;
use crate::test::{compile_strategy, gen_engine};

#[test]
fn test_exit() {
    let bin_path = compile_strategy("../test-resources/strategies/exit");
    let wasm = fs::read(bin_path).unwrap();
    let strategy = unsafe { Strategy::from_precompiled(wasm) };
    let engine = gen_engine();

    let (logs, r) = engine.run_strategy(strategy);
    assert!(r.is_err());
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "0");
}

#[test]
fn test_log() {
    let bin_path = compile_strategy("../test-resources/strategies/log");
    let wasm = fs::read(bin_path).unwrap();
    let strategy = unsafe { Strategy::from_precompiled(wasm) };
    let engine = gen_engine();

    let (logs, r) = engine.run_strategy(strategy);
    assert!(r.is_ok());
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0], "0");
    assert_eq!(logs[1], "1");
    assert_eq!(logs[2], "2");
}

#[test]
fn test_panic() {
    let bin_path = compile_strategy("../test-resources/strategies/panic");
    let wasm = fs::read(bin_path).unwrap();
    let strategy = unsafe { Strategy::from_precompiled(wasm) };
    let engine = gen_engine();

    let (logs, r) = engine.run_strategy(strategy);
    assert!(r.is_err());
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "0");
    assert_eq!(logs[1], "strategy panic: panicked at src/lib.rs:6:1:\noops");
}
