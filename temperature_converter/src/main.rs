// Exercise from Rust docs Chapter 3
// Convert temperatures between Fahrenheit and Celsius.

use std::fmt;
use std::io;

enum TemperatureScale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f64,
    scale: TemperatureScale,
}

impl Temperature {
    fn convert(&self) -> Temperature {
        match self.scale {
            TemperatureScale::Celsius => Temperature {
                degrees: convert_c_to_f(self.degrees),
                scale: TemperatureScale::Fahrenheit,
            },
            TemperatureScale::Fahrenheit => Temperature {
                degrees: convert_f_to_c(self.degrees),
                scale: TemperatureScale::Celsius,
            },
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let unit = if let TemperatureScale::Celsius = self.scale {
            "C"
        } else {
            "F"
        };

        write!(f, "{:.2} {}", self.degrees, unit)
    }
}

fn main() {
    let mut is_active = true;

    while is_active {
        let degrees = get_degrees();
        let scale = set_temperature_scale();

        let temperature = Temperature { degrees, scale };

        println!("Entered temperature: {}", temperature);
        println!("Converted temperature: {}", temperature.convert());

        is_active = get_active_status();
    }
    println!("Buhbye~");
}

fn get_active_status() -> bool {
    let mut is_active = String::new();
    println!("Convert another temperature? Y / [N]");

    io::stdin()
        .read_line(&mut is_active)
        .expect("Failed to read line");

    let is_active = is_active.trim().to_lowercase();
    is_active.starts_with("y")
}

fn get_degrees() -> f64 {
    loop {
        let mut temperature = String::new();
        println!("Enter temperature: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        match temperature.trim().parse::<f64>() {
            Ok(temp) => break temp,
            Err(_) => {
                println!("Try again and enter a number.");
            }
        };
    }
}

fn set_temperature_scale() -> TemperatureScale {
    let celsius = "celsius";
    let fahrenheit = "fahrenheit";

    loop {
        let mut scale = String::new();
        println!("Enter temperature scale: C / F");

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale = scale.trim().to_lowercase();
        if celsius.starts_with(&scale) {
            break TemperatureScale::Celsius;
        } else if fahrenheit.starts_with(&scale) {
            break TemperatureScale::Fahrenheit;
        } else {
            println!("Input to set temperature scale not recognized.");
            println!("Try: 'C' or 'F'");
        }
    }
}

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

fn convert_c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
