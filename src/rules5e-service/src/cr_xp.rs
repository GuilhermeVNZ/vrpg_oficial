// Challenge Rating and XP System - D&D 5e
// Complete CR to XP conversion table from Monster Manual

use crate::error::{Result, RulesError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ChallengeRating {
    Zero,
    OneEighth,
    OneQuarter,
    OneHalf,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
}

impl ChallengeRating {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "0" => Ok(Self::Zero),
            "1/8" => Ok(Self::OneEighth),
            "1/4" => Ok(Self::OneQuarter),
            "1/2" => Ok(Self::OneHalf),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "10" => Ok(Self::Ten),
            "11" => Ok(Self::Eleven),
            "12" => Ok(Self::Twelve),
            "13" => Ok(Self::Thirteen),
            "14" => Ok(Self::Fourteen),
            "15" => Ok(Self::Fifteen),
            "16" => Ok(Self::Sixteen),
            "17" => Ok(Self::Seventeen),
            "18" => Ok(Self::Eighteen),
            "19" => Ok(Self::Nineteen),
            "20" => Ok(Self::Twenty),
            "21" => Ok(Self::TwentyOne),
            "22" => Ok(Self::TwentyTwo),
            "23" => Ok(Self::TwentyThree),
            "24" => Ok(Self::TwentyFour),
            "25" => Ok(Self::TwentyFive),
            "26" => Ok(Self::TwentySix),
            "27" => Ok(Self::TwentySeven),
            "28" => Ok(Self::TwentyEight),
            "29" => Ok(Self::TwentyNine),
            "30" => Ok(Self::Thirty),
            _ => Err(RulesError::InvalidInput(format!("Invalid CR: {}", s))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Zero => "0".to_string(),
            Self::OneEighth => "1/8".to_string(),
            Self::OneQuarter => "1/4".to_string(),
            Self::OneHalf => "1/2".to_string(),
            Self::One => "1".to_string(),
            Self::Two => "2".to_string(),
            Self::Three => "3".to_string(),
            Self::Four => "4".to_string(),
            Self::Five => "5".to_string(),
            Self::Six => "6".to_string(),
            Self::Seven => "7".to_string(),
            Self::Eight => "8".to_string(),
            Self::Nine => "9".to_string(),
            Self::Ten => "10".to_string(),
            Self::Eleven => "11".to_string(),
            Self::Twelve => "12".to_string(),
            Self::Thirteen => "13".to_string(),
            Self::Fourteen => "14".to_string(),
            Self::Fifteen => "15".to_string(),
            Self::Sixteen => "16".to_string(),
            Self::Seventeen => "17".to_string(),
            Self::Eighteen => "18".to_string(),
            Self::Nineteen => "19".to_string(),
            Self::Twenty => "20".to_string(),
            Self::TwentyOne => "21".to_string(),
            Self::TwentyTwo => "22".to_string(),
            Self::TwentyThree => "23".to_string(),
            Self::TwentyFour => "24".to_string(),
            Self::TwentyFive => "25".to_string(),
            Self::TwentySix => "26".to_string(),
            Self::TwentySeven => "27".to_string(),
            Self::TwentyEight => "28".to_string(),
            Self::TwentyNine => "29".to_string(),
            Self::Thirty => "30".to_string(),
        }
    }

    pub fn to_xp(&self) -> u32 {
        match self {
            Self::Zero => 0,
            Self::OneEighth => 25,
            Self::OneQuarter => 50,
            Self::OneHalf => 100,
            Self::One => 200,
            Self::Two => 450,
            Self::Three => 700,
            Self::Four => 1_100,
            Self::Five => 1_800,
            Self::Six => 2_300,
            Self::Seven => 2_900,
            Self::Eight => 3_900,
            Self::Nine => 5_000,
            Self::Ten => 5_900,
            Self::Eleven => 7_200,
            Self::Twelve => 8_400,
            Self::Thirteen => 10_000,
            Self::Fourteen => 11_500,
            Self::Fifteen => 13_000,
            Self::Sixteen => 15_000,
            Self::Seventeen => 18_000,
            Self::Eighteen => 20_000,
            Self::Nineteen => 22_000,
            Self::Twenty => 25_000,
            Self::TwentyOne => 33_000,
            Self::TwentyTwo => 41_000,
            Self::TwentyThree => 50_000,
            Self::TwentyFour => 62_000,
            Self::TwentyFive => 75_000,
            Self::TwentySix => 90_000,
            Self::TwentySeven => 105_000,
            Self::TwentyEight => 120_000,
            Self::TwentyNine => 135_000,
            Self::Thirty => 155_000,
        }
    }

    pub fn to_proficiency_bonus(&self) -> i32 {
        match self {
            Self::Zero | Self::OneEighth | Self::OneQuarter => 2,
            Self::OneHalf | Self::One => 2,
            Self::Two | Self::Three | Self::Four => 2,
            Self::Five | Self::Six | Self::Seven | Self::Eight => 3,
            Self::Nine | Self::Ten | Self::Eleven | Self::Twelve => 4,
            Self::Thirteen | Self::Fourteen | Self::Fifteen | Self::Sixteen => 5,
            Self::Seventeen | Self::Eighteen | Self::Nineteen | Self::Twenty => 6,
            Self::TwentyOne | Self::TwentyTwo | Self::TwentyThree | Self::TwentyFour => 7,
            Self::TwentyFive | Self::TwentySix | Self::TwentySeven | Self::TwentyEight => 8,
            Self::TwentyNine | Self::Thirty => 8,
        }
    }
}

