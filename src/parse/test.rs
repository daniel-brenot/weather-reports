use super::parse::weather_reports::*;

#[test]
fn parse_icao_identifier() {
    for val in ["KSEA", "A302"] {
        icao_identifier(val).expect(val);
    }
}

#[test]
fn parse_observation_time() {
    for val in ["251453Z"] {
        observation_time(val).expect(val);
    }
}

#[test]
fn parse_wind() {
    for val in ["1804KT", "VRB04G19KT", "09015G25KT", "/////KT ///V///"] {
        wind(val).expect(val);
    }
}

#[test]
fn parse_prevailing_visibility() {
    for val in ["1/2SM ", "10SM "] {
        visibility(val).expect(val);
    }
}

#[test]
fn parse_runway_visibility() {
    for val in ["R40/3000FT", "R01L/3500VP6000FT", "R06/0600N", "R31///////"] {
        runway_visibility(val).expect(val);
    }
}

#[test]
fn parse_weather() {
    for val in ["-RA", "BR", "MIFG"] {
        weather(val).expect(val);
    }
}

#[test]
fn parse_cloud_cover() {
    for val in ["FEW025", "SCT250"] {
        cloud_cover(val).expect(val);
    }
}

#[test]
fn parse_temperatures() {
    for val in ["14/09", "24/M01", "14/"] {
        temperatures(val).expect(val);
    }
}

#[test]
fn parse_pressure() {
    for val in ["A3002"] {
        pressure(val).expect(val);
    }
}

#[test]
fn parse_water_conditions() {
    for val in ["W13/S3", "W13/S/", "W13/H10", "W///S3", "W13/H//"] {
        water_conditions(val).expect(val);
    }
}

#[test]
fn parse_color() {
    for val in ["WHT", "BLACKWHT", "WHT BLU"] {
        color(val).expect(val);
    }
}

#[test]
fn parse_whitespace() {
    for val in [" ///// ", " > ", "\t", "\r\n\r\n", " > /// \n> "] {
        whitespace(val).expect(val);
    }
}