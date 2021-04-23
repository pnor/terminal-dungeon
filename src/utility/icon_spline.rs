use float_cmp::approx_eq;
use splines::{Interpolation, Key, Spline};
use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, IconSplineConstructorError>;

/// Spline specifically for setting icons in a stepwise manner
///
/// Uses `f32` under the hood to interpolate, then casts result to `u8`, then to `char`
struct IconSpline {
    spline: Spline<f32, f32>,
}

impl IconSpline {
    /// Creates an IconSpline from a given HashMap
    ///
    /// `icon_range` is a Vector of tuples that contains each key for the spline.
    /// Each tuple has `(amount, character)` means that starting at `amount`, it should use `character`
    fn new(mut icon_ranges: Vec<(f32, char)>) -> Result<Self> {
        icon_ranges.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        let float_ranges = icon_ranges.into_iter().map(|(s, c)| (s, c as u8 as f32)).collect();

        if validate_ranges(&float_ranges) {
            return Ok(IconSpline {
                spline: create_spline(float_ranges),
            });
        } else {
            return Err(IconSplineConstructorError(float_ranges));
        }
    }

    fn sample(&self, point: f32) -> Option<char> {
        self.spline.sample(point).and_then(|p| Some(p as u8 as char))
    }

    fn clamped_sample(&self, point: f32) -> Option<char> {
        self.spline.clamped_sample(point).and_then(|p| Some(p as u8 as char))
    }
}

/// Ensures that `icon_ranges` has entries between 0..1, and has one that starts at 0
fn validate_ranges(icon_ranges: &Vec<(f32, f32)>) -> bool {
    if icon_ranges.len() == 0 {
        return false;
    }

    if !approx_eq!(f32, icon_ranges[0].0, 0., epsilon = 0.0001) {
        return false;
    }

    if icon_ranges
        .iter()
        .any(|&(start, _)| start > 1. && start < 0.)
    {
        return false;
    }

    true
}

fn create_spline(icon_ranges: Vec<(f32, f32)>) -> Spline<f32, f32> {
    let mut keys = Vec::new();
    let last_character = match icon_ranges.last() {
        Some(&(_, value)) => value,
        None => '?' as u8 as f32
    };

    for (start, character) in icon_ranges {
        let key = Key::new(start, character, Interpolation::Step(1.0));
        keys.push(key);
    }

    keys.push(Key::new(1., last_character, Interpolation::default()));

    Spline::from_vec(keys)
}

struct IconSplineConstructorError(Vec<(f32, f32)>);

impl Error for IconSplineConstructorError {}

impl fmt::Display for IconSplineConstructorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to construct IconSpline due to issue in icon ranges: {:?}",
            self.0
        )
    }
}

impl fmt::Debug for IconSplineConstructorError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_good_new() {
        let spline = IconSpline::new(vec![(0.0, '@')]);
        assert!(spline.is_ok());

        let more_complex_spline = IconSpline::new(vec![
            (0.0, 'a'),
            (0.1, 'b'),
            (0.2, 'c'),
            (0.3, 'd'),
            (0.4, 'e'),
            (0.5, 'f'),
            (0.6, 'g'),
            (0.7, 'h'),
            (0.8, 'i'),
            (0.9, 'j'),
        ]);
        assert!(more_complex_spline.is_ok());

