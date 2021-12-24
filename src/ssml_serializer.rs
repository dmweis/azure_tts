/// Very simple ssml serializer. Currently only supports single voice selection.
use crate::{types::VoiceGender, Style, VoiceSettings};
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
    pub fn new(language: &str, voice: Voice) -> Self {
        Self {
            version: XML_VERSION,
            xmlns: XMLNS_LINK,
            xmlns_mstts: XMLNS_MSTTS_LINK,
            xml_lang: language.to_owned(),
            voice,
        }
    }

    pub fn with_text(language: &str, gender: VoiceGender, voice_name: &str, text: &str) -> Self {
        let voice = Voice {
            xml_lang: language.to_owned(),
            xml_gender: gender.as_string().to_owned(),
            name: voice_name.to_owned(),
            body: vec![VoiceSegment::Plain(text.to_owned())],
        };
        Self::new(language, voice)
    }

    pub fn with_segments(
        language: &str,
        gender: VoiceGender,
        voice_name: &str,
        contents: Vec<VoiceSegment>,
    ) -> Self {
        let voice = Voice {
            xml_lang: language.to_owned(),
            xml_gender: gender.as_string().to_owned(),
            name: voice_name.to_owned(),
            body: contents,
        };
        Self::new(language, voice)
    }

    pub fn text_with_voice_settings(voice_settings: &VoiceSettings, text: &str) -> Self {
        let voice = Voice {
            xml_lang: voice_settings.language.to_owned(),
            xml_gender: voice_settings.gender.as_string().to_owned(),
            name: voice_settings.name.to_owned(),
            body: vec![VoiceSegment::Plain(text.to_owned())],
        };
        Speak::new(&voice_settings.language.to_owned(), voice)
    }

    pub fn segments_with_voice_settings(
        voice_settings: &VoiceSettings,
        contents: Vec<VoiceSegment>,
    ) -> Self {
        let voice = Voice {
            xml_lang: voice_settings.language.to_owned(),
            xml_gender: voice_settings.gender.as_string().to_owned(),
            name: voice_settings.name.to_owned(),
            body: contents,
        };
        Speak::new(&voice_settings.language.to_owned(), voice)
    }

    pub fn to_ssml_xml(&self) -> String {
        let xml = quick_xml::se::to_string(self).expect("Failed to serialize ssml");
        // TODO: This is a weird hack
        // Either there is a bug in the library
        // Or more likely in my code
        xml.replace("$value", "mstts:express-as")
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
    body: Vec<VoiceSegment>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum VoiceSegment {
    Plain(String),
    ExpressAs(ExpressAs),
}

impl VoiceSegment {
    pub fn plain(text: &str) -> Self {
        VoiceSegment::Plain(text.to_owned())
    }

    pub fn with_expression(text: &str, style: Style) -> Self {
        let express_as = ExpressAs {
            style,
            body: text.to_owned(),
        };
        VoiceSegment::ExpressAs(express_as)
    }
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "mstts:express-as")]
pub struct ExpressAs {
    #[serde(rename = "style")]
    style: Style,
    #[serde(rename = "$value")]
    body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_serialization_plain_text() {
        let speak = Speak::with_text(
            "en-US",
            VoiceGender::Female,
            "en-US-JennyNeural",
            "lorem ipsum",
        );

        let ssml = speak.to_ssml_xml();
        let expected = "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" \
xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\">\
<voice xml:lang=\"en-US\" xml:gender=\"Female\" name=\"en-US-JennyNeural\">\
lorem ipsum\
</voice>\
</speak>";
        assert_eq!(expected, &ssml);
    }

    #[test]
    fn xml_serialization_express_as_single() {
        let speak = Speak::with_segments(
            "en-US",
            VoiceGender::Female,
            "en-US-SaraNeural",
            vec![VoiceSegment::with_expression(
                "lorem ipsum",
                Style::Cheerful,
            )],
        );

        let ssml = speak.to_ssml_xml();
        let expected = "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" \
xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\">\
<voice xml:lang=\"en-US\" xml:gender=\"Female\" name=\"en-US-SaraNeural\">\
<mstts:express-as style=\"cheerful\">lorem ipsum</mstts:express-as>\
</voice>\
</speak>";
        assert_eq!(expected, &ssml);
    }

    #[test]
    fn xml_serialization_express_as_multiple() {
        let speak = Speak::with_segments(
            "en-US",
            VoiceGender::Female,
            "en-US-SaraNeural",
            vec![
                VoiceSegment::with_expression("lorem ipsum", Style::Cheerful),
                VoiceSegment::with_expression("lorem ipsum", Style::Sad),
            ],
        );

        let ssml = speak.to_ssml_xml();
        let expected = "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" \
xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\">\
<voice xml:lang=\"en-US\" xml:gender=\"Female\" name=\"en-US-SaraNeural\">\
<mstts:express-as style=\"cheerful\">lorem ipsum</mstts:express-as>\
<mstts:express-as style=\"sad\">lorem ipsum</mstts:express-as>\
</voice>\
</speak>";
        assert_eq!(expected, &ssml);
    }

    #[test]
    fn xml_serialization_plain_and_express_as() {
        let speak = Speak::with_segments(
            "en-US",
            VoiceGender::Female,
            "en-US-SaraNeural",
            vec![
                VoiceSegment::plain("lorem ipsum"),
                VoiceSegment::with_expression("lorem ipsum", Style::Cheerful),
            ],
        );

        let ssml = speak.to_ssml_xml();
        let expected = "<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" \
xmlns:mstts=\"https://www.w3.org/2001/mstts\" xml:lang=\"en-US\">\
<voice xml:lang=\"en-US\" xml:gender=\"Female\" name=\"en-US-SaraNeural\">\
lorem ipsum\
<mstts:express-as style=\"cheerful\">lorem ipsum</mstts:express-as>\
</voice>\
</speak>";
        assert_eq!(expected, &ssml);
    }

    #[test]
    fn express_as_serialization() {
        let express = ExpressAs {
            style: Style::Angry,
            body: String::from("lorem"),
        };
        let xml = quick_xml::se::to_string(&express).unwrap();
        let expected = "<mstts:express-as style=\"angry\">lorem</mstts:express-as>";
        assert_eq!(expected, &xml);
    }
}
