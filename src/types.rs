use serde::{Deserialize, Serialize};

pub struct VoiceSettings {
    pub name: String,
    pub language: String,
    pub gender: VoiceGender,
}

impl VoiceSettings {
    pub fn new(name: &str, language: &str, gender: VoiceGender) -> Self {
        Self {
            name: name.to_owned(),
            language: language.to_owned(),
            gender,
        }
    }

    pub fn default_female_jenny() -> Self {
        Self {
            name: String::from("en-US-JennyNeural"),
            language: String::from("en-US"),
            gender: VoiceGender::Female,
        }
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

impl VoiceGender {
    pub(crate) fn as_string(&self) -> &'static str {
        match self {
            VoiceGender::Female => "Female",
            VoiceGender::Male => "Male",
        }
    }
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

impl VoiceDescription {
    pub fn to_voice_settings(&self) -> VoiceSettings {
        VoiceSettings::new(&self.short_name, &self.local_name, self.gender)
    }
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
    pub(crate) fn as_string(&self) -> &'static str {
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

#[allow(non_camel_case_types)]
pub enum Region {
    centralus,
    eastus,
    eastus2,
    northcentralus,
    southcentralus,
    westcentralus,
    westus,
    westus2,
    canadacentral,
    brazilsouth,
    eastasia,
    southeastasia,
    australiaeast,
    centralindia,
    japaneast,
    japanwest,
    koreacentral,
    northeurope,
    westeurope,
    francecentral,
    switzerlandnorth,
    uksouth,
}

impl Region {
    pub(crate) fn as_string(&self) -> &'static str {
        match self {
            Region::centralus => "centralus",
            Region::eastus => "eastus",
            Region::eastus2 => "eastus2",
            Region::northcentralus => "northcentralus",
            Region::southcentralus => "southcentralus",
            Region::westcentralus => "westcentralus",
            Region::westus => "westus",
            Region::westus2 => "westus2",
            Region::canadacentral => "canadacentral",
            Region::brazilsouth => "brazilsouth",
            Region::eastasia => "eastasia",
            Region::southeastasia => "southeastasia",
            Region::australiaeast => "australiaeast",
            Region::centralindia => "centralindia",
            Region::japaneast => "japaneast",
            Region::japanwest => "japanwest",
            Region::koreacentral => "koreacentral",
            Region::northeurope => "northeurope",
            Region::westeurope => "westeurope",
            Region::francecentral => "francecentral",
            Region::switzerlandnorth => "switzerlandnorth",
            Region::uksouth => "uksouth",
        }
    }
}
