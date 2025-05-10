pub fn swap_char(s: &mut String, index: usize, new_char: char) -> Result<(), String> {
    // Check if the index is valid within the string's char boundaries
    let char_indices: Vec<(usize, char)> = s.char_indices().collect();
    if index >= char_indices.len() {
        return Err(format!("Index {} out of bounds for string of length {}", 
                         index, char_indices.len()));
    }
    
    // Get the byte position and character length at the specified index
    let (byte_pos, old_char) = char_indices[index];
    let char_len = old_char.len_utf8();
    
    // Remove the old character and insert the new one
    s.replace_range(byte_pos..(byte_pos + char_len), &new_char.to_string());
    
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_char_valid_index() {
        let mut s = String::from("hello");
        assert_eq!(swap_char(&mut s, 1, 'a'), Ok(()));
        assert_eq!(s, "hallo");
    }

    #[test]
    fn test_swap_char_out_of_bounds() {
        let mut s = String::from("hello");
        assert!(swap_char(&mut s, 10, 'a').is_err());
    }

    #[test]
    fn test_swap_char_empty_string() {
        let mut s = String::from("");
        assert!(swap_char(&mut s, 0, 'a').is_err());
    }

    #[test]
    fn test_swap_char_replace_with_multibyte_char() {
        let mut s = String::from("hello");
        assert_eq!(swap_char(&mut s, 1, '😊'), Ok(()));
        assert_eq!(s, "h😊llo");
    }

    #[test]
    fn test_swap_char_replace_multibyte_char() {
        let mut s = String::from("h😊llo");
        assert_eq!(swap_char(&mut s, 1, 'a'), Ok(()));
        assert_eq!(s, "hallo");
    }

    #[test]
    fn test_swap_char_replace_last_char() {
        let mut s = String::from("hello");
        assert_eq!(swap_char(&mut s, 4, 'z'), Ok(()));
        assert_eq!(s, "hellz");
    }
}