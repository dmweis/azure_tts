use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let subscription_key = args
        .get(1)
        .expect("Please provide subscription key as argument");
    let mut client = azure_tts::VoiceService::new(subscription_key, azure_tts::Region::uksouth);
    let voices = client.list_voices().await.unwrap();
    for voice in voices {
        if voice.locale == "en-US" {
            println!(
                "short_name: {}\ndisplay_name: {}\nlocale: {}\ngender: {:?}\n",
                voice.short_name, voice.display_name, voice.locale, voice.gender
            );
        }
    }
}
