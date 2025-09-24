// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{command, AppHandle, Emitter};
use whisper_rs::{WhisperContext, FullParams, SamplingStrategy, WhisperContextParameters};
use hound;
use serde_json::json;

fn to_timestamp(t: i64, comma: bool) -> String {
    let mut msec = t * 10;
    let hr = msec / (1000 * 60 * 60);
    msec -= hr * (1000 * 60 * 60);
    let min = msec / (1000 * 60);
    msec -= min * (1000 * 60);
    let sec = msec / 1000;
    msec -= sec * 1000;

    format!(
        "{:02}:{:02}:{:02}{}{:03}",
        hr,
        min,
        sec,
        if comma { ',' } else { '.' },
        msec
    )
}

#[command]
async fn transcribe_audio(app: AppHandle, file_path: String) -> Result<String, String> {
    let model_path = "C:/Users/altho/source/repos/whisper.cpp/models/ggml-large-v3.bin".to_string();

    let wav_path = file_path;

    let mut context_param = WhisperContextParameters::default();

    // Enable DTW token level timestamp for known model by using model preset
    context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
        model_preset: whisper_rs::DtwModelPreset::LargeV3,
    };

    // Enable DTW token level timestamp for unknown model by providing custom aheads
    // see details https://github.com/ggerganov/whisper.cpp/pull/1485#discussion_r1519681143
    // values corresponds to ggml-base.en.bin, result will be the same as with DtwModelPreset::BaseEn
    let custom_aheads = [
        (7, 0), (10, 17), (12, 18), (13, 12), (16, 1), (17, 14), (19, 11), (21, 4), (24, 1), (25, 6)
    ]
    .map(|(n_text_layer, n_head)| whisper_rs::DtwAhead {
        n_text_layer,
        n_head,
    });
    context_param.dtw_parameters.mode = whisper_rs::DtwMode::Custom {
        aheads: &custom_aheads,
    };

    let ctx =
        WhisperContext::new_with_params(&model_path, context_param).expect("failed to load model");
    // Create a state
    let mut state = ctx.create_state().expect("failed to create state");

    // Create a params object for running the model.
    // The number of past samples to consider defaults to 0.
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });

    // Set the number of threads to use to 1.
    params.set_n_threads(1);
    // Enable translation.
    params.set_translate(true);
    // Set the language to translate to to English.
    params.set_language(Some("en"));
    // Disable anything that prints to stdout.
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    // Enable token level timestamps
    params.set_token_timestamps(true);

    // Open the audio file.
    let reader = hound::WavReader::open(wav_path).expect("failed to open file");
    #[allow(unused_variables)]
    let hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample,
        ..
    } = reader.spec();

    // Convert the audio to floating point samples.
    let samples: Vec<i16> = reader
        .into_samples::<i16>()
        .map(|x| x.expect("Invalid sample"))
        .collect();
    let mut audio = vec![0.0f32; samples.len().try_into().unwrap()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut audio).expect("Conversion error");

    // Convert audio to 16KHz mono f32 samples, as required by the model.
    // These utilities are provided for convenience, but can be replaced with custom conversion logic.
    if channels == 2 {
        audio = whisper_rs::convert_stereo_to_mono_audio(&audio).expect("Conversion error");
    } else if channels != 1 {
        panic!(">2 channels unsupported");
    }

    if sample_rate != 16000 {
        panic!("sample rate must be 16KHz");
    }

    // Run the model.
    state.full(params, &audio[..]).expect("failed to run model");

    // fetch the results
    let mut full_text = String::new();

    for segment in state.as_iter() {
        let segment_text = segment.to_string();
        full_text.push_str(&segment_text);
        full_text.push_str("\n");

        let _ = app.emit("transcript-segment", json!({
            "start": to_timestamp(segment.start_timestamp(), false),
            "end": to_timestamp(segment.end_timestamp(), false),
            "text": segment_text
        }));


        // println!(
        //     "[{} - {}]: {}",
        //     // these timestamps are in centiseconds (10s of milliseconds)
        //     segment.start_timestamp(),
        //     segment.end_timestamp(),
        //     // this default Display implementation will result in any invalid UTF-8
        //     // being converted into the Unicode replacement character, U+FFFD
        //     segment
        // );
    }

    Ok(full_text)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![transcribe_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
