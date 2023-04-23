// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECONDS: f64 = 31_557_600.0;
const EARTH_YEARS: f64 = 1.0;
const MERCURY_YEARS: f64 = 0.2408467;
const VENUS_YEARS: f64 = 0.61519726;
const MARS_YEARS: f64 = 1.8808158;
const JUPITER_YEARS: f64 = 11.862615;
const SATURN_YEARS: f64 = 29.447498;
const URANUS_YEARS: f64 = 84.016846;
const NEPTUNE_YEARS: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration{
    duration: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            duration: s
        }
    }
}

pub trait Planet {
    const EARTH_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / (EARTH_SECONDS * Self::EARTH_YEARS)
    }
}

/*pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;*/

macro_rules! create_planet {
    ($name:ident, $earth_years:expr) => {
        pub struct $name;
        impl Planet for $name {
            const EARTH_YEARS: f64 = $earth_years;
        }
    };
}

create_planet!(Mercury, MERCURY_YEARS);
create_planet!(Venus, VENUS_YEARS);
create_planet!(Earth, EARTH_YEARS);
create_planet!(Mars, MARS_YEARS);
create_planet!(Jupiter, JUPITER_YEARS);
create_planet!(Saturn, SATURN_YEARS);
create_planet!(Uranus, URANUS_YEARS);
create_planet!(Neptune, NEPTUNE_YEARS);

/*impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.years / MERCURY_YEARS
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.years / VENUS_YEARS
    }
}
impl Planet for Earth {}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.years / MARS_YEARS
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.years / JUPITER_YEARS
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.years / SATURN_YEARS
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.years / URANUS_YEARS
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.years / NEPTUNE_YEARS
    }
}*/
