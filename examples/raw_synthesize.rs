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
    let mut client = azure_tts::VoiceService::new(subscription_key, azure_tts::Region::uksouth);
    let text = "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\"
    xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\">
    <voice name=\"en-US-SaraNeural\">
    this is normal
        <mstts:express-as style=\"cheerful\">
            And this is cheerful
        </mstts:express-as>
        <mstts:express-as style=\"sad\">
            And this is sad
        </mstts:express-as>
        <mstts:express-as style=\"angry\">
            And this is angry
        </mstts:express-as>
    </voice>
</speak>";

    let res = client
        .synthesize_raw_text(
            text.to_owned(),
            azure_tts::AudioFormat::Audio48khz192kbitrateMonoMp3,
        )
        .await
        .unwrap();
    println!("Response length:\n{}", res.len());
    write_bytes_to_file(&res, "output.mp3");
}
