use mempool_lab::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 75, slack: 34, drag: 20, confidence: 54 };
    assert_eq!(review_score(case), 178);
    assert_eq!(review_lane(case), "ship");
}
