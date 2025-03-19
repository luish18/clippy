// This is the entry point of the application.
// It initializes the application, sets up the necessary components for audio recording, 
// file organization, transcription, and summarization.

mod audio;
mod organizer;
mod transcription;
mod summarization;
mod utils;

use audio::recorder::AudioRecorder;
use organizer::FileOrganizer;
use transcription::Transcriber;
use summarization::Summarizer;

fn main() {
    // Initialize components
    let audio_recorder = AudioRecorder::new();
    let file_organizer = FileOrganizer::new();
    let transcriber = Transcriber::new();
    let summarizer = Summarizer::new();

    // Application logic goes here
    // Example: Start recording audio
    audio_recorder.start_recording();
    
    // After recording, stop and save the audio
    audio_recorder.stop_recording();
    let file_path = audio_recorder.save_recording("output.wav");

    // Organize the recorded file
    file_organizer.create_folder("Recordings");
    file_organizer.move_file(&file_path, "Recordings/output.wav");

    // Transcribe the audio file
    let transcription = transcriber.transcribe_audio("Recordings/output.wav");

    // Summarize the transcribed text
    let summary = summarizer.summarize_text(&transcription);

    // Output the summary
    println!("Summary: {}", summary);
}