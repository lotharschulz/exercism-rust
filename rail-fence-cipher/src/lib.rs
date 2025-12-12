/// Rail fence cipher encoder/decoder.
///
/// The rail fence cipher is a transposition cipher that writes the message
/// in a zigzag pattern across multiple rails, then reads it off row by row.
pub struct RailFence {
    rails: usize,
}

impl RailFence {
    /// Creates a new rail fence cipher with the specified number of rails.
    ///
    /// # Examples
    ///
    /// ```
    /// use rail_fence_cipher::RailFence;
    /// let cipher = RailFence::new(3);
    /// ```
    #[must_use]
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    /// Encodes the given text using the rail fence cipher.
    ///
    /// The text is written in a zigzag pattern across the rails,
    /// then read off horizontally to produce the ciphertext.
    ///
    /// # Examples
    ///
    /// ```
    /// use rail_fence_cipher::RailFence;
    /// let cipher = RailFence::new(3);
    /// assert_eq!(cipher.encode("WEAREDISCOVEREDFLEEATONCE"), "WECRLTEERDSOEEFEAOCAIVDEN");
    /// ```
    #[must_use]
    pub fn encode(&self, text: &str) -> String {
        if self.rails <= 1 || text.is_empty() {
            return text.to_string();
        }

        let chars: Vec<char> = text.chars().collect();
        let mut rails: Vec<Vec<char>> = vec![vec![]; self.rails];

        for (i, &ch) in chars.iter().enumerate() {
            let rail = self.position_to_rail(i);
            rails[rail].push(ch);
        }

        rails.into_iter().flatten().collect()
    }

    /// Decodes the given ciphertext using the rail fence cipher.
    ///
    /// The ciphertext is distributed across rails according to the zigzag pattern,
    /// then read off in zigzag order to recover the original message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rail_fence_cipher::RailFence;
    /// let cipher = RailFence::new(3);
    /// assert_eq!(cipher.decode("WECRLTEERDSOEEFEAOCAIVDEN"), "WEAREDISCOVEREDFLEEATONCE");
    /// ```
    #[must_use]
    pub fn decode(&self, cipher: &str) -> String {
        if self.rails <= 1 || cipher.is_empty() {
            return cipher.to_string();
        }

        let len = cipher.chars().count();

        // Calculate how many characters belong to each rail
        let mut rail_lengths = vec![0; self.rails];
        for i in 0..len {
            let rail = self.position_to_rail(i);
            rail_lengths[rail] += 1;
        }

        // Split the cipher text into rails
        let cipher_chars: Vec<char> = cipher.chars().collect();
        let mut rails: Vec<Vec<char>> = Vec::new();
        let mut start = 0;
        for &length in &rail_lengths {
            rails.push(cipher_chars[start..start + length].to_vec());
            start += length;
        }

        // Read off characters in zig-zag order
        let mut result = Vec::new();
        let mut rail_indices = vec![0; self.rails];

        for i in 0..len {
            let rail = self.position_to_rail(i);
            result.push(rails[rail][rail_indices[rail]]);
            rail_indices[rail] += 1;
        }

        result.into_iter().collect()
    }

    /// Converts a character position to its corresponding rail number.
    fn position_to_rail(&self, position: usize) -> usize {
        let cycle_length = 2 * self.rails - 2;
        let pos = position % cycle_length;
        if pos < self.rails {
            pos
        } else {
            cycle_length - pos
        }
    }
}
