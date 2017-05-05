
use serde_xml_rs::deserialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct KML {
    #[serde(rename = "Document")]
    pub document: Document,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Document {
    #[serde(rename = "Placemark")]
    pub placemark: Placemark,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Placemark {
    pub open: String,

    #[serde(rename = "Track")]
    pub track: gx::Track,
}

mod gx {
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Track {
    #[serde(rename = "altitudeMode")]
    pub altitude_mode: String,

    #[serde(rename = "when", default)]
    pub when: Vec<String>,

    #[serde(rename = "coord", default)]
    pub coord: Vec<String>,
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    // This test currently fails because serde-xml-rs doesn't like the alternating <when> and
    // <gx:coord> elements. <https://github.com/RReverser/serde-xml-rs/issues/11>
    fn it_works() {
        let s = r##"
            <?xml version='1.0' encoding='UTF-8'?>
            <kml xmlns='http://www.opengis.net/kml/2.2' xmlns:gx='http://www.google.com/kml/ext/2.2'>
            <Document>
                <Placemark>
                    <open>1</open>
                    <gx:Track>
                        <altitudeMode>clampToGround</altitudeMode>
                        <when>2017-01-01T17:54:27Z</when>
                        <gx:coord>1.4542331 59.0545834 0</gx:coord>
                        <when>2017-01-01T17:51:27Z</when>
                        <gx:coord>1.4542331 59.0545834 0</gx:coord>
                    </gx:Track>
                </Placemark>
            </Document>
            </kml>
        "##;
        let kml: KML = deserialize(s.as_bytes()).unwrap();
        println!("{:#?}", kml);
    }
}
