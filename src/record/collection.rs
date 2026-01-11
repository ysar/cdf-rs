use std::io::{self, SeekFrom};

use crate::{
    decode::{Decodable, Decoder},
    error::CdfError,
    types::CdfInt8,
};

/// This trait should be implemented for CDF record types that are stored in the form of a linked-
/// list with an attribute than points to the next record of the same type.
pub trait RecordList {
    /// Returns the file offset pointer to the next record in the linked-list.
    fn next_record(&self) -> Option<CdfInt8>;
}

/// This function helps to unravel a linked-list of CDF records into a single Vec.  Any record that
/// calls this must be [`Decodable`] and [`RecordList`].
///
/// # Errors
/// Will return a [`CdfError::Decode`] if the decoding fails for any reason.
pub fn get_record_vec<R, T>(decoder: &mut Decoder<R>, head: &CdfInt8) -> Result<Vec<T>, CdfError>
where
    R: io::Read + io::Seek,
    T: Decodable + RecordList,
{
    let mut result_vec = vec![];
    let mut next = head.clone();
    loop {
        _ = decoder
            .reader
            .seek(SeekFrom::Start(u64::try_from(*next)?))?;
        let record = T::decode_be(decoder)?;
        if let Some(n) = record.next_record() {
            result_vec.push(record);
            next = n;
        } else {
            result_vec.push(record);
            break;
        }
    }
    Ok(result_vec)
}
