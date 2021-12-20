use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let subscription_key = args
        .get(1)
        .expect("Please provide subscription key as argument");
    let mut client = azure_tts::VoiceService::new(subscription_key, "uksouth");
    client.update_auth_token().await.unwrap();
    // let voices = client.list_voices().await.unwrap();
    let ssml = "<speak version='1.0' xml:lang='en-US'><voice xml:lang='en-US' xml:gender='Male'
    name='en-US-ChristopherNeural'>
        Microsoft Speech Service Text-to-Speech API
</voice></speak>";
    let _res = client
        .convert(
            ssml.to_owned(),
            azure_tts::AudioFormat::Audio48khz192kbitrateMonoMp3,
        )
        .await
        .unwrap();
    // println!("res:\n{:?}", res);
}
