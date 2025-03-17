#[derive(Debug)]
pub struct SensorReading {
    temperature: f32, // represented internally in Celsius
    humidity: f32,    // represented internally as percentage
}

impl SensorReading {
    // constructor for DHTReading struct
    pub fn new(temperature: f32, humidity: f32) -> Self {
        Self {
            temperature,
            humidity,
        }
    }

    // getter for humidity
    // returns the humidity as a percentage; e.g. 60.0 for 60%.
    // applicable values are 0% to 100%.
    pub fn humidity(&self) -> f32 {
        self.humidity
    }

    // getter for temperature value, in Celsius
    // applicable values are -40C to 80C.
    pub fn temperature_celsius(&self) -> f32 {
        self.temperature
    }

    // getter for temperature value, in Fahrenheit
    // applicable values are -40C to 80C.
    pub fn temperature_fahrenheit(&self) -> f32 {
        self.temperature * 9.0 / 5.0 + 32.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dht_reading() {
        let reading = SensorReading::new(25.0, 60.0);
        assert_eq!(reading.temperature_celsius(), 25.0);
        assert_eq!(reading.temperature_fahrenheit(), 77.0);
        assert_eq!(reading.humidity(), 60.0);
    }
}
