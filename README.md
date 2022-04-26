[![Rust](https://github.com/valeriyr/weather/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/valeriyr/weather/actions/workflows/rust.yml)

# weather v0.1.0 by Valerii Reutov

Could be the best weather provider but just a simple application for the Rust language training...

Supported providers:

* [OpenWeather](https://openweathermap.org)
* [weatherapi](https://www.weatherapi.com)

# How to build

```bash
cargo build
```

# Usage

```bash
weather configure <provider>
weather get <city> [date=NOW]
```
