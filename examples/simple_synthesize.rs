use std::env;
use std::fs::File;
use std::io::prelude::*;

fn write_bytes_to_file(data: &[u8], path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
    file.flush().unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let subscription_key = args
        .get(1)
        .expect("Please provide subscription key as argument");
    let mut client = azure_tts::VoiceService::new(subscription_key, "uksouth");
    let text = "Microsoft Speech Service Text-to-Speech API";
    let res = client
        .synthesize(
            text,
            &azure_tts::VoiceSettings::default_female_jenny(),
            azure_tts::AudioFormat::Audio48khz192kbitrateMonoMp3,
        )
        .await
        .unwrap();
    println!("Response length:\n{}", res.len());
    write_bytes_to_file(&res, "output.mp3");
}
