use crate::op;
use crate::strategy::engine::StrategyEngine;
use crate::strategy::Strategy;
use std::fs;

#[test]
fn test() {
    let wasm = fs::read("").unwrap(); // TODO
    let strategy = Strategy::from_wasm(wasm);
    let mut engine = StrategyEngine::new();
    engine
        .link_op("log", op::wrapped::log)
        .unwrap()
        .link_op("exit", op::wrapped::exit)
        .unwrap()
        .link_op("file-is-exist", op::wrapped::file::is_exist)
        .unwrap()
        .link_op("file-read", op::wrapped::file::read)
        .unwrap()
        .link_op("file-write", op::wrapped::file::write)
        .unwrap()
        .link_op("file-append", op::wrapped::file::append)
        .unwrap();
    /*
    let strategy = engine
        .precompile_strategy(strategy)
        .unwrap();
    */
    let logs = engine.run_strategy(strategy).unwrap();
    for log in logs {
        println!("{}", log)
    }
}
