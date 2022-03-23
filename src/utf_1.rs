/// This module handle the now deprecated UTF-1 encoding.

/* ---------------------------- Helper functions ---------------------------- */

// Note: as I didn't find proper definition for UTF-1, I had to follow the
// indications here: http://czyborra.com/utf/#UTF-1

// Constants used to easily manipulate UTF-1 encoded values.
const   ONE_BIT_GLYPH_LIMIT: u32 = 0xA0;
const   TWO_BIT_GLYPH_LIMIT: u32 = 0x100;
const THREE_BIT_GLYPH_LIMIT: u32 = 0x4016;
const  FOUR_BIT_GLYPH_LIMIT: u32 = 0x38E2E; /* I found some inconsistencies in this value, the number expressed here: http://czyborra.com/utf/#UTF-1 does not match the test cases made from here: https://en.wikipedia.org/wiki/UTF-1. I chose to use the value that matches the examples in the Wikipedia article because it also matched this document: https://web.archive.org/web/20150318032101/http://kikaku.itscj.ipsj.or.jp/ISO-IR/178.pdf */
const          UTF_1_MODULO: u32 = 0xBE;

/// Transform function with a lot of magic numbers I don't understand fully. ^^'
fn t(z: u8) -> u8 {
    if z < 0x5E {
        z + 0x21
    } else if z < 0xBE {
        z + 0x42
    } else if z < 0xDF {
        z - 0xBE
    } else {
        z - 0x60
    }
}

/// Convert an UTF-32 glyph into the equivalents UTF-1 bytes.
fn utf_32_glyph_to_utf_1(glyph: u32) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    if glyph < ONE_BIT_GLYPH_LIMIT {
        ret.push(glyph as u8);
    } else if glyph < TWO_BIT_GLYPH_LIMIT {
        ret.push(ONE_BIT_GLYPH_LIMIT as u8);
        ret.push(glyph as u8);
    } else if glyph < THREE_BIT_GLYPH_LIMIT {
        ret.push((161 + (glyph - TWO_BIT_GLYPH_LIMIT) / UTF_1_MODULO) as u8);
        ret.push(t(((glyph - TWO_BIT_GLYPH_LIMIT) % UTF_1_MODULO) as u8));
    } else if glyph < FOUR_BIT_GLYPH_LIMIT {
        ret.push((246 + (glyph - THREE_BIT_GLYPH_LIMIT) / UTF_1_MODULO / UTF_1_MODULO) as u8);
        ret.push(t(((glyph - THREE_BIT_GLYPH_LIMIT) / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t(((glyph - THREE_BIT_GLYPH_LIMIT) % UTF_1_MODULO) as u8));
    } else {
        ret.push((252 + (glyph - FOUR_BIT_GLYPH_LIMIT) / UTF_1_MODULO / UTF_1_MODULO / UTF_1_MODULO / UTF_1_MODULO) as u8);
        ret.push(t(((glyph - FOUR_BIT_GLYPH_LIMIT) / UTF_1_MODULO / UTF_1_MODULO / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t(((glyph - FOUR_BIT_GLYPH_LIMIT) / UTF_1_MODULO / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t(((glyph - FOUR_BIT_GLYPH_LIMIT) / UTF_1_MODULO % UTF_1_MODULO) as u8));
        ret.push(t(((glyph - FOUR_BIT_GLYPH_LIMIT) % UTF_1_MODULO) as u8));
    }
    return ret;
}

/* --------------------------------- Testing -------------------------------- */

#[test]
/// Test that various code-points are converted right.
fn test_utf_32_to_utf_8_raw() {
    fn simple_conv(glyph: u32, vec: Vec<u8>) {
        let conv = utf_32_glyph_to_utf_1(glyph);
        assert_eq!(conv, vec);
    }
    simple_conv(0x000045, vec![0x45]);
    simple_conv(0x00007F, vec![0x7F]);
    simple_conv(0x000080, vec![0x80]);
    simple_conv(0x00009F, vec![0x9F]);
    simple_conv(0x0000A0, vec![0xA0, 0xA0]);
    simple_conv(0x0000BF, vec![0xA0, 0xBF]);
    simple_conv(0x0000C0, vec![0xA0, 0xC0]);
    simple_conv(0x0000FF, vec![0xA0, 0xFF]);
    simple_conv(0x000100, vec![0xA1, 0x21]);
    simple_conv(0x00015D, vec![0xA1, 0x7E]);
    simple_conv(0x0001BD, vec![0xA1, 0xFF]);
    simple_conv(0x0001BE, vec![0xA2, 0x21]);
    simple_conv(0x0007FF, vec![0xAA, 0x72]);
    simple_conv(0x000800, vec![0xAA, 0x73]);
    simple_conv(0x000FFF, vec![0xB5, 0x48]);
    simple_conv(0x001000, vec![0xB5, 0x49]);
    simple_conv(0x00D7FF, vec![0xF7, 0x2F, 0xC3]);
    simple_conv(0x00FDEF, vec![0xF7, 0x62, 0xD9]);
    simple_conv(0x038E2D, vec![0xFB, 0xFF, 0xFF]);
    simple_conv(0x038E2E, vec![0xFC, 0x21, 0x21, 0x21, 0x21]);
    simple_conv(0x10FFFF, vec![0xFC, 0x21, 0x39, 0x6E, 0x6C]);
}

