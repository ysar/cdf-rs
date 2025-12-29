# cdf-rs

This is a standalone Rust library to parse and write to files written in NASA's Common Data 
Format ([CDF](https://cdf.gsfc.nasa.gov)).  This is different from the netCDF format.  There are 
various existing parsers to read and write CDF files. `cdf-rs` is written from scratch 
in Rust following the CDF internal format specification. It does not use or interact in any way 
with the official CDF C-library provided by NASA. In addition to the official C-library, other 
parsers for the CDF format include - 

- [`cdflib`](https://github.com/lasp/cdflib) (Python, standalone)
- [`pysatCDF`](https://github.com/pysat/pysatCDF) (Python wrapper to the official C-library)
- [`cdfj`](https://github.com/autoplot/cdfj) (Java)
- [`CDFpp`](https://github.com/SciQLop/CDFpp) (C++)

> This library is a *work in progress*. Currently I am focusing on decoding (parsing) CDFs, since 
most users are interested in reading CDF files rather than generating them. After the decoding part 
is done, I will work on the encoding (writing).  Hopefully the encoders are not too difficult to 
implement by reversing the steps followed while decoding.
