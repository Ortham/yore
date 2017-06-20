use std::fmt;

use chrono::Duration;

#[derive(Debug, PartialEq)]
pub struct SuggestionAccuracy {
    pub space: u16,
    pub time: i64,
}

impl SuggestionAccuracy {
    fn pretty_print_time(&self) -> String {
        if self.time == 0 {
            return "0 seconds".to_string();
        }

        let duration = Duration::seconds(self.time);

        let mut periods: Vec<String> = Vec::new();
        if duration.num_weeks() != 0 {
            periods.push(print_period(duration.num_weeks(), "week", "weeks"));
        }
        if should_print_period(duration.num_days(), 7) {
            periods.push(print_period(
                duration.num_days().wrapping_rem(7),
                "day",
                "days",
            ));
        }
        if should_print_period(duration.num_hours(), 24) {
            periods.push(print_period(
                duration.num_hours().wrapping_rem(24),
                "hour",
                "hours",
            ));
        }
        if should_print_period(duration.num_minutes(), 60) {
            periods.push(print_period(
                duration.num_minutes().wrapping_rem(60),
                "minute",
                "minutes",
            ));
        }
        if should_print_period(duration.num_seconds(), 60) {
            periods.push(print_period(
                duration.num_seconds().wrapping_rem(60),
                "second",
                "seconds",
            ));
        }

        return periods.join(", ");
    }
}

impl fmt::Display for SuggestionAccuracy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} metres, {}", self.space, self.pretty_print_time())
    }
}

fn should_print_period(period: i64, max: u8) -> bool {
    period != 0 && period.wrapping_rem(max as i64) != 0
}

fn print_period(period: i64, singular: &str, plural: &str) -> String {
    if period == 1 {
        format!("{} {}", period, singular)
    } else {
        format!("{} {}", period, plural)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestion_accuracy_display_should_format_value_correctly() {
        let accuracy = SuggestionAccuracy { space: 18, time: 0 };

        assert_eq!("18 metres, 0 seconds", format!("{}", accuracy));

        let accuracy = SuggestionAccuracy {
            space: 18,
            time: 3600,
        };

        assert_eq!("18 metres, 1 hour", format!("{}", accuracy));

        let accuracy = SuggestionAccuracy {
            space: 18,
            time: 90,
        };

        assert_eq!("18 metres, 1 minute, 30 seconds", format!("{}", accuracy));

        let accuracy = SuggestionAccuracy {
            space: 18,
            time: 20499642,
        };

        assert_eq!(
            "18 metres, 33 weeks, 6 days, 6 hours, 20 minutes, 42 seconds",
            format!("{}", accuracy)
        );
    }
}
