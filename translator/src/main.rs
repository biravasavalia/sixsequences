use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

/// SixSequences — translate DNA/RNA into proteins across six reading frames,
/// produce a FASTA protein file and a visually attractive HTML report.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input FASTA file containing one or more nucleotide sequences
    #[arg(short, long)]
    input: PathBuf,

    /// Output file prefix (default: sixsequences)
    #[arg(short = 'p', long, default_value = "sixsequences")]
    out_prefix: String,
}

fn main() -> io::Result<()> {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <input.fasta> <output_prefix>");
        return Ok(());
    }

    let input_file = &args[1];
    let output_prefix = &args[2];

    println!("Reading DNA/RNA sequence from: {}", input_file);

    // Read FASTA sequence
    let sequence = read_sequence(input_file)?;
    println!("Sequence length: {} bp", sequence.len());

    // Generate all six frames
    let frames = six_frame_translation(&sequence);

    // Write FASTA protein output
    let fasta_output = format!("{}_sixframes.fasta", output_prefix);
    write_fasta_output(&fasta_output, &frames)?;

    println!("Six-frame translation written to: {}", fasta_output);

    // Generate HTML report
    let html_output = format!("{}_report.html", output_prefix);
    write_html_report(&html_output, &sequence, &frames)?;
    println!("HTML report generated: {}", html_output);

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

fn write_fasta_output(filename: &str, frames: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for (i, frame) in frames.iter().enumerate() {
        writeln!(file, ">Frame_{}\n{}", i + 1, frame)?;
    }

    Ok(())
}

fn write_html_report(filename: &str, sequence: &str, frames: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, r#"<!DOCTYPE html>
<html>
<head>
<title>SixSequences Translation Report</title>
<style>
body {{
    font-family: Arial, sans-serif;
    background-color: #fafafa;
    padding: 20px;
    line-height: 1.6;
}}
.frame {{
    margin-bottom: 20px;
    padding: 10px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 0 6px rgba(0,0,0,0.1);
}}
.frame-title {{
    font-size: 20px;
    font-weight: bold;
    margin-bottom: 5px;
}}
.seq {{
    font-family: monospace;
    white-space: pre-wrap;
    word-wrap: break-word;
    color: #333;
}}
.footer {{
    margin-top: 40px;
    text-align: center;
    color: #777;
}}
</style>
</head>
<body>

<h1>SixSequences — Translation Report</h1>
<p><strong>Input Sequence Length:</strong> {} bp</p>

<h2>Original DNA/RNA Sequence</h2>
<div class="frame">
<pre class="seq">{}</pre>
</div>
"#, sequence.len(), sequence)?;

    // Add six frames section
    for (i, frame) in frames.iter().enumerate() {
        writeln!(
            file,
            r#"<div class="frame">
<div class="frame-title">Frame {}</div>
<pre class="seq">{}</pre>
</div>"#,
            i + 1,
            frame
        )?;
    }

    // footer
    writeln!(
        file,
        r#"<div class="footer">Generated by SixSequences (Rust)</div>
</body>
</html>"#
    )?;

    Ok(())
}