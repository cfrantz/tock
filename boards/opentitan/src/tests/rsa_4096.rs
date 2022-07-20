use crate::setup::{PERIPHERALS, RSA_HARDWARE};
use crate::tests::run_kernel_op;
use capsules::public_key_crypto::rsa_keys::RSA4096Keys;
use core::cell::Cell;
use kernel::hil::public_key_crypto::keys::{PubKey, PubPrivKey, RsaKey, RsaPrivKey};
use kernel::hil::public_key_crypto::rsa_math::{Client, RsaCryptoBase};
use kernel::static_init;
use kernel::{debug, ErrorCode};

static mut SOURCE: [u8; 64] = [0x23; 64];
static mut DEST: [u8; 512] = [0x56; 512];
static PUB_KEY: [u8; 512] = [
    // Modulus
    0x95, 0x5a, 0x06, 0xf3, 0x64, 0x65, 0x1c, 0x41, 0xc5, 0x41, 0xa3, 0x6e, 0xf4, 0xcb, 0xe9, 0x67,
    0x9f, 0xb0, 0x6e, 0x65, 0xf2, 0xe2, 0x31, 0xea, 0x21, 0x2e, 0xd1, 0xb1, 0x84, 0x82, 0xb9, 0x13,
    0x3b, 0x48, 0x0c, 0xf4, 0xc6, 0x72, 0xf3, 0xc8, 0x25, 0x8e, 0xc7, 0x6a, 0x91, 0xf8, 0x3a, 0x8f,
    0x23, 0xb4, 0x6c, 0x84, 0x9c, 0xac, 0xa0, 0xa1, 0x7b, 0x2e, 0xfb, 0xd6, 0x22, 0x99, 0x4c, 0x24,
    0xed, 0x46, 0x60, 0x67, 0xd7, 0xa4, 0x4c, 0xf1, 0xa0, 0x4e, 0x3a, 0x66, 0x9e, 0x47, 0xb2, 0x4d,
    0x69, 0x8a, 0x76, 0x56, 0x69, 0x2c, 0x2b, 0x45, 0xb6, 0x24, 0x2f, 0x30, 0x29, 0x22, 0xb8, 0xb8,
    0x35, 0xb4, 0xd1, 0x81, 0xff, 0x2a, 0xbe, 0x95, 0xe3, 0x35, 0x76, 0xd7, 0x45, 0x67, 0x99, 0xae,
    0x37, 0x28, 0x75, 0xfc, 0x1a, 0xcb, 0xb7, 0x67, 0x6d, 0x63, 0x7b, 0x8f, 0x12, 0x74, 0x26, 0x0a,
    0x32, 0x6c, 0x10, 0x4f, 0x50, 0x04, 0xf9, 0x84, 0xf1, 0x66, 0x99, 0x20, 0x1d, 0x4a, 0x10, 0x5e,
    0xce, 0xa6, 0xd6, 0xf4, 0xaa, 0xb6, 0x2f, 0xe5, 0xe6, 0xa7, 0xdb, 0xed, 0x46, 0xc6, 0x36, 0x2d,
    0xca, 0x03, 0xb2, 0xb7, 0x6e, 0x20, 0xa3, 0x1d, 0x80, 0x89, 0xa6, 0xc0, 0x31, 0x95, 0xf4, 0xa6,
    0x18, 0x68, 0x68, 0xaa, 0x47, 0xcd, 0xaa, 0x09, 0xa9, 0x22, 0xde, 0x61, 0xa5, 0xf9, 0x99, 0x0b,
    0x50, 0x0e, 0xa2, 0x0d, 0xda, 0x4d, 0x14, 0xcc, 0xec, 0xb1, 0xff, 0xfd, 0x85, 0xad, 0xe7, 0x97,
    0xbf, 0x12, 0xf8, 0xad, 0xbe, 0x21, 0xed, 0xbb, 0xa6, 0xf8, 0x48, 0xe3, 0xe2, 0x7c, 0x69, 0xff,
    0xf7, 0x7e, 0x8f, 0x86, 0x04, 0x7b, 0x6b, 0x91, 0x6c, 0xa4, 0x69, 0xa0, 0x7b, 0x15, 0xf9, 0x9c,
    0x5c, 0x13, 0x3c, 0x69, 0x82, 0x10, 0x4d, 0x0f, 0x85, 0xac, 0xd5, 0xe5, 0x37, 0x80, 0xee, 0xcb,
    0xa5, 0x76, 0x18, 0x14, 0x72, 0x99, 0x08, 0xb7, 0x91, 0x4b, 0x76, 0xc5, 0x93, 0xd1, 0xf5, 0x9c,
    0x6b, 0x29, 0x5d, 0xec, 0x0c, 0xd8, 0xf7, 0xcc, 0xd2, 0xe4, 0xad, 0xd7, 0x4c, 0x95, 0x29, 0x84,
    0x10, 0x6b, 0xed, 0x0c, 0x95, 0x27, 0xc3, 0x94, 0xf4, 0x3f, 0x3b, 0x66, 0x49, 0xec, 0x6c, 0x7e,
    0x18, 0xc1, 0x78, 0xe7, 0xa6, 0x78, 0x6c, 0x13, 0x85, 0x43, 0xd1, 0x85, 0x21, 0xe4, 0xce, 0x81,
    0xac, 0x47, 0x7a, 0xef, 0x38, 0xa5, 0x17, 0xbc, 0x6b, 0xf2, 0x23, 0xf5, 0xd0, 0x61, 0x79, 0x67,
    0x5e, 0x81, 0xd2, 0x24, 0x43, 0xdb, 0x4a, 0xb2, 0x17, 0xa3, 0x4d, 0x34, 0x0c, 0x95, 0xba, 0xfd,
    0x78, 0xe0, 0x6a, 0xa7, 0x50, 0x55, 0x63, 0xca, 0xb0, 0x7a, 0x70, 0xf0, 0x00, 0xc4, 0xa6, 0xdc,
    0x31, 0xd8, 0x91, 0xe0, 0x96, 0xc1, 0x87, 0x38, 0x48, 0xa6, 0x0d, 0x3e, 0xe2, 0xcd, 0xee, 0x76,
    0x55, 0x44, 0xb2, 0x76, 0xf9, 0x00, 0x92, 0x1d, 0x30, 0xd5, 0xb4, 0x7a, 0xd5, 0x6a, 0x70, 0x68,
    0x25, 0x94, 0x47, 0x7a, 0x6a, 0xbc, 0xfe, 0xb8, 0xa1, 0xe7, 0xfe, 0xc6, 0x54, 0x0f, 0xfe, 0x62,
    0xfc, 0x4d, 0x55, 0x45, 0xcc, 0x13, 0xda, 0xdb, 0xf8, 0x80, 0xf8, 0xe5, 0x28, 0x26, 0xec, 0x88,
    0xb0, 0x40, 0xa9, 0x73, 0x24, 0x1d, 0x53, 0x25, 0xaa, 0x09, 0x30, 0xb6, 0x96, 0xa6, 0x81, 0x92,
    0xd5, 0x9f, 0x43, 0x18, 0xf2, 0x14, 0x80, 0x9a, 0x47, 0xb3, 0xbe, 0xb9, 0xa2, 0x1f, 0xc1, 0x4d,
    0x21, 0x07, 0x0a, 0x98, 0xc5, 0x13, 0xd4, 0x2f, 0x04, 0x30, 0x35, 0xb3, 0x61, 0x21, 0x06, 0x05,
    0xbe, 0x7a, 0x1b, 0xf5, 0xd6, 0xb3, 0x63, 0xa8, 0x74, 0x68, 0x2e, 0xdb, 0xac, 0x8e, 0xd6, 0xd0,
    0xf2, 0x60, 0x35, 0x04, 0xdb, 0x74, 0x7b, 0x4d, 0x17, 0xb8, 0xd8, 0xc0, 0x1d, 0x0e, 0x86, 0xd3,
];
static PRIV_KEY: [u8; 512] = [
    // Private Exponent
    0x07, 0x18, 0x1c, 0xa6, 0x69, 0x09, 0x68, 0x7b, 0x33, 0x4c, 0x77, 0xdf, 0xe8, 0x5e, 0xdb, 0x3a,
    0x61, 0xda, 0x76, 0x93, 0xff, 0x22, 0x81, 0x6e, 0x76, 0x9f, 0x0b, 0xb4, 0xdb, 0xef, 0x7d, 0xad,
    0x0d, 0x2e, 0xd1, 0xf6, 0xba, 0x8a, 0x71, 0x4b, 0xfb, 0x84, 0xb9, 0xb2, 0x35, 0x36, 0xce, 0x49,
    0x48, 0x4f, 0xe4, 0xab, 0xb3, 0xe9, 0x7b, 0x43, 0xd0, 0x5f, 0x1d, 0xf5, 0x40, 0xf5, 0x79, 0x29,
    0x73, 0xdf, 0xd8, 0xea, 0x75, 0xd2, 0xc7, 0x18, 0xdf, 0x1d, 0x78, 0x26, 0xb1, 0xb4, 0x04, 0x23,
    0x2b, 0x35, 0x39, 0x83, 0xc7, 0x41, 0x22, 0xd9, 0x0f, 0xda, 0xce, 0x27, 0x02, 0x7d, 0x34, 0xbb,
    0x03, 0x4a, 0x10, 0x7d, 0x95, 0x4a, 0x49, 0x7d, 0x43, 0x2a, 0xa1, 0xf7, 0x7d, 0xc3, 0x7b, 0x08,
    0x4c, 0x74, 0x6e, 0x8e, 0x48, 0x13, 0x8f, 0x25, 0xa9, 0x8b, 0x85, 0x2d, 0xf9, 0x99, 0x6c, 0xc9,
    0x25, 0x35, 0xfe, 0xdc, 0x55, 0x97, 0xb6, 0xe6, 0x7a, 0xb4, 0xfd, 0xe7, 0x09, 0x9d, 0x20, 0x03,
    0xf0, 0xda, 0xf9, 0xf0, 0xeb, 0x3e, 0xf6, 0x2d, 0x7c, 0x74, 0x52, 0xbd, 0x05, 0x94, 0x9a, 0xb4,
    0x38, 0x19, 0x4e, 0xde, 0xe9, 0xab, 0x41, 0x34, 0x40, 0x8b, 0xc9, 0x50, 0xed, 0xca, 0x0a, 0xb1,
    0xed, 0x0c, 0xe0, 0x93, 0xde, 0x0f, 0x45, 0xd9, 0x69, 0xa0, 0x4f, 0x61, 0xe2, 0x09, 0x66, 0x1c,
    0xa4, 0x73, 0xdd, 0x7c, 0xbc, 0xf6, 0xd6, 0x5b, 0x19, 0x8f, 0x26, 0xf2, 0xba, 0xeb, 0xd4, 0xc9,
    0x5f, 0x79, 0x1e, 0x38, 0xda, 0x84, 0x30, 0x0f, 0xae, 0xd1, 0xb2, 0x5c, 0xa8, 0xc0, 0x72, 0x24,
    0x87, 0x74, 0x0a, 0x8b, 0x3a, 0x21, 0x46, 0xad, 0xc4, 0xb4, 0x56, 0x94, 0x19, 0xe4, 0xa4, 0x53,
    0xaa, 0x07, 0xfb, 0xe1, 0xa3, 0x1c, 0xf5, 0x1c, 0x1c, 0xa4, 0x66, 0x10, 0x9a, 0x3f, 0x54, 0x98,
    0x63, 0x32, 0x8f, 0xc9, 0x3f, 0x0a, 0x29, 0x56, 0x83, 0xb2, 0xfc, 0xd4, 0x81, 0x77, 0xa2, 0xe1,
    0x85, 0x5e, 0x1a, 0x28, 0x03, 0x2c, 0xda, 0xcf, 0x0d, 0xd7, 0x65, 0x78, 0x5c, 0xe7, 0xf2, 0x87,
    0x92, 0x02, 0x6b, 0xbd, 0x49, 0x4a, 0xd7, 0x0d, 0x7e, 0x10, 0xf8, 0x7b, 0x6c, 0xcb, 0xbc, 0x49,
    0x85, 0x0b, 0xd8, 0x46, 0xf9, 0x1d, 0xe5, 0x45, 0x54, 0x16, 0x71, 0xf9, 0x58, 0xa6, 0xa8, 0x9a,
    0x17, 0xcc, 0x2b, 0x4c, 0xd5, 0xc1, 0xa0, 0xa9, 0xd4, 0x38, 0xcb, 0x32, 0x30, 0x08, 0x62, 0x97,
    0x8b, 0xe8, 0xb4, 0x79, 0xb4, 0xee, 0x74, 0xef, 0x36, 0xbb, 0x44, 0x35, 0x57, 0x55, 0xd5, 0xb1,
    0x69, 0x13, 0xc4, 0x76, 0x6e, 0x1e, 0x8e, 0x5b, 0xef, 0x04, 0xda, 0xa1, 0x21, 0xe4, 0xa1, 0x55,
    0x64, 0x56, 0x03, 0xbf, 0xd2, 0x9d, 0xdb, 0xfa, 0x77, 0xbd, 0xc2, 0xe7, 0x4f, 0xbc, 0x15, 0x96,
    0xfc, 0x93, 0x7e, 0x8d, 0xd1, 0x1f, 0x33, 0x94, 0x5c, 0x12, 0x51, 0xa3, 0x38, 0x72, 0x2e, 0xfa,
    0xce, 0xec, 0x80, 0x8c, 0xd0, 0xbe, 0x55, 0x24, 0xb5, 0x96, 0x4e, 0xd8, 0x89, 0x1f, 0x62, 0xdf,
    0xbe, 0x01, 0x66, 0x87, 0x9f, 0xb2, 0x9a, 0x47, 0x15, 0xf3, 0x74, 0x64, 0xd7, 0x78, 0xc0, 0x7b,
    0x98, 0x79, 0xc8, 0x36, 0xf0, 0x1e, 0x36, 0x60, 0x4f, 0x99, 0xdd, 0x44, 0x0a, 0x6d, 0x45, 0x1f,
    0x6b, 0x8b, 0x06, 0xc0, 0x05, 0x05, 0x1e, 0x66, 0xb8, 0x72, 0x2c, 0x9a, 0xd2, 0xc7, 0x93, 0x81,
    0xcd, 0x2f, 0x6c, 0xbd, 0xf4, 0x44, 0xd9, 0xb8, 0x2e, 0xbb, 0x2b, 0x87, 0x67, 0xbd, 0x9f, 0x9d,
    0xc1, 0x8d, 0xd4, 0x56, 0x51, 0x01, 0x54, 0xe1, 0x26, 0xe4, 0xb2, 0x47, 0x87, 0xaa, 0x7f, 0xf3,
    0x23, 0xaa, 0x93, 0xfe, 0xe0, 0xf6, 0x5d, 0x6a, 0x2d, 0xde, 0xc5, 0x61, 0x27, 0xff, 0xed, 0x21,
];

