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
