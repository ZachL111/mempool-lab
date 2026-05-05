use mempool_lab::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 76, capacity: 107, latency: 24, risk: 22, weight: 6 };
    assert_eq!(score(signal), 113);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 99, capacity: 71, latency: 24, risk: 21, weight: 13 };
    assert_eq!(score(signal), 170);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 82, capacity: 78, latency: 15, risk: 8, weight: 8 };
    assert_eq!(score(signal), 205);
    assert_eq!(classify(signal), "accept");
}