        let even_more_complicated_spline = IconSpline::new(vec![
            (0.0, '@'),
            (0.1, '#'),
            (0.27, '@'),
            (0.89, '#'),
            (0.93, '@'),
        ]);
        assert!(even_more_complicated_spline.is_ok());
    }

    #[test]
    fn test_bad_new() {
        let empty_spline = IconSpline::new(vec![]);
        assert_eq!(empty_spline.is_ok(), false);

        let no_beginning_spline = IconSpline::new(vec![(0.5, '?'), (0.8, '!')]);
        assert_eq!(no_beginning_spline.is_ok(), false);
    }

    #[test]
    fn test_applying_spline() {
        let spline = IconSpline::new(vec![
            (0.0, '0'),
            (0.1, '1'),
            (0.2, '2'),
            (0.3, '3'),
            (0.4, '4'),
            (0.5, '5'),
            (0.6, '6'),
            (0.7, '7'),
            (0.8, '8'),
            (0.9, '9'),
        ])
        .expect("Test uses valid spline");

        assert_eq!(spline.sample(0.00), Some('0'));
        assert_eq!(spline.sample(0.05), Some('0'));
        assert_eq!(spline.sample(0.10), Some('1'));
        assert_eq!(spline.sample(0.15), Some('1'));
        assert_eq!(spline.sample(0.20), Some('2'));
        assert_eq!(spline.sample(0.25), Some('2'));
        assert_eq!(spline.sample(0.30), Some('3'));
        assert_eq!(spline.sample(0.35), Some('3'));
        assert_eq!(spline.sample(0.40), Some('4'));
        assert_eq!(spline.sample(0.45), Some('4'));
        assert_eq!(spline.sample(0.50), Some('5'));
        assert_eq!(spline.sample(0.55), Some('5'));
        assert_eq!(spline.sample(0.60), Some('6'));
        assert_eq!(spline.sample(0.65), Some('6'));
        assert_eq!(spline.sample(0.70), Some('7'));
        assert_eq!(spline.sample(0.75), Some('7'));
        assert_eq!(spline.sample(0.80), Some('8'));
        assert_eq!(spline.sample(0.85), Some('8'));
        assert_eq!(spline.sample(0.90), Some('9'));
        assert_eq!(spline.sample(0.95), Some('9'));
        assert_eq!(spline.sample(1.00), None);
    }

    fn test_applying_clamped_splines() {
        let spline = IconSpline::new(vec![
            (0.0, '0'),
            (0.1, '1'),
            (0.2, '2'),
            (0.3, '3'),
            (0.4, '4'),
            (0.5, '5'),
            (0.6, '6'),
            (0.7, '7'),
            (0.8, '8'),
            (0.9, '9'),
        ])
        .expect("Test uses valid spline");

        assert_eq!(spline.clamped_sample(-3.0), Some('0'));
        assert_eq!(spline.clamped_sample(-2.0), Some('0'));
        assert_eq!(spline.clamped_sample(-1.0), Some('0'));
        assert_eq!(spline.clamped_sample(0.00), Some('0'));
        assert_eq!(spline.clamped_sample(0.05), Some('0'));
        assert_eq!(spline.clamped_sample(0.10), Some('1'));
        assert_eq!(spline.clamped_sample(0.15), Some('1'));
        assert_eq!(spline.clamped_sample(0.20), Some('2'));
        assert_eq!(spline.clamped_sample(0.25), Some('2'));
        assert_eq!(spline.clamped_sample(0.30), Some('3'));
        assert_eq!(spline.clamped_sample(0.35), Some('3'));
        assert_eq!(spline.clamped_sample(0.40), Some('4'));
        assert_eq!(spline.clamped_sample(0.45), Some('4'));
        assert_eq!(spline.clamped_sample(0.50), Some('5'));
        assert_eq!(spline.clamped_sample(0.55), Some('5'));
        assert_eq!(spline.clamped_sample(0.60), Some('6'));
        assert_eq!(spline.clamped_sample(0.65), Some('6'));
        assert_eq!(spline.clamped_sample(0.70), Some('7'));
        assert_eq!(spline.clamped_sample(0.75), Some('7'));
        assert_eq!(spline.clamped_sample(0.80), Some('8'));
        assert_eq!(spline.clamped_sample(0.85), Some('8'));
        assert_eq!(spline.clamped_sample(0.90), Some('9'));
        assert_eq!(spline.clamped_sample(0.95), Some('9'));
        assert_eq!(spline.clamped_sample(1.00), Some('9'));
        assert_eq!(spline.clamped_sample(1.10), Some('9'));
        assert_eq!(spline.clamped_sample(1.23), Some('9'));
        assert_eq!(spline.clamped_sample(1.98), Some('9'));
        assert_eq!(spline.clamped_sample(2.00), Some('9'));
    }
}
