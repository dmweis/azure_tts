use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct VoiceService {
    service_region: String,
    subscription_key: String,
    access_token: String,
    https_client: reqwest::Client,
}

impl VoiceService {
    pub fn new(subscription_key: &str, service_region: &str) -> Self {
        let https_client = reqwest::Client::new();
        // make optional or query immediately
        let access_token = String::from("");
        Self {
            service_region: service_region.to_owned(),
            subscription_key: subscription_key.to_owned(),
            access_token,
            https_client,
        }
    }

    pub async fn list_voices(&self) -> Result<Vec<VoiceDescription>> {
        let region_host_name = format!("{}.tts.speech.microsoft.com", self.service_region);
        let endpoint = format!("https://{}/cognitiveservices/voices/list", region_host_name);
        let bearer_token = format!("Bearer: {}", self.access_token);

        // this can auth using either access token or sub key
        let response = self
            .https_client
            .get(endpoint)
            // .header("Ocp-Apim-Subscription-Key", &self.subscription_key)
            .header("Authorization", bearer_token)
            .header("Host", region_host_name)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn convert(&self, text: String, audio_format: AudioFormat) -> Result<bytes::Bytes> {
        let endpoint = format!(
            "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
            self.service_region
        );
        let bearer_token = format!("Bearer: {}", self.access_token);

        // this can auth using either access token or sub key
        let response = self
            .https_client
            .post(endpoint)
            // .header("Ocp-Apim-Subscription-Key", &self.subscription_key)
            .header("Authorization", bearer_token)
            .header("X-Microsoft-OutputFormat", audio_format.as_string())
            .header("Content-Type", "application/ssml+xml")
            .header("User-Agent", "rust-azure-tts-client-lib")
            .body(text)
            .send()
            .await?;
        let res_code = response.status();
        println!("Res code: {}", res_code);
        let audio = response.bytes().await?;
        Ok(audio)
    }

    pub async fn update_auth_token(&mut self) -> Result<()> {
        let region_host_name = format!("{}.api.cognitive.microsoft.com", self.service_region);
        let endpoint = format!("https://{}/sts/v1.0/issuetoken", region_host_name);
        let response = self
            .https_client
            .post(endpoint)
            .header("Ocp-Apim-Subscription-Key", &self.subscription_key)
            .header("Host", region_host_name)
            .header("Content-type", "application/x-www-form-urlencoded")
            .header("Content-Length", "0")
            .send()
            .await?
            .text()
            .await?;
        self.access_token = response;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoiceGender {
    // TODO: Other values?
    #[serde(rename = "Male")]
    Male,
    #[serde(rename = "Female")]
    Female,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VoiceDescription {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "DisplayName")]
    pub display_name: String,
    #[serde(alias = "LocalName")]
    pub local_name: String,
    #[serde(alias = "ShortName")]
    pub short_name: String,
    #[serde(alias = "Gender")]
    pub gender: VoiceGender,
    #[serde(alias = "Locale")]
    pub locale: String,
    #[serde(alias = "SecondaryLocaleList")]
    pub secondary_locale_list: Option<Vec<String>>,
    #[serde(alias = "StyleList")]
    pub style_list: Option<Vec<String>>,
    #[serde(alias = "SampleRateHertz")]
    // TODO: Why is this a string?
    pub sample_rate_hertz: String,
    #[serde(alias = "VoiceType")]
    pub voice_type: String,
    #[serde(alias = "Status")]
    pub status: String,
}

pub enum AudioFormat {
    audio_16khz_128kbitrate_mono_mp3,
    audio_16khz_32kbitrate_mono_mp3,
    audio_16khz_64kbitrate_mono_mp3,
    audio_24khz_160kbitrate_mono_mp3,
    audio_24khz_48kbitrate_mono_mp3,
    audio_24khz_96kbitrate_mono_mp3,
    audio_48khz_192kbitrate_mono_mp3,
    audio_48khz_96kbitrate_mono_mp3,
    ogg_16khz_16bit_mono_opus,
    ogg_24khz_16bit_mono_opus,
    ogg_48khz_16bit_mono_opus,
    raw_16khz_16bit_mono_pcm,
    raw_16khz_16bit_mono_truesilk,
    raw_24khz_16bit_mono_pcm,
    raw_24khz_16bit_mono_truesilk,
    raw_48khz_16bit_mono_pcm,
    raw_8khz_8bit_mono_alaw,
    raw_8khz_8bit_mono_mulaw,
    riff_16khz_16bit_mono_pcm,
    riff_24khz_16bit_mono_pcm,
    riff_48khz_16bit_mono_pcm,
    riff_8khz_8bit_mono_alaw,
    riff_8khz_8bit_mono_mulaw,
    webm_16khz_16bit_mono_opus,
    webm_24khz_16bit_mono_opus,
}

impl AudioFormat {
    fn as_string(&self) -> String {
        match self {
            AudioFormat::audio_16khz_128kbitrate_mono_mp3 => {
                String::from("audio-16khz-128kbitrate-mono-mp3")
            }
            AudioFormat::audio_16khz_32kbitrate_mono_mp3 => {
                String::from("audio-16khz-32kbitrate-mono-mp3")
            }
            AudioFormat::audio_16khz_64kbitrate_mono_mp3 => {
                String::from("audio-16khz-64kbitrate-mono-mp3")
            }
            AudioFormat::audio_24khz_160kbitrate_mono_mp3 => {
                String::from("audio-24khz-160kbitrate-mono-mp3")
            }
            AudioFormat::audio_24khz_48kbitrate_mono_mp3 => {
                String::from("audio-24khz-48kbitrate-mono-mp3")
            }
            AudioFormat::audio_24khz_96kbitrate_mono_mp3 => {
                String::from("audio-24khz-96kbitrate-mono-mp3")
            }
            AudioFormat::audio_48khz_192kbitrate_mono_mp3 => {
                String::from("audio-48khz-192kbitrate-mono-mp3")
            }
            AudioFormat::audio_48khz_96kbitrate_mono_mp3 => {
                String::from("audio-48khz-96kbitrate-mono-mp3")
            }
            AudioFormat::ogg_16khz_16bit_mono_opus => String::from("ogg-16khz-16bit-mono-opus"),
            AudioFormat::ogg_24khz_16bit_mono_opus => String::from("ogg-24khz-16bit-mono-opus"),
            AudioFormat::ogg_48khz_16bit_mono_opus => String::from("ogg-48khz-16bit-mono-opus"),
            AudioFormat::raw_16khz_16bit_mono_pcm => String::from("raw-16khz-16bit-mono-pcm"),
            AudioFormat::raw_16khz_16bit_mono_truesilk => {
                String::from("raw-16khz-16bit-mono-truesilk")
            }
            AudioFormat::raw_24khz_16bit_mono_pcm => String::from("raw-24khz-16bit-mono-pcm"),
            AudioFormat::raw_24khz_16bit_mono_truesilk => {
                String::from("raw-24khz-16bit-mono-truesilk")
            }
            AudioFormat::raw_48khz_16bit_mono_pcm => String::from("raw-48khz-16bit-mono-pcm"),
            AudioFormat::raw_8khz_8bit_mono_alaw => String::from("raw-8khz-8bit-mono-alaw"),
            AudioFormat::raw_8khz_8bit_mono_mulaw => String::from("raw-8khz-8bit-mono-mulaw"),
            AudioFormat::riff_16khz_16bit_mono_pcm => String::from("riff-16khz-16bit-mono-pcm"),
            AudioFormat::riff_24khz_16bit_mono_pcm => String::from("riff-24khz-16bit-mono-pcm"),
            AudioFormat::riff_48khz_16bit_mono_pcm => String::from("riff-48khz-16bit-mono-pcm"),
            AudioFormat::riff_8khz_8bit_mono_alaw => String::from("riff-8khz-8bit-mono-alaw"),
            AudioFormat::riff_8khz_8bit_mono_mulaw => String::from("riff-8khz-8bit-mono-mulaw"),
            AudioFormat::webm_16khz_16bit_mono_opus => String::from("webm-16khz-16bit-mono-opus"),
            AudioFormat::webm_24khz_16bit_mono_opus => String::from("webm-24khz-16bit-mono-opus"),
        }
    }
}
