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
    #[arg(short, long)]
    sentences_per_paragraph: u8,
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(&args.input).unwrap();
    let input_reader = BufReader::new(&input_file);
    let mut temporary_file1 =
        File::create("temporary_file1.txt").expect("Failed to Create [ Temporary File 1 ]");

    // Message - Paragraph Size Provided
    println!(
        "Paragraph Size Provided... [ {} Sentences ]",
        &args.sentences_per_paragraph
    );

    clean_notation(input_reader, &mut temporary_file1);

    create_paragraphs(args.sentences_per_paragraph);

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

    for line in input_reader.lines() {
        let line = line.unwrap();

        if start_detect.is_match(&line) {
            let cleaned_detect = start_detect.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{}", cleaned_detect);
        } else if start_transcription.is_match(&line) {
            let cleaned_transcription = start_transcription.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{}", cleaned_transcription);
        } else if start_chapter.is_match(&line) {
            let cleaned_chapter = start_chapter.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{}", cleaned_chapter);
        } else if start_bracket.is_match(&line) {
            let cleaned_bracket = start_bracket.replace_all(&line, "");
            let _ = writeln!(temporary_file1, "{}", cleaned_bracket);
        } else {
            {}
        }
    }
}

fn create_paragraphs(spp: u8) {
    // Read the input file
    let temporary_file1 = File::open("temporary_file1.txt").unwrap();
    let temporary_reader1 = BufReader::new(temporary_file1);

    // Trim input lines and push each line to the master string
    let mut string_base = "".to_string();
    for line in temporary_reader1.lines() {
        let trimmed_line = line.unwrap();
        string_base.push_str(trimmed_line.trim());
        string_base.push(' ');
    }

    // Replace all periods with Periods + New Lines
    let sentence_lines = string_base.replace(". ", ".\n");

    // Write to a Temporary File
    let temporary_path2 = Path::new("temporary_file2.txt");
    let mut temporary_file2 =
        File::create(temporary_path2).expect("Failed to Create [ Temporary File ]");
    let _ = writeln!(temporary_file2, "{}", sentence_lines);

    // Read Temporary File
    let temporary_in = File::open("temporary_file2.txt").unwrap();
    let temporary_reader2 = BufReader::new(temporary_in);

    // Push each full sentence in the temporary file to a vector
    let mut line_vector: Vec<String> = Vec::new();
    for line in temporary_reader2.lines() {
        line_vector.push(line.unwrap());
    }

    // Group the vector into Paragraphs
    let sentences_per_paragraph = spp; // Number of sentences per paragraph
    let paragraph_vector: Vec<Vec<String>> = line_vector
        .chunks(sentences_per_paragraph.into())
        .map(|paragraph| paragraph.to_vec())
        .collect();

    // Write to the Final Output
    let mut final_output =
        File::create("cleaned_transcription.txt").expect("Failed to Create [ Output File ]");
    for paragraph in &paragraph_vector {
        let flattened_paragraph: String = paragraph.join(" ") + "\n";
        let _ = writeln!(final_output, "{}", flattened_paragraph);
    }
}
