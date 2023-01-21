pub mod fixed_bases;

// 3 bit windows for 255 bit number = 85 windows
pub const NUM_WINDOWS: usize = 85;
// domain seperator for hash to curve
pub const BOARD_COMMITMENT_PERSONALIZATION: &str = "battlezips:hash2curve";

/// SWU hash-to-curve value for the board commitment generator
pub const BOARD_COMMITMENT_V_BYTES: [u8; 1] = *b"v";

/// SWU hash-to-curve value for the board commitment generator
pub const BOARD_COMMITMENT_R_BYTES: [u8; 1] = *b"r";