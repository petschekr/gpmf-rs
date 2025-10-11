use time::Duration;

use crate::{DataType, DeviceName, FourCC, Gpmf, SensorType, Stream};

use super::{SensorField, OrientationField, Orientation, SensorQuantifier};

#[derive(Debug, Clone)]
pub enum Field {
    Sensor(SensorField),
    Orientation(OrientationField),
}

/// Sensor data from a single `DEVC` stream:
/// - Accelerometer, fields are acceleration (m/s2).
/// - Gyroscope, fields are rotation (rad/s).
/// - Gravity vector, fields are direction of gravity in relation to camera angle.
#[derive(Debug, Default, Clone)]
pub struct SensorData {
    /// Camera device name
    pub device: DeviceName,
    /// Accelerometer, gyroscope, gravimeter
    pub sensor: SensorType,
    /// Units
    pub units: Option<String>,
    /// Physical quantity
    pub quantifier: SensorQuantifier,
    /// Total samples delivered so far
    pub total: u32,
    /// Sensor orientation
    pub orientation: Orientation,
    pub fields: Vec<Field>,
    /// Timestamp relative to video start.
    pub timestamp: Option<Duration>,
    /// Duration in video.
    pub duration: Option<Duration>,
}

impl SensorData {
    /// Parse sensor data from given `Stream`.
    pub fn new(devc_stream: &Stream, sensor: &SensorType, device: &DeviceName) -> Option<Self> {
        // Scale, should only be a single value for Gyro
        let scale = *devc_stream
            .find(&FourCC::SCAL)
            .and_then(|s| s.to_f64())?
            .first()?;

        // See https://github.com/gopro/gpmf-parser/issues/165#issuecomment-1207241564
        let orientation_str: Option<String> = devc_stream
            .find(&FourCC::ORIN)
            .and_then(|s| s.first_value())
            .and_then(|s| s.into());

        let orientation = orientation_str
            .map(|s| Orientation::from(s.as_str()))
            .unwrap_or(Orientation::XZY);

        let units: Option<String> = devc_stream
            .find(&FourCC::SIUN)
            .and_then(|s| s.first_value())
            .and_then(|s| s.into());

        // let orientation = match orientation_str {
        //     Some(orin) => Orientation::from(orin.as_str()),
        //     // None => Orientation::ZXY
        //     // Changed to XZY: https://github.com/gopro/gpmf-parser/issues/170#issuecomment-1322414755
        //     None => Orientation::XZY
        // };

        let total: u32 = devc_stream
            .find(&FourCC::TSMP)
            .and_then(|s| s.first_value())
            .and_then(|s| s.into())?;

        // Set FourCC for raw data arrays
        let sensor_fourcc = match &sensor {
            SensorType::Accelerometer => FourCC::ACCL,
            SensorType::Gyroscope => FourCC::GYRO,
            SensorType::GravityVector => FourCC::GRAV,
            SensorType::CameraOrientation => FourCC::CORI,
            SensorType::ImageOrientation => FourCC::IORI,
            SensorType::Unknown => return None
        };

        let sensor_quantifier = SensorQuantifier::from(sensor);

        // Vec containing x, y, z values
        let fields = devc_stream.find(&sensor_fourcc)
            .and_then(|val| val.to_vec_f64())? // each contained vec should have exactly 3 values for 3D sensor data
            .iter()
            .filter_map(|values| {
                match &sensor {
                    SensorType::Accelerometer
                    | SensorType::Gyroscope
                    | SensorType::GravityVector
                    => SensorField::new(&values, scale, &orientation)
                        .map(|f| Field::Sensor(f)),
                    SensorType::CameraOrientation
                    | SensorType::ImageOrientation
                    => OrientationField::new(&values, scale)
                        .map(|f| Field::Orientation(f)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        Some(Self{
            device: device.to_owned(),
            sensor: sensor.to_owned(),
            units,
            quantifier:sensor_quantifier,
            total,
            orientation,
            fields,
            timestamp: devc_stream.time_relative(),
            duration: devc_stream.time_duration()
        })
    }

    pub fn from_gpmf(gpmf: &Gpmf, sensor: &SensorType) -> Vec<Self> {
        let device_name: Vec<DeviceName> = gpmf.device_name()
            .iter()
            // .map(|n| DeviceName::from_str(n))
            .filter_map(|n| match DeviceName::from_str(n) {
                DeviceName::Unknown => None,
                name => Some(name)
            })
            .collect();
        // Get camera device name (listed first if GPMF from Karma drone)
        // to get data type (free text data identifier is model dependent)
        if let Some(device) = device_name.first() {
            let data_type = sensor.as_datatype(device);

            let sensor_data_streams = gpmf.filter(&data_type);

            return sensor_data_streams.iter()
                .filter_map(|stream| Self::new(stream, sensor, device))
                .collect::<Vec<Self>>()
        }

        // Failure to determine device name returns empty vec
        Vec::new()
    }

    pub fn as_datatype(&self) -> DataType {
        self.sensor.as_datatype(&self.device)
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }
}

/// Returns the linear mean value.
fn mean_value(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

/// Returns the median value.
fn median_value(values: &[f64]) {

}
