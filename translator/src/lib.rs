pub fn clean_sequence(seq: &str) -> String {
    seq.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

pub fn reverse_complement(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => 'N',
        })
        .collect()
}

pub fn translate_dna(seq: &str) -> String {
    let codon_table = get_codon_table();
    let mut protein = String::new();
    for codon in seq.as_bytes().chunks(3) {
        if codon.len() == 3 {
            let codon_str = std::str::from_utf8(codon).unwrap();
            protein.push(*codon_table.get(codon_str).unwrap_or(&'X'));
        }
    }
    protein
}

fn get_codon_table() -> std::collections::HashMap<&'static str, char> {
    use std::collections::HashMap;
    let mut table = HashMap::new();
    table.insert("ATG", 'M');
    table.insert("TTT", 'F');
    table.insert("TTC", 'F');
    table.insert("TTA", 'L');
    table.insert("TTG", 'L');
    table.insert("TAA", '*');
    table.insert("TAG", '*');
    table.insert("TGA", '*');
    // Add more codons later
    table
}