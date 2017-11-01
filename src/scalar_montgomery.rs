// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2017 Isis Lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! DOCDOC

/// A marker trait for all the possible factors of the Montgomery modulus which
/// may be used.
pub trait MontgomeryFactor {}

/// Indicates that the element is inversely encoded; the value has one
/// `1/R` factor that needs to be canceled out.
#[derive(Copy, Clone)]
pub enum RInverse {}

/// Indicates that the element is not encoded; there is no `R` factor
/// that needs to be canceled out.
#[derive(Copy, Clone)]
pub enum NoFactor {}

/// Indicates that the element is encoded; the value has one `R`
/// factor that needs to be canceled out.
#[derive(Copy, Clone)]
pub enum R {}

/// Indicates the element is encoded twice; the value has two `R`
/// factors that need to be canceled out.
#[derive(Copy, Clone)]
pub enum RR {}

impl MontgomeryFactor for RInverse {}
impl MontgomeryFactor for NoFactor {}
impl MontgomeryFactor for R {}
impl MontgomeryFactor for RR {}

/// The number of factors of the result of a reduction.
pub trait MontgomeryReduction {
    type Output: MontgomeryFactor;
}

impl MontgomeryReduction for RR { type Output = R; }
impl MontgomeryReduction for R { type Output = NoFactor; }
impl MontgomeryReduction for NoFactor { type Output = RInverse; }

/// The number of factors of the result of an expanding operation.
pub trait MontgomeryExpansion {
    type Output: MontgomeryFactor;
}

impl<F: MontgomeryReduction> MontgomeryExpansion for (NoFactor, F) {
    type Output = F::Output;
}

impl<F: MontgomeryFactor> MontgomeryExpansion for (R, F) {
    type Output = F;
}

impl<F: MontgomeryReduction> MontgomeryExpansion for (RInverse, F) where F::Output: MontgomeryReduction {

    type Output = <<F as MontgomeryReduction>::Output as MontgomeryReduction>::Output;
}

impl MontgomeryExpansion for (RR, NoFactor) {
    type Output = <(NoFactor, RR) as MontgomeryExpansion>::Output;
}

impl MontgomeryExpansion for (RR, RInverse) {
    type Output = <(RInverse, RR) as MontgomeryExpansion>::Output;
}
