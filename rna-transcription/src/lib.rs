use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DNA {
    raw: String,
    convert_dna_rna: HashMap<&'static char, &'static char>,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    raw: String,
}

const DNA_NUCLEOTIDE: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDE: [char; 4] = ['C', 'G', 'A', 'U'];

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some((pos, _c)) = dna
            .char_indices()
            .find(|(_i, c)| !DNA_NUCLEOTIDE.contains(c))
        {
            return Err(pos);
        }

        Ok(DNA {
            raw: String::from(dna),
            convert_dna_rna: DNA_NUCLEOTIDE.iter().zip(RNA_NUCLEOTIDE.iter()).collect(),
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            raw: self
                .raw
                .chars()
                .map(|c| self.convert_dna_rna.get(&c).unwrap())
                .copied()
                .collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some((pos, _c)) = rna
            .char_indices()
            .find(|(_i, c)| !RNA_NUCLEOTIDE.contains(c))
        {
            return Err(pos);
        }

        Ok(RNA {
            raw: String::from(rna),
        })
    }
}
