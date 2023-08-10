/// This module is used to convert from stream of bytes to streams of numbers
/// knowing and taking care about the endianness. It works with any number type
/// that can be bit-cased to u64.

use crate::unicode_encoding::UnicodeEncodingError::*;
use crate::unicode_encoding::UnicodeEncodingError;
use std::mem::size_of;

/* ---------------------------- Helper functions ---------------------------- */

/// Cut an integer into a vector of bytes by considering that the number
/// is little endian.
fn cut_int<T>(n: T) -> Vec<u8>  where u64: From<T> {

    /// Cut an u64 into a vector of bytes but only consider the bytes included
    /// in the size of U.
    fn cut_int_as_u64<U>(n: u64) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::new();
        for i in 0..size_of::<U>() {
            ret.push(((n >> ((8 * i) as u8)) & 0xFF) as u8);
        }
        return ret;
    }

    return cut_int_as_u64::<T>(n.into());
}

/// Generates an array of indexes used to index the result of cut_int depending
/// on the desired endianess.
fn gen_endian_indexes<T>(big_endian: bool) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    if big_endian {
        for i in (0..size_of::<T>()).rev() {
            ret.push(i);
        }
    } else {
        for i in 0..size_of::<T>() {
            ret.push(i);
        }
    }
    return ret;
}

/* ----------------------------------- API ---------------------------------- */

/// Converts a vector of numbers into a vector of bytes that is ordered with
/// the correct endianess.
pub fn to_bytes<T: Copy>(data: &Vec<T>, big_endian: bool) -> Vec<u8>  where u64: From<T> {
    let mut ret: Vec<u8> = Vec::new();
    let endian_index = gen_endian_indexes::<T>(big_endian);
    for number in data {
        let litle_endianed_number = cut_int::<T>(*number);
        for index in &endian_index {
            ret.push(litle_endianed_number[*index]);
        }
    }
    return ret;
}

/// Convert a slice of bytes into a vector of numbers.
pub fn from_bytes<T: Copy + TryFrom<u64> + std::fmt::Debug>(bytes: &[u8], big_endian: bool) -> Result<Vec<T>, UnicodeEncodingError> where <T as TryFrom<u64>>::Error: std::fmt::Debug {
    let len_t = size_of::<T>();
    if bytes.len() % len_t != 0 {
        return Err(InvalidStreamSize);
    }
    let mut ret: Vec<T> = Vec::new();
    let endian_index = gen_endian_indexes::<T>(big_endian);
    for i in 0.. (bytes.len()/len_t) {
        let mut new_number: u64 = 0;
        for j in 0..len_t {
            new_number |= (bytes[i*len_t + endian_index[j]] as u64) << (j * 8);
        }
        let new_push: T = std::convert::TryFrom::<u64>::try_from(new_number).expect("A type bigger than u64 have been used for conversions assuming that it was not. This is very bad.");
        ret.push(new_push);
    }
    return Ok(ret);
}

/* ---------------------------------- Test ---------------------------------- */

#[test]
fn test_cut_int() {
    let i1: u16 = 0x1234;
    let v1 = cut_int::<u16>(i1);
    assert_eq!(v1, vec![0x34, 0x12]);
    let i2: u64 = 0xAB_CD_EF_01_23_45_67_89;
    let v2 = cut_int::<u64>(i2);
    assert_eq!(v2, vec![0x89, 0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB]);
}

#[test]
fn test_to_bytes() {
    let nums: Vec<u16> = vec![0x1234, 0xABCD];
    let conv_le = to_bytes::<u16>(&nums, false);
    let conv_be = to_bytes::<u16>(&nums, true);
    assert_eq!(conv_le, vec![0x34, 0x12, 0xCD, 0xAB]); 
    assert_eq!(conv_be, vec![0x12, 0x34, 0xAB, 0xCD]); 
}

#[test]
fn test_from_bytes() {
    let bytes: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let conv_le = from_bytes::<u32>(&bytes, false).unwrap();
    let conv_be = from_bytes::<u32>(&bytes, true).unwrap();
    assert_eq!(conv_le, vec![0x67452301, 0xEFCDAB89]); 
    assert_eq!(conv_be, vec![0x0123_4567, 0x89AB_CDEF]); 
}

