// Copyright 2015 Michael 'ExpHP' Lamparski
//
// Licensed under the terms of the MIT License, available at:
//  http://opensource.org/licenses/MIT
// and also included in the file COPYING at the root of this distribution.
// This file may not be copied, modified, or distributed except according
// to those terms.

//! Documentation is currently under construction.

// TODO: Make documentation not be "currently under construction." :P

#![feature(test)]
#![feature(step_by)]

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![deny(unused_must_use)]

extern crate num;
extern crate num_bigint;
extern crate test;
extern crate rand;
extern crate bit_set;

mod primes;
mod factorizer;
mod factorizer_dixon;
mod factorizer_pollard;
mod factorizer_sieve;
mod iter_ext;
mod util;
mod factors;
pub mod prelude;

pub use primes::PrimeTester;
pub use primes::PrimeSieve;
pub use primes::MillerRabinTester;
pub use factorizer::Factorizer;
pub use factorizer::TrialDivisionFactorizer;
pub use factorizer::StubbornFactorizer;
pub use factorizer::FactorSieve;
//pub use factorizer::FermatFactorizer;
//pub use factorizer::GeneralFactorizer;
pub use factorizer::SafeFactorizer;
pub use factorizer::DefaultFactorizer;
pub use factorizer::ListFactorizer;
//pub use factorizer_dixon::DixonFactorizer; // FIXME broken, has disabled tests
pub use factorizer_pollard::PollardBrentFactorizer;
pub use factorizer_pollard::PollardBrentFactorizerBigInt; // FIXME total hack
pub use iter_ext::FactorExt;
pub use util::isqrt;
pub use util::gcd;
pub use factors::Factors;

use util::literal;

use std::iter::FromIterator;
use std::ops::Shr;

use num::{Zero,One,Integer};
use num::{FromPrimitive,ToPrimitive};

/// Factors a number using `DefaultFactorizer`.
pub fn factorize<T>(x: T) -> Factors<T>
 where T: Clone + Zero + One + Integer + Shr<usize, Output=T> + ToPrimitive + FromPrimitive
{
	DefaultFactorizer.factorize(x)
}

/// Collects all primes up to a limit (inclusive)
///
/// # Example
///
/// ```
/// use factor::primes_upto;
///
/// let v: Vec<u64> = primes_upto(17);
/// assert_eq!(v, vec![2, 3, 5, 7, 11, 13, 17]);
/// ```
pub fn primes_upto<Out>(limit: usize) -> Out
 where Out: FromIterator<u64>
{
	use std::iter::{empty,once};

	if limit < 2 {
		return empty().collect();
	}

	let stop = limit + 1;
	let sieve = primes::PrimeSieve::new(stop);

	let odd_primes = (3..stop).step_by(2)
		.filter(|&i| sieve.is_prime(&i))
		.map(|i| i as u64);

	once(2).chain(odd_primes).collect()
}

#[test]
fn test_primes_upto() {
	// a func to help rust infer the output type
	fn get_em(limit:usize) -> Vec<u64> { primes_upto(limit) }

	assert_eq!(get_em(0), vec![]);
	assert_eq!(get_em(1), vec![]);
	assert_eq!(get_em(2), vec![2]);
	assert_eq!(get_em(3), vec![2, 3]); // first odd prime
	assert_eq!(get_em(4), vec![2, 3]); // even between odd primes
	assert_eq!(get_em(17), vec![2, 3, 5, 7, 11, 13, 17]); // arbitrary odd prime
}
