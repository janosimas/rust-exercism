use std::collections::HashMap;

const VALID_NUCLETIDE: [char; 4] = ['A', 'G', 'C', 'T'];
fn is_valid_nucleotide(c: &char) -> bool {
    VALID_NUCLETIDE.contains(c)
}

fn not_valid_nucleotide(c: &char) -> bool {
    !is_valid_nucleotide(c)
}

fn is_dna_valid(dna: &str) -> Result<(), char> {
    if let Some(c) = dna.chars().find(not_valid_nucleotide) {
        return Err(c);
    }
    Ok(())
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_dna_valid(dna)?;

    if !VALID_NUCLETIDE.contains(&nucleotide) {
        return Err(nucleotide);
    }

    Ok(dna.chars().filter(|c| c == &nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    is_dna_valid(dna)?;

    Ok(VALID_NUCLETIDE
        .iter()
        .copied()
        .map(|c| (c, count(c, dna).unwrap_or(0)))
        .collect::<HashMap<char, usize>>())
}
