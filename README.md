# romu.rs, a rust implementation of the Romu pRNGs

This is a Rust implementation of Romu, a family of fast nonlinear pseudo-random
number generators by Mark Overton.

The Romu pRNGs combine the linear operations of multiplication and addition
with the nonlinear operation of rotation (shifting and wrapping truncated bits
to the end of the integer).

Note that Romu pRNGs are not cryptographically secure, because the functions
are invertible. So given knowledge of a current state, the previous state can
be calculated.

For a thorough discussion of Romu, refer to its [website] and [white paper].
The present Rust code is a conversion of its [reference implementation] in C
for the 64 bit pRNGs.

[website]: http://www.romu-random.org/
[white paper]: http://www.romu-random.org/romupaper.pdf
[reference implementation]: http://www.romu-random.org/code.c

# License

Released under MIT and Apache 2.0 licenses.
