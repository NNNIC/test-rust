#[allow(dead_code)]
mod hoge2_control;
use hoge2_control::hoge2_control::*;

fn main() {
    let mut hc = Hoge2Control::new();
    hc.run();
}