pub fn xp_to_cr_approximate(xp: u32) -> ChallengeRating {
    // Find closest CR based on XP
    let crs = [
        ChallengeRating::Zero,
        ChallengeRating::OneEighth,
        ChallengeRating::OneQuarter,
        ChallengeRating::OneHalf,
        ChallengeRating::One,
        ChallengeRating::Two,
        ChallengeRating::Three,
        ChallengeRating::Four,
        ChallengeRating::Five,
        ChallengeRating::Six,
        ChallengeRating::Seven,
        ChallengeRating::Eight,
        ChallengeRating::Nine,
        ChallengeRating::Ten,
        ChallengeRating::Eleven,
        ChallengeRating::Twelve,
        ChallengeRating::Thirteen,
        ChallengeRating::Fourteen,
        ChallengeRating::Fifteen,
        ChallengeRating::Sixteen,
        ChallengeRating::Seventeen,
        ChallengeRating::Eighteen,
        ChallengeRating::Nineteen,
        ChallengeRating::Twenty,
        ChallengeRating::TwentyOne,
        ChallengeRating::TwentyTwo,
        ChallengeRating::TwentyThree,
        ChallengeRating::TwentyFour,
        ChallengeRating::TwentyFive,
        ChallengeRating::TwentySix,
        ChallengeRating::TwentySeven,
        ChallengeRating::TwentyEight,
        ChallengeRating::TwentyNine,
        ChallengeRating::Thirty,
    ];

    let mut closest = ChallengeRating::Zero;
    let mut min_diff = u32::MAX;

    for cr in &crs {
        let cr_xp = cr.to_xp();
        let diff = if xp >= cr_xp { xp - cr_xp } else { cr_xp - xp };
        if diff < min_diff {
            min_diff = diff;
            closest = *cr;
        }
    }

    closest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cr_to_xp() {
        assert_eq!(ChallengeRating::Zero.to_xp(), 0);
        assert_eq!(ChallengeRating::OneEighth.to_xp(), 25);
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
    fn test_xp_to_cr_approximate() {
        assert_eq!(xp_to_cr_approximate(0), ChallengeRating::Zero);
        assert_eq!(xp_to_cr_approximate(25), ChallengeRating::OneEighth);
        assert_eq!(xp_to_cr_approximate(200), ChallengeRating::One);
        assert_eq!(xp_to_cr_approximate(1_800), ChallengeRating::Five);
        assert_eq!(xp_to_cr_approximate(25_000), ChallengeRating::Twenty);
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
            ChallengeRating::from_str("1").unwrap(),
            ChallengeRating::One
        );
        assert_eq!(
            ChallengeRating::from_str("30").unwrap(),
            ChallengeRating::Thirty
        );
        assert!(ChallengeRating::from_str("invalid").is_err());
    }
}
