use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // reading file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing input file.\nUsage: cargo run -- <input_file>");
        return Ok(());
    }

    let filename = &args[1];
    println!("Reading DNA sequence: {}", filename);

    //print few lines
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines().take(5) {
        println!("{}", line?);
    }

    Ok(())
}

// read DNA sequence from file
fn read_sequence(filename: &str) -> io::Result<String> {
    let mut seq = String::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let l = line?;
        if !l.starts_with('>') {
            seq.push_str(&l.trim().to_uppercase());
        }
    }
    Ok(seq)
}

// generate reverse complement
fn reverse_complement(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            'N' => 'N',
            _ => 'N',
        })
        .collect()
}

// Translate codons to amino acids
fn codon_to_aa(codon: &str) -> char {
    match codon {
        "TTT" | "TTC" => 'F',
        "TTA" | "TTG" | "CTT" | "CTC" | "CTA" | "CTG" => 'L',
        "ATT" | "ATC" | "ATA" => 'I',
        "ATG" => 'M',
        "GTT" | "GTC" | "GTA" | "GTG" => 'V',
        "TCT" | "TCC" | "TCA" | "TCG" | "AGT" | "AGC" => 'S',
        "CCT" | "CCC" | "CCA" | "CCG" => 'P',
        "ACT" | "ACC" | "ACA" | "ACG" => 'T',
        "GCT" | "GCC" | "GCA" | "GCG" => 'A',
        "TAT" | "TAC" => 'Y',
        "TAA" | "TAG" | "TGA" => '*', // stop codons
        "CAT" | "CAC" => 'H',
        "CAA" | "CAG" => 'Q',
        "AAT" | "AAC" => 'N',
        "AAA" | "AAG" => 'K',
        "GAT" | "GAC" => 'D',
        "GAA" | "GAG" => 'E',
        "TGT" | "TGC" => 'C',
        "TGG" => 'W',
        "CGT" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => 'R',
        "GGT" | "GGC" | "GGA" | "GGG" => 'G',
        _ => 'X',
    }
}

//translate a sequence in one reading frame
fn translate_frame(seq: &str, start: usize) -> String {
    let mut protein = String::new();
    let chars: Vec<char> = seq.chars().collect();
    let mut i = start;
    while i + 2 < chars.len() {
        let codon: String = chars[i..i + 3].iter().collect();
        protein.push(codon_to_aa(&codon));
        i += 3;
    }
    protein
}

// generate translations for all 6 possible reading frames
fn six_frame_translation(seq: &str) -> Vec<String> {
    let rev = reverse_complement(seq);
    let mut frames = Vec::new();
    for start in 0..3 {
        frames.push(translate_frame(seq, start));
        frames.push(translate_frame(&rev, start));
    }
    frames
}