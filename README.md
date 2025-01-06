# conversions

A simple CLI-based unit converter

Is mostly a rust port of [this file](https://github.com/sbrstrkkdwmdr/sbrbot/blob/main/src/vars/conversions.ts) from my discord bot

Some very large units (eg. light years) lose precision when converting from imperial units as they are converted to metric units first

### compiling

`rustc src/main.rs`

`./main.exe`

### Available conversions

| Type        | Units                                   |
| ----------- | --------------------------------------- |
| Angle       | xyz                                     |
| Area        | xyz                                     |
| Distance    | Inch, Foot, Metre, Mile, AU, LY, Parsec |
| Rnergy      | xyz                                     |
| Mass        | xyz                                     |
| Power       | xyz                                     |
| Pressure    | xyz                                     |
| Speed       | xyz                                     |
| Temperature | Celsius, Fahrenheit, Kelvin             |
| Time        | xyz                                     |
| Volume      | xyz                                     |
