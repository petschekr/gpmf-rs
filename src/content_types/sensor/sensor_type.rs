use std::fmt::Display;

use crate::{DataType, DeviceName};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    Accelerometer,
    GravityVector,
    Gyroscope,
    CameraOrientation,
    ImageOrientation,
    Unknown
}

impl Default for SensorType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for SensorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorType::Accelerometer => write!(f, "Accelerometer"),
            SensorType::GravityVector => write!(f, "Gravity Vector"),
            SensorType::Gyroscope => write!(f, "Gyroscope"),
            SensorType::CameraOrientation => write!(f, "Camera Orientation"),
            SensorType::ImageOrientation => write!(f, "Image Orientation"),
            SensorType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<&str> for SensorType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "acc" | "accl" | "accelerometer" => Self::Accelerometer,
            "grv" | "grav" | "gravityvector" | "gravity vector" => Self::GravityVector,
            "gyr" | "gyro" | "gyroscope" => Self::Gyroscope,
            "cori" => Self::CameraOrientation,
            "iori" => Self::ImageOrientation,
            _ => Self::Unknown
        }
    }
}

impl SensorType {
    /// Convert `SensorType` to `DataType`
    pub fn as_datatype(&self, device: &DeviceName) -> DataType {
        match &self {
            Self::Accelerometer => match device {
                DeviceName::Hero5Black | DeviceName::Hero6Black => DataType::AccelerometerUrf,
                _ => DataType::Accelerometer
            }
            Self::GravityVector => DataType::GravityVector,
            Self::Gyroscope => match device {
                DeviceName::Hero5Black | DeviceName::Hero6Black => DataType::GyroscopeZxy,
                _ => DataType::Gyroscope
            },
            Self::CameraOrientation => DataType::CameraOrientation,
            Self::ImageOrientation => DataType::ImageOrientation,
            Self::Unknown => DataType::Other("Unkown".to_owned())
        }
    }

    /// Convert `DataType` to `SensorType`
    pub fn from_datatype(data_type: &DataType) -> Self {
        match &data_type {
            DataType::Accelerometer | DataType::AccelerometerUrf => Self::Accelerometer,
            DataType::GravityVector => Self::GravityVector,
            DataType::Gyroscope | DataType::GyroscopeZxy => Self::Gyroscope,
            DataType::CameraOrientation => Self::CameraOrientation,
            DataType::ImageOrientation => Self::ImageOrientation,
            _ => Self::Unknown
        }
    }

    pub fn units(&self) -> &str {
        match &self {
            SensorType::Accelerometer => "m/s²",
            SensorType::GravityVector => "N/A",
            SensorType::Gyroscope => "rad/s",
            _ => "N/A",
        }
    }

    pub fn quantifier(&self) -> &str {
        match &self {
            SensorType::Accelerometer => "Acceleration",
            SensorType::GravityVector => "N/A",
            SensorType::Gyroscope => "Rotation",
            _ => "N/A",
        }
    }
}
