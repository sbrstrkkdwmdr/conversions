# conversions

A simple CLI-based unit converter

Is mostly a rust port of [this file](https://github.com/sbrstrkkdwmdr/sbrbot/blob/main/src/vars/conversions.ts) from my discord bot

Some very large units (eg. light years) lose precision when converting from imperial units as they are converted to metric units first

months have a value of 30.437 days and years are 365.25 days

### how to use

`./conversions.exe [args]`

| arg | type   | description                              | example      |
| --- | ------ | ---------------------------------------- | ------------ |
| -i  | string | input unit                               | -i kilometre |
| -o  | string | output unit                              | -o ft        |
| -v  | float  | the value to convert                     | -v 727       |
| -l  | bool   | list all [units](#available-conversions) |              |
| --v | bool   | return app version                       |              |

eg. </br>
`./conversions.exe -i m -o ft -v 42` (metres to feet)</br>
`./conversions.exe -i c -o f -v 25` (celsius to fahrenheit)</br>

Most units have aliases (eg. m for metres) which can be seen [here](https://sbrstrkkdwmdr.github.io/projects/conversions)

### compiling

`rustc src/main.rs --crate-name "conversions"`

`./conversions.exe [args]`

### Available conversions

| Type        | Units                                                                                                  |
| ----------- | ------------------------------------------------------------------------------------------------------ |
| Angle       | Gradian, Degree, Radian                                                                                |
| Area        | Square Inch , Square Foot, Square Metre, Acre, Hectare, Square Kilometre, Square mile                  |
| Distance    | Inch, Foot, Metre, Mile, AU, LY, Parsec                                                                |
| Rnergy      | Electron-Volt, Joule, Calorie, BTU, Watt-Hour                                                          |
| Mass        | Gram, Ounce, Pound, Stone, US Ton, Metric Tonne                                                        |
| Power       | Erg, Watt, Decibel-milliwatt, foot-pounds per minute, Calories per second, Horse power, BTU per second |
| Pressure    | Pascal, mmHg, PSI, Bar, Standard Atmosphere                                                            |
| Speed       | km/h, mp/h, kt, m/s, Light                                                                             |
| Temperature | Celsius, Fahrenheit, Kelvin                                                                            |
| Time        | Second, Minute, Hour, Day, Week, Fortnight, Month, Quarantine, Year, Decade, Century, Megaannum, Eon   |
| Volume      | Teaspoon, Tablespoon, Fluid Ounce, Cup, Pint, Litre, Gallon, Cubic Metre                               |
