#[derive(Deserialize, PartialEq, Debug)]
struct GoogleLocationHistory {
    locations: Vec<Location>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    #[serde(deserialize_with = "i64_string::deserialize")]
    timestamp_ms: i64,
    latitude_e7: i64,
    longitude_e7: i64,
    accuracy: u16,
    activitys: Option<Vec<TimestampedActivity>>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct TimestampedActivity {
    #[serde(deserialize_with = "i64_string::deserialize")]
    timestamp_ms: i64,
    activities: Vec<Activity>,
    extras: Option<Vec<Extra>>,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Activity {
    #[serde(rename = "type")]
    activity_type: String,
    confidence: u16,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Extra {
    #[serde(rename = "type")]
    extra_type: String,
    name: String,
    #[serde(rename = "intVal")]
    int_val: u8,
}

mod i64_string {
    use serde::{de, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
        where D: Deserializer<'de> {
        String::deserialize(deserializer)?.parse::<i64>().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    #[test]
    fn can_deserialize_a_google_location_history_file() {
        let s = r##"
            {
                "locations" : [ {
                    "timestampMs" : "1498358433377",
                    "latitudeE7" : 5207967334,
                    "longitudeE7" : 11965831,
                    "accuracy" : 18,
                    "activitys" : [ {
                      "timestampMs" : "1498358433377",
                      "activities" : [ {
                        "type" : "still",
                        "confidence" : 100
                      } ],
                      "extras" : [ {
                        "type" : "value",
                        "name" : "vehicle_personal_confidence",
                        "intVal" : 100
                      } ]
                    } ]
                }, {
                    "timestampMs" : "1498358433377",
                    "latitudeE7" : 5207967334,
                    "longitudeE7" : 11965831,
                    "accuracy" : 18,
                    "activitys" : [ {
                      "timestampMs" : "1498358433377",
                      "activities" : [ {
                        "type" : "still",
                        "confidence" : 100
                      } ]
                    } ]
                }, {
                    "timestampMs" : "1493657963571",
                    "latitudeE7" : 5205674674,
                    "longitudeE7" : 11485831,
                    "accuracy" : 18
                } ]
            }
        "##;
        let glh: GoogleLocationHistory = serde_json::from_str(s).unwrap();

        assert_eq!(glh, GoogleLocationHistory {
            locations: vec![Location {
                timestamp_ms: 1498358433377,
                latitude_e7: 5207967334,
                longitude_e7: 11965831,
                accuracy: 18,
                activitys: Some(vec![TimestampedActivity {
                    timestamp_ms: 1498358433377,
                    activities: vec![Activity {
                        activity_type: "still".to_string(),
                        confidence: 100,
                    }],
                    extras: Some(vec![Extra {
                        extra_type: "value".to_string(),
                        name: "vehicle_personal_confidence".to_string(),
                        int_val: 100,
                    }]),
                }]),
            }, Location {
                timestamp_ms: 1498358433377,
                latitude_e7: 5207967334,
                longitude_e7: 11965831,
                accuracy: 18,
                activitys: Some(vec![TimestampedActivity {
                    timestamp_ms: 1498358433377,
                    activities: vec![Activity {
                        activity_type: "still".to_string(),
                        confidence: 100,
                    }],
                    extras: None,
                }]),
            }, Location {
                timestamp_ms: 1493657963571,
                latitude_e7: 5205674674,
                longitude_e7: 11485831,
                accuracy: 18,
                activitys: None,
            }]
        });
    }
}
