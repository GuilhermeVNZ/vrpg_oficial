use rules5e_service::cr_xp::{ChallengeRating, xp_to_cr_approximate};

#[test]
fn test_cr_to_xp() {
    assert_eq!(ChallengeRating::Zero.to_xp(), 0);
    assert_eq!(ChallengeRating::OneEighth.to_xp(), 25);
    assert_eq!(ChallengeRating::OneQuarter.to_xp(), 50);
    assert_eq!(ChallengeRating::OneHalf.to_xp(), 100);
    assert_eq!(ChallengeRating::One.to_xp(), 200);
    assert_eq!(ChallengeRating::Five.to_xp(), 1_800);
    assert_eq!(ChallengeRating::Ten.to_xp(), 5_900);
    assert_eq!(ChallengeRating::Twenty.to_xp(), 25_000);
    assert_eq!(ChallengeRating::Thirty.to_xp(), 155_000);
}

#[test]
fn test_cr_to_proficiency_bonus() {
    assert_eq!(ChallengeRating::Zero.to_proficiency_bonus(), 2);
    assert_eq!(ChallengeRating::One.to_proficiency_bonus(), 2);
    assert_eq!(ChallengeRating::Five.to_proficiency_bonus(), 3);
    assert_eq!(ChallengeRating::Ten.to_proficiency_bonus(), 4);
    assert_eq!(ChallengeRating::Fifteen.to_proficiency_bonus(), 5);
    assert_eq!(ChallengeRating::Twenty.to_proficiency_bonus(), 6);
    assert_eq!(ChallengeRating::TwentyFive.to_proficiency_bonus(), 8);
    assert_eq!(ChallengeRating::Thirty.to_proficiency_bonus(), 8);
}

#[test]
fn test_cr_from_string() {
    assert_eq!(
        ChallengeRating::from_str("0").unwrap(),
        ChallengeRating::Zero
    );
    assert_eq!(
        ChallengeRating::from_str("1/8").unwrap(),
        ChallengeRating::OneEighth
    );
    assert_eq!(
        ChallengeRating::from_str("1/4").unwrap(),
        ChallengeRating::OneQuarter
    );
    assert_eq!(
        ChallengeRating::from_str("1/2").unwrap(),
        ChallengeRating::OneHalf
    );
    assert_eq!(
        ChallengeRating::from_str("1").unwrap(),
        ChallengeRating::One
    );
    assert_eq!(
        ChallengeRating::from_str("30").unwrap(),
        ChallengeRating::Thirty
    );
    assert!(ChallengeRating::from_str("invalid").is_err());
}

#[test]
fn test_xp_to_cr_approximate() {
    assert_eq!(xp_to_cr_approximate(0), ChallengeRating::Zero);
    assert_eq!(xp_to_cr_approximate(25), ChallengeRating::OneEighth);
    assert_eq!(xp_to_cr_approximate(200), ChallengeRating::One);
    assert_eq!(xp_to_cr_approximate(1_800), ChallengeRating::Five);
    assert_eq!(xp_to_cr_approximate(25_000), ChallengeRating::Twenty);
}



