#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    {% case board -%}
      {%- when "Adafruit Trinket" -%}
    let mut led = pins.d1.into_output();
      {%- when "Arduino Leonardo", "Arduino Mega 2560", "Arduino Nano", "Arduino Uno", "Nano168" -%}
    let mut led = pins.d13.into_output();
      {%- when "SparkFun ProMicro" -%}
    let mut led = pins.led_rx.into_output();
    {%- endcase %}

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
{%- comment %}
# vim: ft=rust.jinja2
{% endcomment %}
