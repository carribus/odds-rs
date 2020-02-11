#[derive(Debug, Copy, Clone, PartialEq)]
enum Odds {
    Decimal(f32),
    Fractional(f32, f32),
    American(f32),
}

impl Odds {
    fn to_decimal(&self) -> Self {
        Odds::Decimal(0.0)
    }

    fn to_fractional(&self) -> Self {
        Odds::Fractional(0.0, 0.0)
    }

    fn to_american(&self) -> Self {
        Odds::American(0.0)
    }
}

struct OddsCalculator;

impl OddsCalculator {
    fn new() -> Self {
        OddsCalculator
    }

    fn odds_from_probability(&self, chance: f32) -> Odds {
        Odds::Decimal(1.0 / chance)
    }

    fn probability_from_odds(&self, odds: Odds) -> f32 {
        match odds {
            Odds::Decimal(odds) => 1.0 / odds,
            Odds::Fractional(n, d) => d/(d+n),
            Odds::American(odds) => {
                if odds < 0.0 {
                    odds.abs() / (odds.abs() + 100.0)
                } else {
                    100.0 / (odds + 100.0)
                }
            },
        }
    }

    fn expected_return(stake: f32, odds: Odds) -> f32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odds_from_probability() {
        let calc = OddsCalculator::new();

        assert_eq!(calc.odds_from_probability(0.5), Odds::Decimal(2.0));
        assert_eq!(calc.odds_from_probability(0.74), Odds::Decimal(1.3513514));
        assert_eq!(calc.odds_from_probability(0.57), Odds::Decimal(1.754386));
        assert_eq!(calc.odds_from_probability(1.0), Odds::Decimal(1.0));
        assert_eq!(calc.odds_from_probability(0.1), Odds::Decimal(10.0));
    }

    #[test]
    fn probability_from_odds() {
        let calc = OddsCalculator::new();

        assert_eq!(calc.probability_from_odds(Odds::Decimal(1.65)), 0.6060606);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(1.36)), 0.7352941);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(1.0)), 1.0);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(5.0)), 0.2);

        assert_eq!(calc.probability_from_odds(calc.odds_from_probability(0.47)), 0.47);
        assert_eq!(calc.probability_from_odds(calc.odds_from_probability(0.57)), 0.57);
        assert_eq!(calc.probability_from_odds(calc.odds_from_probability(0.67)), 0.67);

        assert_eq!(calc.probability_from_odds(Odds::Fractional(5.0, 2.0)), 0.2857143);

        assert_eq!(calc.probability_from_odds(Odds::American(-120.0)), 0.54545456);
        assert_eq!(calc.probability_from_odds(Odds::American(180.0)), 0.35714287);
    }
}
