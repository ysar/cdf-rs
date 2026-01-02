# cdf-rs

This is a standalone Rust library to parse and write to files written in NASA's Common Data 
Format ([CDF](https://cdf.gsfc.nasa.gov)), which is different from UCAR's 
[netCDF](https://en.wikipedia.org/wiki/NetCDF) format.  The CDF format is commonly used in space 
physics to store data from various instruments onboard NASA spacecraft.

There are various existing parsers to read and write CDF files. `cdf-rs` is written from scratch in 
Rust following the CDF internal format specification. It does not use or interact in any way with 
the official CDF C library provided by NASA. In addition to the official C library, other parsers 
for the CDF format include,

- [`cdflib`](https://github.com/lasp/cdflib) (Python, standalone),
- [`pysatCDF`](https://github.com/pysat/pysatCDF) (Python wrapper to the official C-library),
- [`cdfj`](https://github.com/autoplot/cdfj) (Java), and
- [`CDFpp`](https://github.com/SciQLop/CDFpp) (C++).

`cdf-rs` is not zero-copy. At the primitive level, there is a call to `from_le_bytes` or 
`from_be_bytes` on a byte-slice buffer.  While it seems like it could be theoretically possible to 
implement decoding (deserializing) in a zero-copy manner, a CDF file allows for data and attributes 
to have different endianness, even within one file. This makes memory-mapping a more challenging 
task than what I can currently achieve.

[!NOTE]
> This library is a work in progress. Currently I am focusing on decoding (parsing) CDFs, since 
most users are interested in reading CDF files rather than generating them. After the decoding part 
is done, I will work on the encoding (writing).  Hopefully the encoders are not too difficult to 
implement by reversing the steps followed while decoding.
