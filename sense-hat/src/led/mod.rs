
pub mod screen;

const OFF: [u8; 128] = [0x00; 128];

// OUR Files
pub const HALLOWEEN: [u8; 128] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0x00, 0x00,
    0xa0, 0xf2, 0xa0, 0xf2, 0x00, 0x00, 0xa0, 0xf2, 0xa0, 0xf2, 0x00, 0x00, 0xa0, 0xf2, 0xa0, 0xf2,
    0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2,
    0xa0, 0xf2, 0x00, 0x00, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0x00, 0x00, 0xa0, 0xf2,
    0xa0, 0xf2, 0xa0, 0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xf2, 0xa0, 0xf2,
    0x00, 0x00, 0x00, 0x00, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0xa0, 0xf2, 0x00, 0x00, 0x00, 0x00,
];

// tag::xmas[]
// Christmas Tree
pub const CHRISTMAS_TREE: [u8; 128] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x07, 0xE0, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0x00, 0x00,
    0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07,
    0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x61, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x61, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
// end::xmas[]


// r = (255, 0, 0)     # red
// o = (255, 128, 0)   # orange
// y = (255, 255, 0)   # yellow
// g = (0, 255, 0)     # green
// c = (0, 255, 255)   # cyan
// b = (0, 0, 255)     # blue
// p = (255, 0, 255)   # purple
// n = (255, 128, 128) # pink
// w =(255, 255, 255)  # white
// k = (0, 0, 0)       # blank

// heart = [
//     k, r, r, k, k, r, r, k,
//     r, r, r, r, r, r, r, r,
//     r, r, r, r, r, r, r, r,
//     r, r, r, r, r, r, r, r,
//     r, r, r, r, r, r, r, r,
//     k, r, r, r, r, r, r, k,
//     k, k, r, r, r, r, k, k,
//     k, k, k, r, r, k, k, k
//     ]

// tag::valentine[]
// Heart 
pub const HEART: [u8; 128] = [
    0x00, 0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x00,
    0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8,
    0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8,
    0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8,
    0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8,
    0x00, 0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
// end::valentine[]


//    ];
// yellow f760
const YELLOW_SMALL: [u8; 128] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const YELLOW_MED: [u8; 128] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const YELLOW_LARGE: [u8; 128] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00,
    0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00,
    0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00,
    0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00,
    0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00,
    0x00, 0x00, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const YELLOW_XL: [u8; 128] = [
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
    0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7, 0x60, 0xf7,
];