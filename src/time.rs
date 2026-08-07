//! [TimeAdInfinitum]
//! - "TAIm" /teɪm/
//! 
//! …you get the gist.
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum TimeAdInfinitumNotify {
    /// Not an error - just a notification.
    Step0Increased,
    /// Not an error - just a notification about time reversal.
    Step0Decreased,
    FlooredToZeroTime,
    BeyondTimeAndSpace,
}

impl Display for TimeAdInfinitumNotify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Step0Decreased => "step0 decreased",
            Self::Step0Increased => "step0 increased",
            Self::FlooredToZeroTime => "floored to zero-time",
            Self::BeyondTimeAndSpace => "presented amount of time exceeds RAM limits; sorry Dave, cannot let you do that"
        })
    }
}

/// Scale agnostic timey-wimey.
/// 
/// What scale of time `short` expresses is up to you really, but
/// treating it as 'seconds' is kinds of convenient.
/// 
pub struct TimeAdInfinitum {
    short: f64,
    step0: Vec<u64>,
}

const SAFE_THRESHOLD: f64 = 1e9;
const SAFE_THRESHOLD_I: i64 = SAFE_THRESHOLD as i64;

impl TimeAdInfinitum {
    pub fn new() -> Self {
        Self {
            short: 0.0,
            step0: Vec::new(),
        }
    }
    
    pub fn inc(&mut self, f: f32) -> Result<(), TimeAdInfinitumNotify> {
        if f == 0.0 { return Ok(()) }// nothing to add = nothing to do
        
        if f < 0.0 {
            log::warn!("Time reversal? Really? Fine, fine…");
        }

        let mut notif = Ok(());
        let delta = f as f64;
        self.short += delta;

        while self.short >= SAFE_THRESHOLD {
            let carry = (self.short / SAFE_THRESHOLD).floor() as i64;
            self.short -= carry as f64 * SAFE_THRESHOLD;
            self.propagate_carry(0, carry, &mut notif, TimeAdInfinitumNotify::Step0Increased);
        }

        while self.short < 0.0 {
            let borrow = ((-self.short) / SAFE_THRESHOLD).ceil() as i64;
            self.short += borrow as f64 * SAFE_THRESHOLD;
            self.propagate_carry(0, -borrow, &mut notif, TimeAdInfinitumNotify::Step0Decreased);
        }

        notif
    }

    #[inline(always)]
    pub fn step(&mut self) -> Result<(), TimeAdInfinitumNotify> {
        self.inc(1.0)
    }

    fn propagate_carry(
        &mut self,
        index: usize,
        amount: i64,
        notif: &mut Result<(), TimeAdInfinitumNotify>,
        variant: TimeAdInfinitumNotify
    ) {
        if amount == 0 { return }

        if amount < 0 && index >= self.step0.len() {
            // can't borrow from non-existent higher tier; clamp at abs.zero time origin.
            self.short = 0.0;
            self.step0.clear();
            *notif = Err(TimeAdInfinitumNotify::FlooredToZeroTime);
            return;
        }

        if index >= self.step0.len() {
            let needed = index + 1;
            if let Err(_) = self.step0
                .try_reserve_exact(needed - self.step0.len())
                .map_err(|_| TimeAdInfinitumNotify::BeyondTimeAndSpace) {
                    // this is an actual *FATAL* error, but very likely never happening…
                    panic!("{}", TimeAdInfinitumNotify::BeyondTimeAndSpace);
                }
            self.step0.resize(needed, 0);
        }

        *notif = Err(variant);

        let mut new = self.step0[index] as i64 + amount;
        if new >= 0 {
            let carry = new / SAFE_THRESHOLD_I;
            new %= SAFE_THRESHOLD_I;
            self.step0[index] = new as u64;
            if carry != 0 {
                self.propagate_carry(index + 1, carry, notif, variant);
            }
        } else {
            let borrow = ((-new) + SAFE_THRESHOLD_I - 1) / SAFE_THRESHOLD_I;
            new += borrow * SAFE_THRESHOLD_I;
            self.step0[index] = new as u64;
            self.propagate_carry(index + 1, -borrow, notif, variant);
        }
    }

    /// Returns the *approximate* total magnitude as a single float.
    /// FYI: *very* lossy for *immense* time scales…
    pub fn as_f64(&self) -> f64 {
        let mut total = self.short;
        let mut mul = SAFE_THRESHOLD;
        for &val in &self.step0 {
            total += val as f64 * mul;
            mul *= SAFE_THRESHOLD;
        }
        total
    }

