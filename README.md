# fan-ctrl-rs

An extremely simple application to control a PWM fan.
Even under full load, keeps the RPi below 60 ºC!

![image](https://github.com/arminveres/fan-ctrl-rs/assets/45210978/d745514a-bd87-4ff8-a0b9-fae1fbb8e1ae)

## Requirements

First of all PWM needs to be enabled on the RPi:

Add `dtoverlay=pwm,pin=18,func=2` (pwm/pwm-2chan docs) to `/boot/config.txt` to enable PWM Channel 0 on PIN 18.

Unsure, if it needs to be done once or if setup by RPPAL but I prompted the creation of `pwm0` by
running as sudo
```bash
echo 0 > /sys/class/pwm/pwmchip0/export
```

## TODOs

- [ ] Add support for other pins (currently Pin 18 is used)
- [ ] Add customizable fan curve
- [ ] Add TACH support, measuring incoming fan speed.

## Acknowledgements

- [RRPAL Examples](https://github.com/golemparts/rppal/blob/master/examples/pwm_blinkled.rs)
- [DriftKing Fan Control](https://github.com/DriftKingTW/Raspberry-Pi-PWM-Fan-Control)
- [This SO post](https://raspberrypi.stackexchange.com/questions/118235/can-i-control-a-4-pin-fan-with-a-raspberry-pi-without-adding-more-circuitry)