static EXPECTING: [u8; 512] = [
    0x88, 0x8c, 0x2b, 0x88, 0xb7, 0xe0, 0x55, 0x23, 0xaa, 0x52, 0xea, 0x6f, 0x99, 0xd5, 0x08, 0x36,
    0x85, 0xd3, 0x0c, 0x66, 0xd9, 0x70, 0x75, 0xa6, 0x44, 0x9a, 0x20, 0x28, 0x4d, 0x17, 0x85, 0x2a,
    0x04, 0x0a, 0x86, 0x92, 0xd3, 0xf8, 0xd5, 0xf2, 0xec, 0x9a, 0x9e, 0x2b, 0x56, 0x7d, 0x5b, 0x3f,
    0x50, 0xde, 0x26, 0x73, 0x8f, 0xd8, 0xa7, 0xbf, 0xe7, 0xa3, 0xf3, 0x47, 0x9a, 0xc7, 0x74, 0x3f,
    0x71, 0x98, 0x6a, 0xfa, 0x35, 0x59, 0x96, 0xa1, 0x4c, 0xda, 0x6f, 0x44, 0x90, 0x24, 0x47, 0x48,
    0xc3, 0x28, 0x35, 0x54, 0x4d, 0x82, 0xfe, 0xdf, 0xb9, 0x38, 0x22, 0xa2, 0x3f, 0x6c, 0x54, 0x93,
    0xb2, 0xb9, 0x2f, 0x7e, 0x60, 0x59, 0x74, 0x6a, 0xbb, 0xab, 0xdb, 0x5a, 0xbb, 0x42, 0xc9, 0x2d,
    0xd8, 0x81, 0x2a, 0x66, 0xb2, 0xcd, 0x01, 0xdd, 0x64, 0x59, 0xb9, 0x7e, 0x15, 0x37, 0x8d, 0xe3,
    0x99, 0x39, 0x4f, 0xc6, 0x9b, 0xa5, 0xe8, 0xcd, 0x96, 0x20, 0x34, 0x8d, 0xa0, 0x52, 0x5c, 0x09,
    0x7e, 0x72, 0x02, 0x75, 0xda, 0x2f, 0x10, 0x2d, 0xcc, 0x20, 0xdf, 0x10, 0x67, 0xfe, 0x87, 0x03,
    0xe8, 0xd6, 0x8f, 0xad, 0xa6, 0xaa, 0xfb, 0x41, 0xd7, 0xcf, 0x84, 0xa0, 0x3d, 0xbc, 0x1a, 0xaf,
    0x71, 0xab, 0xdf, 0x69, 0xb6, 0x63, 0xcc, 0x99, 0xca, 0x86, 0xeb, 0xc0, 0x92, 0x03, 0x40, 0xe4,
    0xbd, 0x9b, 0xbd, 0x5b, 0x06, 0xfd, 0xdb, 0xb3, 0x38, 0x11, 0x98, 0x70, 0x08, 0x59, 0xc0, 0x20,
    0xd4, 0x3d, 0x58, 0x76, 0xca, 0xd0, 0x00, 0x6a, 0xce, 0x67, 0xaf, 0x4f, 0x5f, 0x96, 0x0e, 0x34,
    0x0e, 0xa8, 0xdf, 0x04, 0x4c, 0xcc, 0x8e, 0xb8, 0x21, 0xfe, 0x9b, 0x15, 0xfd, 0x7d, 0xc0, 0x3d,
    0xb3, 0xe5, 0xac, 0xcd, 0x58, 0xcc, 0x1a, 0x4f, 0x10, 0x83, 0x0e, 0x46, 0xff, 0x13, 0xa0, 0x8e,
    0x78, 0xfe, 0xf2, 0xb1, 0x91, 0x79, 0x21, 0x0f, 0x6c, 0x44, 0x26, 0x52, 0xaa, 0x4e, 0xcc, 0x74,
    0xe0, 0x9d, 0x32, 0xdc, 0x6a, 0x41, 0x59, 0xfb, 0xa3, 0x52, 0xdc, 0xad, 0x26, 0xdf, 0xed, 0xb5,
    0xc6, 0x94, 0x86, 0x8d, 0xfb, 0x97, 0x26, 0xc2, 0xec, 0x3c, 0xf6, 0x53, 0x0c, 0x5f, 0x3d, 0x1d,
    0x42, 0x34, 0x83, 0x41, 0xaf, 0xa9, 0xcc, 0xba, 0xe6, 0xcb, 0xed, 0x44, 0xe1, 0xa5, 0xbf, 0x75,
    0x6d, 0x02, 0x98, 0x85, 0x51, 0xcd, 0x08, 0xba, 0xfc, 0x98, 0x2e, 0xd3, 0x36, 0x5a, 0x8a, 0x7c,
    0x7b, 0x27, 0x06, 0x78, 0x2d, 0x87, 0x54, 0x30, 0x42, 0xa1, 0xdf, 0xec, 0xc7, 0xb3, 0xfa, 0x2e,
    0x79, 0xe5, 0xa2, 0x4d, 0x3e, 0xff, 0xc3, 0xad, 0xa6, 0x1e, 0xa0, 0x70, 0x1a, 0xa0, 0xb0, 0xeb,
    0xa1, 0xaa, 0x29, 0xfb, 0x8d, 0x5b, 0x9d, 0x45, 0x63, 0x4d, 0x13, 0xa0, 0xb7, 0x06, 0x1e, 0x64,
    0x58, 0x86, 0xed, 0x22, 0xa3, 0xbd, 0x5c, 0xf9, 0xa6, 0x13, 0xb0, 0x2c, 0x57, 0xcc, 0x8d, 0xd7,
    0x25, 0x23, 0x98, 0xc5, 0xf3, 0xf5, 0x53, 0x07, 0x7a, 0x2c, 0x92, 0xd8, 0x6a, 0xf5, 0x8e, 0x79,
    0xdd, 0x84, 0x5c, 0x62, 0x02, 0x9a, 0x02, 0x6c, 0x4d, 0xf3, 0x9e, 0x9f, 0x8e, 0xc4, 0xce, 0xc6,
    0x44, 0x8e, 0x96, 0x8d, 0xa1, 0x1b, 0xa0, 0x08, 0x0b, 0x89, 0x84, 0x3d, 0xb6, 0x67, 0x45, 0x65,
    0xc5, 0xc4, 0x6b, 0x2b, 0x8f, 0xcf, 0x0a, 0xc9, 0xa7, 0x44, 0x97, 0xff, 0x55, 0x38, 0x30, 0x2b,
    0xdd, 0x94, 0x5d, 0x91, 0xeb, 0xe4, 0x75, 0x91, 0x61, 0xeb, 0xdd, 0x97, 0x58, 0x57, 0x18, 0x55,
    0x2e, 0x8f, 0xaf, 0x17, 0xaf, 0x76, 0x3a, 0x92, 0x63, 0x71, 0xde, 0x7e, 0x94, 0xae, 0x47, 0x11,
    0xf1, 0x1f, 0x46, 0x27, 0x7f, 0x94, 0x97, 0xcd, 0x44, 0x1d, 0x91, 0x20, 0x52, 0xcf, 0x5e, 0x4a,
];

