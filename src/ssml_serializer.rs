use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Speak {
    version: f32,
    #[serde(rename = "xml:lang")]
    xml_lang: String,
    voice: Voice,
}

impl Speak {
    pub fn with_single_voice(language: &str, gender: &str, voice_name: &str, text: &str) -> Self {
        let voice = Voice {
            xml_lang: language.to_owned(),
            xml_gender: gender.to_owned(),
            name: voice_name.to_owned(),
            body: text.to_owned(),
        };

        Self {
            version: 1.0,
            xml_lang: language.to_owned(),
            voice,
        }
    }

    pub fn to_ssml_xml(&self) -> String {
        quick_xml::se::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, PartialEq)]
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
        let speak =
            Speak::with_single_voice("en-US", "male", "voice_name", "Hi how are you doing?");

        let _xml = speak.to_ssml_xml();
        // assert_eq!("", &xml);
    }
}
