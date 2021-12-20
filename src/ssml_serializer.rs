/// Very simple ssml serializer. Currently only supports single voice selection.
use crate::{types::VoiceGender, VoiceSettings};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "speak")]
pub struct Speak {
    // Needs decimal numbers
    version: &'static str,
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    voice: Voice,
}

impl Speak {
    pub fn new(language: &str, gender: VoiceGender, voice_name: &str, text: &str) -> Self {
        let voice = Voice {
            xml_lang: language.to_owned(),
            xml_gender: gender.as_string().to_owned(),
            name: voice_name.to_owned(),
            body: text.to_owned(),
        };
        Self {
            version: "1.0",
            xml_lang: language.to_owned(),
            voice,
        }
    }

    pub fn with_voice_settings(voice: &VoiceSettings, text: &str) -> Self {
        Speak::new(&voice.language, voice.gender, &voice.name, text)
    }

    pub fn to_ssml_xml(&self) -> String {
        quick_xml::se::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "voice")]
pub struct Voice {
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    #[serde(rename = "xml:gender")]
    xml_gender: String,
    name: String,
    #[serde(rename = "$value")]
    body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_serialization() {
        let speak = Speak::new(
            "en-US",
            VoiceGender::Female,
            "en-US-JennyNeural",
            "lorem ipsum",
        );

        let ssml = speak.to_ssml_xml();
        let expected = "<speak version=\"1.0\" xml:lang=\"en-US\"><voice xml:lang=\"en-US\" xml:gender=\"Female\" name=\"en-US-JennyNeural\">lorem ipsum</voice></speak>";
        assert_eq!(expected, &ssml);
    }
}
