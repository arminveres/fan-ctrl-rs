/// Adapted from: https://github.com/DriftKingTW/Raspberry-Pi-PWM-Fan-Control
/// and from the RPPAL examples
use std::{error::Error, fs, thread, time::Duration};

use rppal::pwm::{Channel, Polarity, Pwm};

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
// gain curve, this is linear now
const FAN_GAIN: f64 = (FAN_HIGH - FAN_LOW) / (MAX_TEMP - MIN_TEMP);

fn get_cpu_temp() -> f64 {
    let temp_file = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .expect("Thermal zone was not found");
    let mut temp_val = temp_file.trim().parse::<f64>().expect("Didn't parse temp");
    // Convert to Degress from Millidegrees
    temp_val /= 1000.;
    println!("Temperature at : {temp_val:.1} C");
    temp_val
}

fn main() -> Result<(), Box<dyn Error>> {
    let pwm = Pwm::with_frequency(
        Channel::Pwm0,
        PWM_FREQ,
        PWM_DUTY_CYCLE,
        Polarity::Normal,
        true,
    )?;

    loop {
        let temp = get_cpu_temp();
        let mut dc = 0.;
        if temp > MIN_TEMP {
            let delta = temp.min(MAX_TEMP) - MIN_TEMP;
            dc = FAN_LOW + delta * FAN_GAIN;
        } else if temp < OFF_TEMP {
            dc = FAN_OFF;
        }
        println!("Setting duty cycle to: {dc:.2}!");
        pwm.set_duty_cycle(dc)?;

        // Sleep for 1 seconds while for lower usage
        thread::sleep(Duration::from_secs(1));
    }
}
