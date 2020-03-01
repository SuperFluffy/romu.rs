//! This is a Rust implementation of Romu, a family of fast nonlinear pseudo-random number
//! generators by Mark Overton.
//!
//! The Romu pRNGs combine the linear operations of multiplication and addition with the nonlinear operation
//! of rotation (shifting and wrapping truncated bits to the end of the integer).
//!
//! Note that Romu pRNGs are not cryptographically secure, because the functions are invertible. So
//! given knowledge of a current state, the previous state can be calculated.
//!
//! For a thorough discussion of Romu, refer to its [website] and [white paper]. The present Rust
//! code is a conversion of its [reference implementation] in C for the 64 bit pRNGs.
//!
//! [website]: http://www.romu-random.org/
//! [white paper]: http://www.romu-random.org/romupaper.pdf
//! [reference implementation]: http://www.romu-random.org/code.c

const MULTIPLICATION_CONSTANT: u64 = 15241094284759029579u64;

/// An even faster version of `RomuDuo` with fewer registers, but yet again reduced capacity.
///
/// + Estimated capacity: `>= 2^62` bytes;
/// + Register pressure: 7;
/// + State size: 128 bits.
pub struct RomuDuoJr {
    state_x: u64,
    state_y: u64,
}

impl RomuDuoJr {
    pub fn new(state_x: u64, state_y: u64) -> Self {
        Self {
            state_x,
            state_y,
        }
    }

    pub fn next(&mut self) -> u64 {
        let xp: u64 = self.state_x;
        self.state_x = MULTIPLICATION_CONSTANT * self.state_y;
        self.state_y -= xp;
        self.state_y = self.state_y.rotate_left(27);
        xp
    }
}

/// Faster than `RomuTrio` due to using fewer registers, but at reduced capacity.
///
/// + Estimated capacity: 2^61 bytes;
/// + Register pressure: 5;
/// + State size: 128 bits.
pub struct RomuDuo {
    state_x: u64,
    state_y: u64,
}

impl RomuDuo {
    pub fn new(state_x: u64, state_y: u64) -> Self {
        Self {
            state_x,
            state_y,
        }
    }

    pub fn next(&mut self) -> u64 {
        let xp = self.state_x;
        self.state_x = MULTIPLICATION_CONSTANT * self.state_y;
        self.state_y = self.state_y.rotate_left(36) + self.state_y.rotate_left(15) - xp;
        xp
    }
}

/// Good general purposes RNG for producing a lot of random numbers.
///
/// + Estimated capacity: `2^75` bytes;
/// + Register pressure: 6;
/// + State size: 192 bits.
pub struct RomuTrio {
    state_x: u64,
    state_y: u64,
    state_z: u64,
}

impl RomuTrio {
    pub fn new(state_x: u64, state_y: u64, state_z: u64) -> Self {
        Self {
            state_x,
            state_y,
            state_z,
        }
    }
    
    pub fn next(&mut self) -> u64 {
        let xp = self.state_x;
        let yp = self.state_y;
        let zp = self.state_z;

        self.state_x = MULTIPLICATION_CONSTANT * zp;
        self.state_y = yp - xp;
        self.state_y = self.state_y.rotate_left(12);
        self.state_z = zp - yp;
        self.state_z = self.state_z.rotate_left(44);
        xp
    }
}

/// Very robust pRNG, but with very high capacity, but also high register pressure.
///
/// + Estimated capacity: `>= 2^90` bytes;
/// + Register pressure:: 8;
/// + State size: 256 bits.
pub struct RomuQuad {
    state_w: u64,
    state_x: u64,
    state_y: u64,
    state_z: u64,
}

impl RomuQuad {
    pub fn new(state_w: u64, state_x: u64, state_y: u64, state_z: u64) -> Self {
        Self {
            state_w,
            state_x,
            state_y,
            state_z,
        }
    }

    pub fn next(&mut self) -> u64 {
        let wp = self.state_w;
        let xp = self.state_x;
        let yp = self.state_y;
        let zp = self.state_z;

        self.state_w = MULTIPLICATION_CONSTANT * zp;
        self.state_x = zp + wp.rotate_left(52);
        self.state_y = yp - xp;
        self.state_z = yp + wp;
        self.state_z = self.state_z.rotate_left(19);

        xp
    }
}
