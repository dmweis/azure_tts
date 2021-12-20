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
    Audio16khz128kbitrateMonoMp3,
    Audio16khz32kbitrateMonoMp3,
    Audio16khz64kbitrateMonoMp3,
    Audio24khz160kbitrateMonoMp3,
    Audio24khz48kbitrateMonoMp3,
    Audio24khz96kbitrateMonoMp3,
    Audio48khz192kbitrateMonoMp3,
    Audio48khz96kbitrateMonoMp3,
    Ogg16khz16bitMonoOpus,
    Ogg24khz16bitMonoOpus,
    Ogg48khz16bitMonoOpus,
    Raw16khz16bitMonoPcm,
    Raw16khz16bitMonoTruesilk,
    Raw24khz16bitMonoPcm,
    Raw24khz16bitMonoTruesilk,
    Raw48khz16bitMonoPcm,
    Raw8khz8bitMonoAlaw,
    Raw8khz8bitMonoMulct,
    Riff16khz16bitMonoPcm,
    Riff24khz16bitMonoPcm,
    Riff48khz16bitMonoPcm,
    Riff8khz8bitMonoAlaw,
    Riff8khz8bitMonoMulaw,
    Webm16khz16bitMonoOpus,
    Webm24khz16bitMonoOpus,
}

impl AudioFormat {
    fn as_string(&self) -> &'static str {
        match self {
            AudioFormat::Audio16khz128kbitrateMonoMp3 => "audio-16khz-128kbitrate-mono-mp3",
            AudioFormat::Audio16khz32kbitrateMonoMp3 => "audio-16khz-32kbitrate-mono-mp3",
            AudioFormat::Audio16khz64kbitrateMonoMp3 => "audio-16khz-64kbitrate-mono-mp3",
            AudioFormat::Audio24khz160kbitrateMonoMp3 => "audio-24khz-160kbitrate-mono-mp3",
            AudioFormat::Audio24khz48kbitrateMonoMp3 => "audio-24khz-48kbitrate-mono-mp3",
            AudioFormat::Audio24khz96kbitrateMonoMp3 => "audio-24khz-96kbitrate-mono-mp3",
            AudioFormat::Audio48khz192kbitrateMonoMp3 => "audio-48khz-192kbitrate-mono-mp3",
            AudioFormat::Audio48khz96kbitrateMonoMp3 => "audio-48khz-96kbitrate-mono-mp3",
            AudioFormat::Ogg16khz16bitMonoOpus => "ogg-16khz-16bit-mono-opus",
            AudioFormat::Ogg24khz16bitMonoOpus => "ogg-24khz-16bit-mono-opus",
            AudioFormat::Ogg48khz16bitMonoOpus => "ogg-48khz-16bit-mono-opus",
            AudioFormat::Raw16khz16bitMonoPcm => "raw-16khz-16bit-mono-pcm",
            AudioFormat::Raw16khz16bitMonoTruesilk => "raw-16khz-16bit-mono-truesilk",
            AudioFormat::Raw24khz16bitMonoPcm => "raw-24khz-16bit-mono-pcm",
            AudioFormat::Raw24khz16bitMonoTruesilk => "raw-24khz-16bit-mono-truesilk",
            AudioFormat::Raw48khz16bitMonoPcm => "raw-48khz-16bit-mono-pcm",
            AudioFormat::Raw8khz8bitMonoAlaw => "raw-8khz-8bit-mono-alaw",
            AudioFormat::Raw8khz8bitMonoMulct => "raw-8khz-8bit-mono-mulaw",
            AudioFormat::Riff16khz16bitMonoPcm => "riff-16khz-16bit-mono-pcm",
            AudioFormat::Riff24khz16bitMonoPcm => "riff-24khz-16bit-mono-pcm",
            AudioFormat::Riff48khz16bitMonoPcm => "riff-48khz-16bit-mono-pcm",
            AudioFormat::Riff8khz8bitMonoAlaw => "riff-8khz-8bit-mono-alaw",
            AudioFormat::Riff8khz8bitMonoMulaw => "riff-8khz-8bit-mono-mulaw",
            AudioFormat::Webm16khz16bitMonoOpus => "webm-16khz-16bit-mono-opus",
            AudioFormat::Webm24khz16bitMonoOpus => "webm-24khz-16bit-mono-opus",
        }
    }
}
