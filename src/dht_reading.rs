#[derive(Debug)]
pub struct DHTReading {
    temperature: f32, // represented internally in Celsius
    humidity: f32,    // represented internally as percentage
}
impl DHTReading {
    // constructor for DHTReading struct
    pub fn new(temperature: f32, humidity: f32) -> Self {
        Self {
            temperature,
            humidity,
        }
    }

    // getter for humidity
    pub fn humidity(&self) -> f32 {
        self.humidity
    }

    // getters for temperature
    pub fn temperature_celsius(&self) -> f32 {
        self.temperature
    }

    pub fn temperature_fahrenheit(&self) -> f32 {
        self.temperature * 9.0 / 5.0 + 32.0
    }
}