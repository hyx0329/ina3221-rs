# INA3221 driver

Use with embedded-hal 1.0.

This is a minimum implementation.

IN3221 supports 4 different i2c addresses. Use [`Ina3221::new_with_addr`] for addresses other than 0x40.

What's implemented:

- power mode(off/oneshot/continuous)
- all 3 channel readings
- averaging mode
- (not yet)voltage conversion time
- current calculation
- chip ID reading

## Usage

```rust,ignore
let mut voltmon = Ina3221::new(SharedI2cBus::new(mutex_i2c_bus))
    // optionally set the shunt resistor values
    // all defaults to 10 milli-ohms
    .shunt_r1(100)
    .shunt_r2(10)
    .shunt_r3(100);

println!("Channel1 voltage: {}mV", voltmon.bus_channel1().unwrap())
println!("Channel2 current: {}mA", voltmon.current_channel2().unwrap())
```