    #[cfg(test)]
    fn set(&mut self, v: f64, step0: Vec<u64>) -> &mut Self {
        self.short = v;
        self.step0 = step0;
        self
    }
}

impl Display for TimeAdInfinitum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.step0.len(), self.short, self.as_f64())
    }
}

#[cfg(test)]
mod absurd_time {
    use crate::time::{SAFE_THRESHOLD, TimeAdInfinitum, TimeAdInfinitumNotify};

    #[test]
    fn we_get_step0inc() {
        _ = env_logger::try_init();
        let mut at = TimeAdInfinitum::new();
        log::debug!("@{at}");
        
        at.set(SAFE_THRESHOLD - 0.1, vec![]);
        log::debug!("@{at}");

        if let Err(e) = at.inc(0.05) {
            log::warn!("{e}")
        }
        log::debug!("@{at}");

        if let Err(e) = at.inc(0.05) {
            log::warn!("{e}")
        }
        log::debug!("@{at}");

        if let Err(e) = at.inc(0.05) {
            log::warn!("{e}")
        }
        log::debug!("@{at}");
    }

    #[test]
    fn norm_inc_and_fast_path() {
        let mut at = TimeAdInfinitum::new();
        assert!(at.inc(500.0).is_ok());
        assert!(at.inc(500.0).is_ok());
        // should 'short' without step0'ing
        assert_eq!(at.step0.len(), 0);
        assert!((at.as_f64() - 1000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn exact_threshold_boundary() {
        let mut at = TimeAdInfinitum::new();
        at.set(SAFE_THRESHOLD - 1.0, vec![]);
        
        // hop across the border…
        let res = at.inc(1.0);
        // border patrol notified?
        assert!(res.is_err());
        assert_eq!(at.step0.len(), 1);
        assert_eq!(at.step0[0], 1);
        assert!((at.short).abs() < f64::EPSILON);
    }

    #[test]
    fn tier_cascade() {
        let mut at = TimeAdInfinitum::new();
        // fill'er tier 0 right to the brim; SAFE_THRESHOLD-1 as 'short', and max in step0[0])
        let max_tier = (SAFE_THRESHOLD as u64) - 1;
        at.set(SAFE_THRESHOLD - 1.0, vec![max_tier]);

        // adding 1.0 should over-step0[0] and cascade into step0[1]
        let res = at.inc(1.0);
        assert!(matches!(res, Err(TimeAdInfinitumNotify::Step0Increased)));
        
        // step0 should now have grown to index 1
        assert!(at.step0.len() >= 2);
        assert_eq!(at.step0[0], 0);
        assert_eq!(at.step0[1], 1);
    }

    #[test]
    fn time_reversal_and_borrow() {
        let mut at = TimeAdInfinitum::new();
        // start with carriage
        at.set(10.0, vec![5]);

        // decrease enough to force a borrow from step0[0] down to short
        let res = at.inc(-20.0);
        assert!(matches!(res, Err(TimeAdInfinitumNotify::Step0Decreased)));
        
        assert_eq!(at.step0[0], 4);
        // short should be 10.0 - 20.0 + SAFE_THRESHOLD
        let expected_short = SAFE_THRESHOLD - 10.0;
        assert!((at.short - expected_short).abs() < 1e-3);
    }

    #[test]
    fn test_clamp_at_absolute_zero() {
        let mut at = TimeAdInfinitum::new();
        at.set(50.0, vec![]);

        // try to go way back past zero
        let res = at.inc(-100.0);
        assert!(matches!(res, Err(TimeAdInfinitumNotify::FlooredToZeroTime)));
        
        // should clamp properly
        assert!((at.short).abs() < f64::EPSILON);
        assert!(at.step0.is_empty());
    }

    #[test]
    fn test_massive_jump() {
        let mut at = TimeAdInfinitum::new();
        // jump ahead, far exceeding single-tier bounds
        let massive_delta = SAFE_THRESHOLD * SAFE_THRESHOLD * 3.5 + 50.0;
        _ = at.inc(massive_delta as f32); // f32 cast is lossy, but good enough for a stress test
        
        // see to that it didn't panic or catch fire, and it built out the higher tiers
        assert!(at.step0.len() >= 2);
        assert!(at.as_f64() > 0.0);
    }
}
