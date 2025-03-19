pub struct Transcriber {
    model: Option<whisper::Model>,
}

impl Transcriber {
    pub fn new() -> Self {
        Transcriber { model: None }
    }

    pub fn load_whisper_model(&mut self, model_path: &str) -> Result<(), String> {
        self.model = whisper::load_model(model_path).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn transcribe_audio(&self, audio_path: &str) -> Result<String, String> {
        if let Some(model) = &self.model {
            model.transcribe(audio_path).map_err(|e| e.to_string())
        } else {
            Err("Whisper model not loaded".to_string())
        }
    }
}