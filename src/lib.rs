mod error;
mod ssml_serializer;
mod types;

use std::time::{Duration, Instant};

use bytes::Buf;
pub use error::TtsError;
pub use ssml_serializer::Speak;
pub use types::*;

type Result<T> = std::result::Result<T, TtsError>;

pub struct VoiceService {
    service_region: Region,
    subscription_key: String,
    access_token: String,
    access_toke_time: Instant,
    https_client: reqwest::Client,
}

/// Timeout is 10 minutes
const ACCESS_TOKEN_TIMEOUT: Duration = Duration::from_secs(60 * 9);

impl VoiceService {
    pub fn new(subscription_key: &str, service_region: Region) -> Self {
        let https_client = reqwest::Client::new();
        // make optional or query immediately
        let access_token = String::from("");
        Self {
            service_region,
            subscription_key: subscription_key.to_owned(),
            access_token,
            access_toke_time: Instant::now() - ACCESS_TOKEN_TIMEOUT,
            https_client,
        }
    }

    pub async fn list_voices(&mut self) -> Result<Vec<VoiceDescription>> {
        self.renew_token_if_expired().await?;
        let region_host_name = format!(
            "{}.tts.speech.microsoft.com",
            self.service_region.as_string()
        );
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

    pub async fn synthesize_raw_text(
        &mut self,
        text: String,
        audio_format: AudioFormat,
    ) -> Result<Vec<u8>> {
        self.renew_token_if_expired().await?;
        let endpoint = format!(
            "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
            self.service_region.as_string()
        );
        let bearer_token = format!("Bearer: {}", self.access_token);

        let response = self
            .https_client
            .post(endpoint)
            .header("Authorization", bearer_token)
            .header("X-Microsoft-OutputFormat", audio_format.as_string())
            .header("Content-Type", "application/ssml+xml")
            .header("User-Agent", "rust-azure-tts-client-lib")
            .body(text)
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => (),
            reqwest::StatusCode::BAD_REQUEST => return Err(TtsError::BadRequest),
            reqwest::StatusCode::UNAUTHORIZED => return Err(TtsError::AuthError),
            reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE => {
                return Err(TtsError::UnsupportedMediaType)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => return Err(TtsError::TooManyRequest),
            _ => return Err(TtsError::UnknownConnectionError),
        }
        let audio = response.bytes().await?;
        Ok(audio.chunk().to_vec())
    }

    pub async fn synthesize(
        &mut self,
        text: &str,
        voice: &VoiceSettings,
        audio_format: AudioFormat,
    ) -> Result<Vec<u8>> {
        self.renew_token_if_expired().await?;
        let endpoint = format!(
            "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
            self.service_region.as_string()
        );
        let bearer_token = format!("Bearer: {}", self.access_token);

        let response = self
            .https_client
            .post(endpoint)
            .header("Authorization", bearer_token)
            .header("X-Microsoft-OutputFormat", audio_format.as_string())
            .header("Content-Type", "application/ssml+xml")
            .header("User-Agent", "rust-azure-tts-client-lib")
            .body(Speak::with_voice_settings(voice, text).to_ssml_xml())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::OK => (),
            reqwest::StatusCode::BAD_REQUEST => return Err(TtsError::BadRequest),
            reqwest::StatusCode::UNAUTHORIZED => return Err(TtsError::AuthError),
            reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE => {
                return Err(TtsError::UnsupportedMediaType)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => return Err(TtsError::TooManyRequest),
            _ => return Err(TtsError::UnknownConnectionError),
        }
        let audio = response.bytes().await?;
        Ok(audio.chunk().to_vec())
    }

    pub async fn update_auth_token(&mut self) -> Result<()> {
        let region_host_name = format!(
            "{}.api.cognitive.microsoft.com",
            self.service_region.as_string()
        );
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
        self.access_toke_time = Instant::now();
        Ok(())
    }

    async fn renew_token_if_expired(&mut self) -> Result<()> {
        if self.access_toke_time.elapsed() > ACCESS_TOKEN_TIMEOUT {
            self.update_auth_token().await?;
        }
        Ok(())
    }
}
