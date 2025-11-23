# **sixsequences**
#### **Project**(BINF-2111)

A Rust tool to translate DNA/RNA sequences in all six reading frames to protein sequences for infectious disease research.


## **Team members**

Birava Savalia(bsavalia@charlotte.edu)
Tanusree Chepuri(tchepuri@charlotte)

## **Problem Statement**

In infectious disease research, scientists often analyze the genetic material of viruses and bacteria — DNA or RNA sequences.

However, understanding how these pathogens behave requires studying the **proteins** that their genes produce, since proteins perform the biological functions that cause infection or drug resistance.

Each DNA/RNA strand can be read in **three different reading frames**, and since there are **two strands** (the original and its reverse complement), there are **six possible translations**.  

Existing tools often translate only one frame or are complex to use.  
**SixSequences** provides a **fast, accurate, and easy-to-use Rust solution** that translates all six reading frames simultaneously.

## **Algorithm** (Pseudocode)

1. Convert RNA (if present) to DNA form by replacing 'U' with 'T'.
2. Generate the reverse complement of the DNA sequence:
     a. Reverse the sequence.
     b. Replace each nucleotide with its complement (A↔T, C↔G).
3. For each of the three reading frames on the original strand:
     a. Start translation from positions 0, 1, and 2.
     b. Group codons (triplets of bases) from the start position.
     c. Translate each codon into its corresponding amino acid.
4. Repeat step 3 for the reverse complement strand.
5. Output all six translated protein sequences.