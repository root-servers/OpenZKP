// Criterion requires the second argument to be by reference
#![allow(clippy::trivially_copy_pass_by_ref)]

use criterion::{black_box, Bencher};
use curve25519_dalek::{
    field::FieldElement,
    scalar::{Scalar, UnpackedScalar},
};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use sha2::Sha512;

pub fn dalek_ed25519_verify(bench: &mut Bencher, _i: &()) {
    let mut csprng = OsRng::new().unwrap();
    let keypair = Keypair::generate::<Sha512, _>(&mut csprng);
    let msg: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let sig = keypair.sign::<Sha512>(msg);

    bench.iter(|| black_box(keypair.verify::<Sha512>(black_box(msg), black_box(&sig))))
}

pub fn dalek_field_mul(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from_bytes(&[
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0xf2, 0x04,
    ]);
    let b = FieldElement::from_bytes(&[
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0xf2, 0x04,
    ]);
    // TODO: This does also does packing and unpacking.
    bench.iter(|| black_box(black_box(&a) * black_box(&b)));
}

pub fn dalek_field_sqr(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from_bytes(&[
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0xf2, 0x04,
    ]);
    bench.iter(|| black_box(black_box(&a).square()));
}

pub fn dalek_field_inv(bench: &mut Bencher, _i: &()) {
    let a = FieldElement::from_bytes(&[
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0xf2, 0x04,
    ]);
    bench.iter(|| black_box(black_box(&a).invert()));
}

pub fn dalek_scalar_mul(bench: &mut Bencher, _i: &()) {
    let a = Scalar::from_bytes_mod_order([
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0xf2, 0x04,
    ])
    .unpack();
    let b = Scalar::from_bytes_mod_order([
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0xf2, 0x04,
    ])
    .unpack();
    bench.iter(|| black_box(UnpackedScalar::montgomery_mul(black_box(&a), black_box(&b))));
}

pub fn dalek_scalar_sqr(bench: &mut Bencher, _i: &()) {
    let a = Scalar::from_bytes_mod_order([
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0xf2, 0x04,
    ])
    .unpack();
    bench.iter(|| black_box(black_box(&a).montgomery_square()));
}

pub fn dalek_scalar_inv(bench: &mut Bencher, _i: &()) {
    let a = Scalar::from_bytes_mod_order([
        0x4e, 0x5a, 0xb4, 0x34, 0x5d, 0x47, 0x08, 0x84, 0x59, 0x13, 0xb4, 0x64, 0x1b, 0xc2, 0x7d,
        0x52, 0x52, 0xa5, 0x85, 0x10, 0x1b, 0xcc, 0x42, 0x44, 0xd4, 0x49, 0xf4, 0xa8, 0x79, 0xd9,
        0xf2, 0x04,
    ])
    .unpack();
    bench.iter(|| black_box(black_box(&a).montgomery_invert()));
}
