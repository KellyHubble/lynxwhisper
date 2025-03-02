use whisper_rs::WhisperContext;

pub struct TranscriptionEngine {
    ctx: Box<WhisperContext>,
}

impl TranscriptionEngine {
    pub fn new(model_path: &str) -> Self {
        let ctx = Box::new(
            WhisperContext::new_with_params(model_path, Default::default())
                .expect("Failed to load model")
        );
        TranscriptionEngine { ctx }
    }

    pub fn transcribe(&self, audio: &[i16]) -> String {
        let audio_f32: Vec<f32> = audio.iter().map(|&sample| sample as f32 / 32768.0).collect();
        println!("Transcribing {} samples", audio_f32.len());
        let params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
        let mut state = self.ctx.create_state().expect("Failed to create state");
        match state.full(params, &audio_f32) {
            Ok(_) => {
                let num_segments = state.full_n_segments().expect("Failed to get segment count");
                let mut text = String::new();
                for i in 0..num_segments {
                    text.push_str(&state.full_get_segment_text(i).expect("Failed to get segment text"));
                }
                text
            }
            Err(e) => {
                println!("Transcription error: {:?}", e);
                String::new()
            }
        }
    }
}