struct RsaTestCallback {
    mod_exp_done: Cell<bool>,
    run: Cell<usize>,
}

unsafe impl Sync for RsaTestCallback {}

impl<'a> RsaTestCallback {
    const fn new() -> Self {
        RsaTestCallback {
            mod_exp_done: Cell::new(false),
            run: Cell::new(0),
        }
    }

    fn reset(&self) {
        self.mod_exp_done.set(false);
    }
}

impl<'a> Client<'a> for RsaTestCallback {
    fn mod_exponent_done(
        &'a self,
        status: Result<bool, ErrorCode>,
        _message: &'static mut [u8],
        _modulus: &'static [u8],
        _exponent: &'static [u8],
        result: &'static mut [u8],
    ) {
        assert_eq!(status, Ok(true));

        if self.run.get() == 0 {
            assert_eq!(result, EXPECTING);
        }

        self.run.set(self.run.get() + 1);
        self.mod_exp_done.set(true);
    }
}

static CALLBACK: RsaTestCallback = RsaTestCallback::new();

#[test_case]
fn rsa_import_key() {
    let key = unsafe { static_init!(RSA4096Keys, RSA4096Keys::new()) };

    debug!("check rsa 4096 bit key import... ");
    run_kernel_op(100);

    if let Err(e) = key.import_public_key(&PUB_KEY) {
        panic!("Failed to import public key: {:?}", e.0);
    }
    if let Err(e) = key.import_private_key(&PRIV_KEY) {
        panic!("Failed to import private key: {:?}", e.0);
    }

    run_kernel_op(1000);

    assert_eq!(
        key.map_modulus(&|modulus| {
            assert_eq!(modulus, PUB_KEY);
        }),
        Some(())
    );

    assert_eq!(
        key.map_exponent(&|exponent| {
            assert_eq!(exponent, PRIV_KEY);
        }),
        Some(())
    );

    assert_eq!(key.public_exponent(), Some(0x10001));

    debug!("    [ok]");
    run_kernel_op(100);
}

