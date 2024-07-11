// Represents a duration in seconds as a floating-point number (f64). 
// This struct is designed to hold the age of an entity in seconds.
pub struct Duration(f64);

const SECONDS_IN_A_YEAR: f64 = 31557600.0;

// This is an implementation of the From trait for the Duration struct, 
// allowing a u64 integer (representing seconds) to be converted into 
// a Duration instance. This is a convenience for creating 
// Duration instances from literal numbers.
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

// common interface for all planets
pub trait Planet {
    // representing the length of a year on the planet relative to Earth's year. 
    const ORBITAL_PERIOD: f64;

    // returns the number of years that have passed on the planet, 
    // given a duration in seconds.
    fn years_during(d: &Duration) -> f64 {
        (d.0 / SECONDS_IN_A_YEAR) / Self::ORBITAL_PERIOD
    }
}
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

// Each planet struct has an implementation of the Planet trait, 
// specifying the ORBITAL_PERIOD for that planet. 
// This period is a floating-point number representing how long 
// a year on that planet lasts compared to a year on Earth.

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;
}
