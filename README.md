# Clean Speech-to-Text Transcriptions

## Use Case:
- The [Speech-to-Text pipeline](https://github.com/hedeenza/Speech-to-Text) is set to include time stamps and transcription details. While these details can be incredibly helpful for later reference, they do not offer the cleanest reading experience. This tool cleans up the transcript to only include the transcription (and file names for batch transcriptions), and organizes the text into paragraphs. There is functionality to set the number of sentences in each paragraph, should there be situations where shorter or longer paragraphs are desired.
- Currently the file names will be subsumed into the paragraphs. Manual editing is required to place them at the beginning of a paragraph / section.

## Get the Tool
- The pre-compiled binary (for Linux and Windows) and source code are available in "Releases".
- macOS users will need to compile from source.

## Running the CLI
`$ ./clean_stt_transcription -i <INPUT> -o <OUTPUT> -s <NUMBER>`

- `-i / --input`: Input file - The speech-to-text transcript to clean.
- `-o / --output`: Output file - The name of the clean output file.
- `-s / --sentences-per-paragraph`: Number of sentences per paragraph - The desired number of sentences in each paragraph (default 5).

- Ensure the program has executable permissions.

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Clean Speech to Text Transcriptions
export PATH="$PATH:/home/path/to/directory/where/this/program/lives"

alias cstt="clean_stt_transcription"
```

