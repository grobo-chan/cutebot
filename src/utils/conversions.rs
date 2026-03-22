pub mod celsius {
    pub fn to_fahrenheit(value: f32) -> f32 {
        return value * 1.8 + 32.0;
    }
    pub fn to_kelvin(value: f32) -> f32 {
        return value + 273.15;
    }
    pub fn to_rankine(value: f32) -> f32 {
        return value * 9.0 / 5.0 + 491.67;
    }
    pub fn to_reaumur(value: f32) -> f32 {
        return value / 1.25;
    }
}

pub mod fahrenheit {
    pub fn to_celsius(value: f32) -> f32 {
        return ((value - 32.0) * 5.0) / 9.0;
    }
    pub fn to_kelvin(value: f32) -> f32 {
        return (((value - 32.0) * 5.0) / 9.0) + 273.15;
    }
    pub fn to_rankine(value: f32) -> f32 {
        return value + 459.67;
    }
    pub fn to_reaumur(value: f32) -> f32 {
        return (value - 32.0) / 2.25;
    }
}

pub mod kelvin {
    pub fn to_celsius(value: f32) -> f32 {
        return value - 273.15;
    }
    pub fn to_fahrenheit(value: f32) -> f32 {
        return ((value - 273.15) * 1.8) + 32.0;
    }
    pub fn to_rankine(value: f32) -> f32 {
        return value * 1.8;
    }
    pub fn to_reaumur(value: f32) -> f32 {
        return (value - 273.15) / 1.25;
    }
}

pub mod rankine {
    pub fn to_celsius(value: f32) -> f32 {
        return (value - 491.67) * 5.0 / 9.0;
    }
    pub fn to_fahrenheit(value: f32) -> f32 {
        return value - 459.67;
    }
    pub fn to_kelvin(value: f32) -> f32 {
        return value / 1.8;
    }
    pub fn to_reaumur(value: f32) -> f32 {
        return (value - 491.67) / 2.25;
    }
}

pub mod reaumur {
    pub fn to_kelvin(value: f32) -> f32 {
        return value * 1.25 + 273.15;
    }
    pub fn to_celsius(value: f32) -> f32 {
        return value * 1.25;
    }
    pub fn to_fahrenheit(value: f32) -> f32 {
        return value * 2.25 + 32.0;
    }
    pub fn to_rankine(value: f32) -> f32 {
        return value * 2.25 + 491.67;
    }
}
