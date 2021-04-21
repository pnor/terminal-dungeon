use float_cmp::{approx_eq, Ulps};
use splines::{Interpolation, Key, Spline};
use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, IconSplineConstructorError>;

/// Spline specifically for setting icons in a stepwise manner
struct IconSpline {
    pub spline: Spline<f32, char>,
}

impl IconSpline {
    /// Creates an IconSpline from a given HashMap
    ///
    /// `icon_range` is a Vector of tuples that contains each key for the spline.
    /// Each tuple has `(amount, character)` means that starting at `amount`, it should use `character`
    fn new(mut icon_ranges: Vec<(f32, char)>) -> Result<Self> {
        icon_ranges.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        if validate_ranges(&icon_ranges) {
            return Ok(IconSpline {
                spline: create_spline(icon_ranges),
            });
        } else {
            return Err(IconSplineConstructorError(icon_ranges));
        }
    }
}

/// Ensures that `icon_ranges` has entries between 0..1, and has one that starts at 0
fn validate_ranges(icon_ranges: &Vec<(f32, char)>) -> bool {
    if !approx_eq!(f32, icon_ranges[0].0, 0., epsilon = 2.0, ulps = 2) {
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

fn create_spline(icon_ranges: Vec<(f32, char)>) -> Spline<f32, char> {
    let mut keys = Vec::new();

    for (start, character) in icon_ranges {
        let key = Key::new(start, character, Interpolation::Step(1.0));
        keys.push(key);
    }
    keys.push(Key::new(1., '?', Interpolation::default()));

    Spline::from_vec(keys)
}

struct IconSplineConstructorError(Vec<(f32, char)>);

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
        let ranges = vec![(0.0, '@')];
        let spline = IconSpline::new(ranges);

        let res = match spline {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(res);

        // let more_complex_ranges = vec![
        //     (0.0, 'a'),
        //     (0.1, 'b'),
        //     (0.2, 'c'),
        //     (0.3, 'd'),
        //     (0.4, 'e'),
        //     (0.5, 'f'),
        //     (0.6, 'g'),
        //     (0.7, 'h'),
        //     (0.8, 'i'),
        //     (0.9, 'j'),
        // ];
        // let more_complex_spline = IconSpline::new(more_complex_ranges).unwrap();

        // let even_more_complicated_ranges = vec![
        //     (0.0, '@'),
        //     (0.1, '#'),
        //     (0.27, '@'),
        //     (0.89, '#'),
        //     (0.93, '@'),
        // ];
        // let even_more_complicated_spline = IconSpline::new(even_more_complicated_ranges).unwrap();
    }
}
