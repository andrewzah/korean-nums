use crate::errors::HangeulError;

const SYLLABLE_START: u32 = 0xAC00;
const SYLLABLE_END: u32 = 0xD7AF;

pub fn get_subject_marker(s: &str) -> &str {
    match ends_with_consonant(s).unwrap() {
        true => "이",
        false => "가",
    }
}

pub fn get_topic_marker(s: &str) -> &str {
    match ends_with_consonant(s).unwrap() {
        true => "은",
        false => "는",
    }
}

fn ends_with_consonant(input: &str) -> Result<bool, HangeulError> {
    let c = ending_syllable(input).unwrap();

    let char = syllable_to_u32(c)?;
    Ok((char - SYLLABLE_START) % 28 != 0)
}

fn ending_syllable(input: &str) -> Result<char, HangeulError> {
    match input.chars().last() {
        Some(c) => Ok(c),
        None => return Err(HangeulError::NotASyllable),
    }
}

fn is_syllable(c: char) -> bool {
    let char = c as u32;

    char >= SYLLABLE_START && char <= SYLLABLE_END
}

fn syllable_to_u32(c: char) -> Result<u32, HangeulError> {
    let code = c as u32;
    if is_syllable(c) {
        Ok(code)
    } else {
        Err(HangeulError::NotASyllable)
    }
}
