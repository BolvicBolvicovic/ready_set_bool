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