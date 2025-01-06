# conversions

A simple CLI-based unit converter

Is mostly a rust port of [this file](https://github.com/sbrstrkkdwmdr/sbrbot/blob/main/src/vars/conversions.ts) from my discord bot

Some very large units (eg. light years) lose precision when converting from imperial units as they are converted to metric units first

months have a value of 30.437 days and years are 365.25 days

### compiling

`rustc src/main.rs`

`./main.exe`

### Available conversions

| Type        | Units                                                                                                  |
| ----------- | ------------------------------------------------------------------------------------------------------ |
| Angle       | xyz                                                                                                    |
| Area        | xyz                                                                                                    |
| Distance    | Inch, Foot, Metre, Mile, AU, LY, Parsec                                                                |
| Rnergy      | Electron-Volt, Joule, Calorie, BTU, Watt-Hour                                                          |
| Mass        | Gram, Ounce, Pound, Stone, US Ton, Metric Tonne                                                        |
| Power       | Erg, Watt, Decibel-milliwatt, foot-pounds per minute, Calories per second, Horse power, BTU per second |
| Pressure    | Pascal, mmHg, PSI, Bar, Standard Atmosphere                                                            |
| Speed       | xyz                                                                                                    |
| Temperature | Celsius, Fahrenheit, Kelvin                                                                            |
| Time        | Second, Minute, Hour, Day, Week, Fortnight, Month, Quarantine, Year, Decade, Century, Megaannum, Eon   |
| Volume      | Teaspoon, Tablespoon, Fluid Ounce, Cup, Pint, Litre, Gallon, Cubic Metre                               |
