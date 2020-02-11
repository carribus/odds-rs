#[derive(Debug, Copy, Clone, PartialEq)]
enum Odds {
    Decimal(f32),
    Fractional(f32, f32),
    American(f32),
}

impl Odds {
    fn to_decimal(&self) -> Self {
        let c = OddsCalculator::new();
        let p = c.probability_from_odds(self.clone());

        Odds::Decimal(1.0 / p)
    }

    fn to_fractional(&self) -> Self {
        // TODO: Implement algo to convert to a more human readable fraction (using integers only)
        let c = OddsCalculator::new();
        let p = c.probability_from_odds(self.clone());

        Odds::Fractional(1.0 / p - 1.0, 1.0)
    }

    fn to_american(&self) -> Self {
        let c = OddsCalculator::new();
        let p = c.probability_from_odds(self.clone());

        if p > 0.5 {
            Odds::American(-(p / (1.0 - p) * 100.0).round())
        } else {
            Odds::American(((1.0 - p) / p * 100.0).round())
        }
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
                    (odds.abs() / (odds.abs() + 100.0))
                } else {
                    (100.0 / (odds + 100.0))
                }
            },
        }
    }

    fn expected_return(&self, stake: f32, odds: Odds) -> f32 {
        match odds {
            Odds::Decimal(odds) => stake * odds,
            Odds::Fractional(n, d) => stake + stake * n / d,
            Odds::American(odds) => {
                if odds > 0.0 {
                    stake + odds * (stake / 100.0)
                } else {
                    stake - (100.0 / odds) * stake
                }
            },
        }
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
        assert_eq!(calc.odds_from_probability(0.2857143), Odds::Decimal(3.4999998));
        assert_eq!(calc.odds_from_probability(0.35714287), Odds::Decimal(2.8));
        assert_eq!(calc.odds_from_probability(0.54545456), Odds::Decimal(1.8333333));
    }

    #[test]
    fn probability_from_odds() {
        let calc = OddsCalculator::new();

        assert_eq!(calc.probability_from_odds(Odds::Decimal(1.65)), 0.6060606);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(1.36)), 0.7352941);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(1.0)), 1.0);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(5.0)), 0.2);
        assert_eq!(calc.probability_from_odds(Odds::Decimal(3.4999998)), 0.2857143);

        assert_eq!(calc.probability_from_odds(calc.odds_from_probability(0.47)), 0.47);
        assert_eq!(calc.probability_from_odds(calc.odds_from_probability(0.57)), 0.57);
        assert_eq!(calc.probability_from_odds(calc.odds_from_probability(0.67)), 0.67);

        assert_eq!(calc.probability_from_odds(Odds::Fractional(5.0, 2.0)), 0.2857143);

        assert_eq!(calc.probability_from_odds(Odds::American(-120.0)), 0.54545456);
        assert_eq!(calc.probability_from_odds(Odds::American(180.0)), 0.35714287);
    }

    #[test]
    fn convert_to_decimal() {
        let c = OddsCalculator::new();

        assert_eq!(Odds::Fractional(1.0, 1.0).to_decimal(), Odds::Decimal(2.0));
        assert_eq!(Odds::Fractional(2.0, 1.0).to_decimal(), Odds::Decimal(3.0));
        assert_eq!(Odds::Fractional(3.0, 1.0).to_decimal(), Odds::Decimal(4.0));
        assert_eq!(Odds::Fractional(1.0, 4.0).to_decimal(), Odds::Decimal(1.25));
        assert_eq!(Odds::Fractional(1.0, 3.0).to_decimal(), Odds::Decimal(1.3333334));

        assert_eq!(Odds::American(-120.0).to_decimal(), Odds::Decimal(1.8333333));
        assert_eq!(Odds::American(-100.0).to_decimal(), Odds::Decimal(2.0));
        assert_eq!(Odds::American(100.0).to_decimal(), Odds::Decimal(2.0));
        assert_eq!(Odds::American(150.0).to_decimal(), Odds::Decimal(2.5));
    }

    #[test]
    fn convert_to_fractional() {
        let c = OddsCalculator::new();

        assert_eq!(Odds::Decimal(3.4999998).to_fractional(), Odds::Fractional(2.4999998, 1.0));
    }

    #[test]
    fn convert_to_american() {
        let c = OddsCalculator::new();

        assert_eq!(c.odds_from_probability(0.25).to_american(), Odds::American(300.0));
        assert_eq!(c.odds_from_probability(0.75).to_american(), Odds::American(-300.0));

        assert_eq!(Odds::Decimal(1.8333333).to_american(), Odds::American(-120.0));
        assert_eq!(Odds::Decimal(2.8333333).to_american(), Odds::American(183.0));
        assert_eq!(Odds::Decimal(2.8).to_american(), Odds::American(180.0));
    }

    #[test]
    fn test_return_decimal() {
        let c = OddsCalculator::new();

        assert_eq!(c.expected_return(10.0, Odds::Decimal(2.0)), 20.0);
        assert_eq!(c.expected_return(10.0, Odds::Decimal(3.5)), 35.0);
        assert_eq!(c.expected_return(9.50, Odds::Decimal(3.5)), 33.25);
    }

    #[test]
    fn test_return_fractional() {
        let c = OddsCalculator::new();

        assert_eq!(c.expected_return(10.0, Odds::Fractional(1.0, 1.0)), 20.0);
        assert_eq!(c.expected_return(10.0, Odds::Fractional(2.0, 3.0)), 16.666666);
    }

    #[test]
    fn test_return_american() {
        let c = OddsCalculator::new();

        assert_eq!(c.expected_return(10.0, Odds::American(120.0)), 22.0);
        assert_eq!(c.expected_return(10.0, Odds::American(-120.0)), 18.333332);
        assert_eq!(c.expected_return(15.0, Odds::American(119.0)), 32.85);
        assert_eq!(c.expected_return(13.75, Odds::American(-129.0)), 24.408915);
    }
}
