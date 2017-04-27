
use serde_json;

#[derive(Deserialize, PartialEq, Debug)]
struct GoogleLocationHistory {
    locations: Vec<Location>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    timestamp_ms: String,
    latitude_e7: i64,
    longitude_e7: i64,
    accuracy: u16,
    activitys: Option<Vec<TimestampedActivity>>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct TimestampedActivity {
    timestamp_ms: String,
    activities: Vec<Activity>,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Activity {
    #[serde(rename = "type")]
    activity_type: String,
    confidence: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

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
                timestamp_ms: "1498358433377".to_string(),
                latitude_e7: 5207967334,
                longitude_e7: 11965831,
                accuracy: 18,
                activitys: Some(vec![TimestampedActivity {
                    timestamp_ms: "1498358433377".to_string(),
                    activities: vec![Activity {
                        activity_type: "still".to_string(),
                        confidence: 100,
                    }]
                }]),
            }, Location {
                timestamp_ms: "1493657963571".to_string(),
                latitude_e7: 5205674674,
                longitude_e7: 11485831,
                accuracy: 18,
                activitys: None,
            }]
        });
    }
}
