#![warn(clippy::pedantic)]
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

// Create a struct to hold the CLI arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input File
    #[arg(short, long)]
    input: String,

    /// Output File
    #[arg(short, long)]
    output: String,

    /// Number of sentences per paragraph
    #[arg(short, long, default_value_t = 5)]
    sentences_per_paragraph: u8,
}

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    // Read the input file
    let input_file = File::open(args.input).unwrap();
    let input_reader = BufReader::new(input_file);

    // Create the first temporary file
    let mut temporary_file1 =
        File::create("temporary_file1.txt").expect("Failed to Create [ Temporary File 1 ]");

    // Message - Paragraph Size
    println!(
        "Paragraph Size... [ {} Sentences ]",
        &args.sentences_per_paragraph
    );

    // Clean the text of Time Stamps and Other Transcription Notation
    // and Produce a temporary file containing the disjointed text
    clean_notation(input_reader, &mut temporary_file1);

    // Merge all sentences into one line, then write to a second temporary file
    // Read Temporary File 2 into a vector, Chunk vector by Paragraph Size,
    // Write to Final Output
    create_paragraphs(args.sentences_per_paragraph, args.output);

    // Remove the Temporary Files
    let _ = Command::new("rm")
        .arg("temporary_file1.txt")
        .arg("temporary_file2.txt")
        .spawn();
}

fn clean_notation<R>(input_reader: BufReader<R>, temporary_file1: &mut File)
where
    R: std::io::Read,
{
    // Detect what's happening in the lines
    let start_detect = Regex::new(r"^Detect(.*)").unwrap(); // Detect: Starts with "Detect" -> Skip
    let start_transcription = Regex::new(r"^Transcription(.*)").unwrap(); // Detect: Starts with "Transcription" -> Skip
    let start_chapter = Regex::new(r"^Ch(.*)").unwrap(); // Detect: Starts with "Chapter" -> Skip
    let start_bracket = Regex::new(r"\[(.*?)\]  ").unwrap(); // Detect: Contains the time stamps "[time to time]" -> Transform -> Write

    // For each line in the input file...
    for line in input_reader.lines() {
        // Unwrap the line
        let line = line.unwrap();

        // Match the line to a Regex pattern, process appropriately, and write to Temporary File 1
        if start_detect.is_match(&line) {
            let cleaned_detect = start_detect.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{cleaned_detect}");
        } else if start_transcription.is_match(&line) {
            let cleaned_transcription = start_transcription.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{cleaned_transcription}");
        } else if start_chapter.is_match(&line) {
            let cleaned_chapter = start_chapter.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{cleaned_chapter}");
        } else if start_bracket.is_match(&line) {
            let cleaned_bracket = start_bracket.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{cleaned_bracket}");
        } else {
            {}
        }
    }
}

fn create_paragraphs(spp: u8, output_file: String) {
    // Read Temporary File 1
    let temporary_file1 = File::open("temporary_file1.txt").unwrap();
    let temporary_reader1 = BufReader::new(temporary_file1);

    // Trim input lines and push each line to the master string
    let mut string_base = String::new();
    for line in temporary_reader1.lines() {
        let trimmed_line = line.unwrap();
        string_base.push_str(trimmed_line.trim());
        string_base.push(' ');
    }

    // Replace all question marks with Question Marks + New Lines
    let question_lines = string_base.replace("? ", "?\n");

    // Replace all exclamation points with Exclamation Point + New Lines
    let exclamation_lines = question_lines.replace("! ", "!\n");

    // Replace all periods with Periods + New Lines
    let sentence_lines = exclamation_lines.replace(". ", ".\n");

    // Write to Temporary File 2
    let temporary_path2 = Path::new("temporary_file2.txt");
    let mut temporary_file2 =
        File::create(temporary_path2).expect("Failed to Create [ Temporary File ]");
    let _ = writeln!(temporary_file2, "{sentence_lines}");

    // Read Temporary File 2
    let temporary_in = File::open("temporary_file2.txt").unwrap();
    let temporary_reader2 = BufReader::new(temporary_in);

    // Push each full sentence in Temporary File 2 to a vector
    let mut line_vector: Vec<String> = Vec::new();
    for line in temporary_reader2.lines() {
        line_vector.push(line.unwrap());
    }

    // Group the vector into Paragraphs
    let paragraph_vector: Vec<Vec<String>> = line_vector
        .chunks(spp.into())
        .map(<[std::string::String]>::to_vec)
        .collect();

    // Write to the Final Output
    let mut final_output = File::create(output_file).expect("Failed to Create [ Output File ]");
    for paragraph in &paragraph_vector {
        let flattened_paragraph: String = paragraph.join(" ") + "\n";
        let _ = writeln!(final_output, "{flattened_paragraph}");
    }
}
