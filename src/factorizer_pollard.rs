// Copyright 2015 Michael 'ExpHP' Lamparski
//
// Licensed under the terms of the MIT License, available at:
//  http://opensource.org/licenses/MIT
// and also included in the file COPYING at the root of this distribution.
// This file may not be copied, modified, or distributed except according
// to those terms.

extern crate num;
extern crate test;

use std::collections::hash_map::{HashMap,Hasher};
use std::collections::{Bitv,BitvSet};
use std::num::{ToPrimitive,FromPrimitive}; // and regret it
use std::hash::Hash;
use std::ops::{Shr,Rem};
use std::rand::Rng;
use std::rand::weak_rng;
use std::rand::distributions::range::SampleRange;
use std::mem::swap;
use std::num::Int;

use std::cmp::min;

use num::{Zero, One, Integer};

use factorize;
use factorization::Factorization;
use factorizer::Factorizer;
use util::literal;
use util::gcd;
use util::mod_pow;

pub struct PollardBrentFactorizer<T>;

impl<T> Factorizer<T>
for PollardBrentFactorizer<T>
 where T: Eq + Clone + FromPrimitive + ToPrimitive + Zero + One + Integer + Shr<usize, Output=T> + Hash<Hasher> + SampleRange + Int,
{
	/// Produce a single factor of `x`.  PollardBrentFactorizer is nondeterministic,
	///  and will always produce the smallest non-trivial factor of any composite number.  // <--- lies  FIXME
	///  Thus, the number it returns is also always prime.
	///
	/// The runtime scales linearly with the size of the smallest factor of `x`.
	fn get_factor(self: &Self, x: &T) -> T
	{
		// Adapted from https://comeoncodeon.wordpress.com/2010/09/18/pollard-rho-brent-integer-factorization/
		if x.is_even() { return literal(2); }
		if x.is_multiple_of(&literal(3)) { return literal(3); }
		if x < &literal(2) { return x.clone(); }

		let mut rng = weak_rng();
		let mut y: T = rng.gen_range(One::one(), x.clone()); // current value in the sequence:  y := y^2 + c (mod n)
		let mut c: T = rng.gen_range(One::one(), x.clone()); // parameter of y sequence
		let mut m: T = rng.gen_range(One::one(), x.clone()); // step size when multiplying crap together

		let mut g: T = One::one(); // contains the result
		let mut r: T = One::one(); // some kind of very coarse index
		let mut q: T = One::one(); // running product of `(z-y)` values  (TODO: I think this can be made local to each k loop?)

		let mut z: T = Zero::zero();      // Initial value of `y` for the current `r` value.
		let mut y_prev: T = Zero::zero(); // Initial value of `y` for the current `k` value.

		// Perform a coarse-grained search through the sequence of `y` values.
		while g == One::one() {
			z = y.clone();

			for _ in num::iter::range(Zero::zero(), r) {
				y = next_in_sequence(y, x.clone(), c.clone());
			}

			let mut k: T = Zero::zero();
			while k < r && g == One::one() {
				y_prev = y;

				let niter = min(m.clone(), r.clone() - k.clone());

				// Multiply a bunch of (z-y) terms together (which may share factors with x)
				for _ in num::iter::range(Zero::zero(), niter) {
					y = next_in_sequence(y, x.clone(), c.clone());

					// Deviation from the source linked above, to support unsigned integers:
					//    abs(z-y) % x  --->  (x+z-y) % x
					// This is based on the notion that `gcd(+a % b, b) == gcd(-a % b, b)`,
					// so the absolute value isn't really necessary.
					q = q * (x.clone() + z.clone() - y.clone());
					q = q % x.clone();
				}

				g = gcd(x.clone(), q.clone());
				k = k + m.clone();
			}

			r = r * literal(2);
		} // end coarse-grained search

		// N.B. The following occurs when q == 0 (mod x).
		if &g == x {

			// Return to the beginning of this `k` step
			y = y_prev;

			loop {
				// Do a more fine grained search (computing the GCD every step)
				y = next_in_sequence(y, x.clone(), c.clone());
				g = gcd(x.clone(), x.clone() + z.clone() - y.clone()); // same deviation as noted above

				if g > One::one() { break; }
			}
		}

		// At this point, g is a nontrivial factor, or g == x.
		// In the latter case, g may still be composite (a "pseudoprime")
		return g;
	}
}

// computes (y**2 + c) % x
fn next_in_sequence<T>(y: T, x: T, c: T) -> T
	where T: Clone + Integer,
{
	let mut result = y.clone() * y;
	result = result % x.clone();
	result = result + c;
	result = result % x;
	return result;
}