#[test_case]
fn rsa_check_exponent() {
    let perf = unsafe { PERIPHERALS.unwrap() };
    let otbn = &perf.otbn;
    if let Some(rsa) = unsafe { RSA_HARDWARE } {
        let key = unsafe { static_init!(RSA4096Keys, RSA4096Keys::new()) };

        debug!("check rsa 4096 exponent... ");
        run_kernel_op(100);

        // Possibly overwridden by other tests
        otbn.set_client(rsa);
        rsa.set_client(&CALLBACK);

        if let Err(e) = key.import_public_key(&PUB_KEY) {
            panic!("Failed to import public key: {:?}", e.0);
        }
        if let Err(e) = key.import_private_key(&PRIV_KEY) {
            panic!("Failed to import private key: {:?}", e.0);
        }

        CALLBACK.reset();
        unsafe {
            match rsa.mod_exponent(
                &mut SOURCE,
                key.take_modulus().unwrap(),
                key.take_exponent().unwrap(),
                &mut DEST,
            ) {
                Ok(_) => {}
                Err(_) => panic!("exponent failed"),
            }
        }

        run_kernel_op(1000000);
        assert_eq!(CALLBACK.mod_exp_done.get(), true);
        unsafe {
            assert_eq!(DEST, EXPECTING);
        }

        debug!("    [ok]");
        run_kernel_op(100);
    } else {
        debug!("Not running RSA tests");
    }
}
