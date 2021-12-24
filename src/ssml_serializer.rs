/// Very simple ssml serializer. Currently only supports single voice selection.
use crate::{types::VoiceGender, VoiceSettings};
use serde::Serialize;

const XML_VERSION: &str = "1.0";
const XMLNS_LINK: &str = "http://www.w3.org/2001/10/synthesis";
const XMLNS_MSTTS_LINK: &str = "https://www.w3.org/2001/mstts";

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "speak")]
pub struct Speak {
    // Needs decimal numbers
    version: &'static str,
    xmlns: &'static str,
    #[serde(rename = "xmlns:mstts")]
    xmlns_mstts: &'static str,
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
            version: XML_VERSION,
            xmlns: XMLNS_LINK,
            xmlns_mstts: XMLNS_MSTTS_LINK,
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
        let expected = "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\"><voice xml:lang=\"en-US\" xml:gender=\"Female\" name=\"en-US-JennyNeural\">lorem ipsum</voice></speak>";
        assert_eq!(expected, &ssml);
    }
}
