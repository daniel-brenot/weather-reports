use super::parser::weather_reports::*;
use super::remarks::metar_remarks::*;

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
        println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", val);
        let metar = visibility(val);
        println!("[PEG_TRACE_STOP]");
        metar.expect(val);
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
        println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", val);
        whitespace(val).expect(val);
        println!("[PEG_TRACE_STOP]");
    }
}

#[test]
fn parse_slp_remark() {
    println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", "RMK SLP250=");
    let remark = remarks("RMK SLP250=");
    println!("[PEG_TRACE_STOP]");
    assert_eq!(
        remark.unwrap().sea_level_pressure
            .expect("Failed to get sea level pressure").get::<uom::si::pressure::hectopascal>(),
        1025.0
    )
}