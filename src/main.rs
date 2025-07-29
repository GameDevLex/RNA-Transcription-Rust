use std::io;

fn main() {
    loop {
        // Input DNA sequence
        println!("DNA Sequence:");
        let mut dna_sequence = String::new();
        io::stdin()
            .read_line(&mut dna_sequence)
            .expect("Failed to read line");
        let dna_sequence = dna_sequence.trim(); // Remove trailing newline

        if dna_sequence.to_lowercase() == "cancel" {
            println!("closing...");
            break;
        }

        // Convert string to Uppercase
        let dna_sequence = dna_sequence.to_uppercase();

        // Check user input
        let is_base_valid: std::collections::HashSet<char> =
            ['A', 'T', 'C', 'G'].iter().cloned().collect();

        let dna_chars: std::collections::HashSet<char> = dna_sequence.chars().collect();

        if !dna_chars.is_subset(&is_base_valid) {
            println!("Invalid sequence");
            continue;
        }

        // Let's convert the dna_sequence to rna_sequence
        let rna_sequence = dna_sequence.replace("T", "U");

        println!("{}", rna_sequence);
    }
}