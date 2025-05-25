#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a sequence of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &value in values {
        let mut encoded = Vec::new();
        let mut num = value;
        
        // Handle zero case specially
        if num == 0 {
            encoded.push(0);
        } else {
            // Extract 7-bit chunks from right to left
            while num > 0 {
                encoded.push((num & 0x7F) as u8);
                num >>= 7;
            }
            
            // Reverse to get correct order (most significant first)
            encoded.reverse();
            
            // Set continuation bit (bit 7) for all bytes except the last
            for i in 0..encoded.len() - 1 {
                encoded[i] |= 0x80;
            }
        }
        
        result.extend(encoded);
    }
    
    result
}

/// Given a sequence of bytes, extract each number and return a vector of the values.
/// A variable length quantity (VLQ) is a universal code that uses an arbitrary number of binary octets (eight-bit bytes) to represent an arbitrarily large integer.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < bytes.len() {
        let mut value = 0u32;
        let mut found_end = false;
        
        // Read bytes until we find one without the continuation bit
        while i < bytes.len() {
            let byte = bytes[i];
            i += 1;
            
            // Check for overflow before shifting
            if value > (u32::MAX >> 7) {
                return Err(Error::IncompleteNumber);
            }
            
            value = (value << 7) | (byte & 0x7F) as u32;
            
            // If bit 7 is not set, this is the last byte of the number
            if (byte & 0x80) == 0 {
                found_end = true;
                break;
            }
        }
        
        // If we reached the end without finding a byte without continuation bit
        if !found_end {
            return Err(Error::IncompleteNumber);
        }
        
        result.push(value);
    }
    
    Ok(result)
}
