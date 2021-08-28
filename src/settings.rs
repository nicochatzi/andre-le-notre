use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub irrigation: IrrigationSettings,
    pub stations: Vec<StationSettings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrrigationSettings {
    pub hour_of_day: u8,
    pub rain_threshold: f32,
    pub frequency: FrequencySettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrequencySettings {
    pub spring: u8,
    pub summer: u8,
    pub autumn: u8,
    pub winter: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationSettings {
    pub name: String,
    pub irrigation_length: u32,
    pub pressure_threshold: f32,
}

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Failed to open file : {0}")]
    FileIo(#[from] std::io::Error),

    #[error("Failed to deserialize : {0}")]
    Deserialize(#[from] serde_json::Error),
}

const SETTINGS_FILE: &str = "settings.json";

impl Settings {
    pub fn read() -> Self {
        Self::try_read().unwrap_or_default()
    }

    pub fn try_read() -> Result<Self, SettingsError> {
        Settings::from_file(SETTINGS_FILE)
    }

    pub fn irrigation_frequency() -> u8 {
        let f = Settings::read().irrigation.frequency;
        match Utc::now().month() {
            3..=5 => f.spring,
            6..=9 => f.summer,
            9..=12 => f.autumn,
            _ => f.winter,
        }
    }

    pub fn station(name: &str) -> StationSettings {
        Settings::read()
            .stations
            .iter()
            .find(|s| s.name == name)
            .unwrap()
            .clone()
    }

    fn from_file(filename: &str) -> Result<Self, SettingsError> {
        let settings = std::fs::read_to_string(filename)?;
        Ok(serde_json::from_str(&settings)?)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            irrigation: IrrigationSettings {
                hour_of_day: 7,
                rain_threshold: 80.0,
                frequency: FrequencySettings {
                    spring: 3,
                    summer: 2,
                    autumn: 2,
                    winter: 3,
                },
            },
            stations: vec![
                StationSettings {
                    name: "bottom".into(),
                    irrigation_length: 15,
                    pressure_threshold: 100.0,
                },
                StationSettings {
                    name: "mid".into(),
                    irrigation_length: 30,
                    pressure_threshold: 100.0,
                },
                StationSettings {
                    name: "top".into(),
                    irrigation_length: 45,
                    pressure_threshold: 100.0,
                },
                StationSettings {
                    name: "garden".into(),
                    irrigation_length: 60,
                    pressure_threshold: 100.,
                },
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_valid_settings_file() {
        let settings = Settings::from_file("tests/fixtures/settings/valid.json").unwrap();
        assert_eq!(settings.irrigation.hour_of_day, 14);
        assert_eq!(settings.irrigation.rain_threshold, 80.0);
        assert_eq!(
            settings.irrigation.frequency,
            FrequencySettings {
                spring: 3,
                summer: 1,
                autumn: 3,
                winter: 7,
            }
        );
        assert_eq!(settings.stations.len(), 4);
        assert_eq!(
            settings.stations[0],
            StationSettings {
                name: "bottom".into(),
                irrigation_length: 15,
                pressure_threshold: 100.0,
            }
        );
        assert_eq!(
            settings.stations[1],
            StationSettings {
                name: "top".into(),
                irrigation_length: 60,
                pressure_threshold: 40.0,
            }
        );
        assert_eq!(
            settings.stations[2],
            StationSettings {
                name: "mid".into(),
                irrigation_length: 30,
                pressure_threshold: 70.0,
            }
        );
        assert_eq!(
            settings.stations[3],
            StationSettings {
                name: "garden".into(),
                irrigation_length: 60,
                pressure_threshold: 120.0,
            }
        );
    }

    #[test]
    fn can_read_but_fail_to_deserialize_invalid_file() {
        assert!(matches!(
            Settings::from_file("tests/fixtures/settings/invalid.json").unwrap_err(),
            SettingsError::Deserialize(_)
        ))
    }

    #[test]
    fn fails_to_parse_file_that_does_not_exist() {
        assert!(matches!(
            Settings::from_file("does-not-exist.json").unwrap_err(),
            SettingsError::FileIo(_)
        ))
    }
}
