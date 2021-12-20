use std::env;

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
}
