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
`from_be_bytes` on a byte-slice buffer.  Maybe deserializing/decoding can be done in a zero-copy 
manner.  A CDF file allows for data and attributes to have different endianness, even within one 
file.  So, it might be complicated and this is not a priority at the moment.

## Usage
`cdf-rs` decodes the CDF file in a heirarchical manner by recursively calling `decode_*` on each 
constituent. Calling the top-level `Cdf::read_cdf_file` function is the easiest.
This reads in the entire content of the CDF file.

```rust,ignore
let cdf = Cdf::read_cdf_file(PathBuf::from("examples/data/test_alltypes.cdf")).unwrap();
```

## The CDF data model and `serde` 

A CDF file is a collection of 'records'. There are different kinds of records, and some records 
point to other records of a different type, or different records of the same type 
(creating a linked-list).  But, at the lowest level, data is stored in the form of integers, floats, 
etc. Different kinds of CDF records, and different kinds of CDF primitive types are defined in the 
CDF Internal Format specification.

In a way, `cdf-rs` mimics `serde`'s strategy by creating its own data model via types that wrap 
around native Rust types.  In addition, nearly all "CdfTypes" implement `serde::Serialize` and 
`serde::Deserialize` and can be used, for example, to store the contents of the CDF file into a 
JSON file, or any other format that has `serde` support.

```text
                         _____________
                         | .CDF file |
                         |___________|
                               |
_____________      ____________|________________      ____________________      _________________
| User data | ---> | CDF data model (this lib) | ---> | serde data model | ---> | Other formats |
|___________|      |___________________________|      |__________________|      |_______________|
```

`serde` support within `cdf-rs` needs to be enabled by enabling the `serde` feature. 

Then, for example, using `serde` to convert previously read CDF data into JSON is trivial using 
`serde_json` -
```rust,ignore
let cdf_as_json = serde_json::to_string(&cdf).unwrap();
```
At the moment, any user that wishes to use this model needs to convert their data into the CDF data 
model. But that is something we could work on later to simplify.

## Work in progress
This is a new project and so will likely go through some revisions during which the API may change.

If you are interested in helping, please raise an issue on Github with whatever you'd like to work 
on.

Currently I am focusing on decoding (parsing) CDFs, since most users are interested in reading CDF 
files rather than generating them. After the decoding part is done, I will work on the encoding 
(writing).  Hopefully the encoders are not too difficult to implement by reversing the steps 
followed while decoding.
