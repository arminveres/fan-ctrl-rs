use std::{error::Error, fs, thread, time::Duration};

use rppal::pwm::{Channel, Polarity, Pwm};

/// Adjusted from: https://github.com/DriftKingTW/Raspberry-Pi-PWM-Fan-Control

const PWM_FREQ: f64 = 25_000.;
const PWM_DUTY_CYCLE: f64 = 0.5;

/// [°C] temperature below which to stop the fan
const OFF_TEMP: f64 = 40.;
/// [°C] temperature above which to start the fan
const MIN_TEMP: f64 = 45.;
/// [°C] temperature at which to operate at max fan speed
const MAX_TEMP: f64 = 70.;

const FAN_LOW: f64 = 0.1;
const FAN_HIGH: f64 = 1.;
const FAN_OFF: f64 = 0.;
// const FAN_MAX: f64 = 1.;
const FAN_GAIN: f64 = (FAN_HIGH - FAN_LOW) / (MAX_TEMP - MIN_TEMP);

fn get_cpu_temp() -> f64 {
    let temp_file = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .expect("Thermal zone was not found");
    let mut temp_val = temp_file.trim().parse::<f64>().expect("Didn't parse temp");
    temp_val /= 1000.;
    temp_val
}

// fn handle_temp() {
// }

fn main() -> Result<(), Box<dyn Error>> {
    let pwm = Pwm::with_frequency(
        Channel::Pwm0,
        PWM_FREQ,
        PWM_DUTY_CYCLE,
        Polarity::Normal,
        true,
    )?;

    loop {
        // Sleep for 1 seconds while for lower usage
        let temp = get_cpu_temp();
        if temp > MIN_TEMP {
            let delta = temp.min(MAX_TEMP) - MIN_TEMP;
            pwm.set_duty_cycle(FAN_LOW + delta * FAN_GAIN)?;
        } else if temp < OFF_TEMP {
            pwm.set_duty_cycle(FAN_OFF)?;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
