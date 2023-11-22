mod batch_tests;

/// Testing when a pilot report remark occurs before a normal remark
#[test]
fn validate_pirep_remark_and_regular_remark() {
    crate::parse::metar(
        "KTDF 202145Z AUTO 09004KT 10SM CLR 13/M01 A3029 RM AO1 8 RMK AO2 T0123101K AO2 T01520052="
    ).unwrap();
}

/// Testing when a runway visibility occurs after a runway report
#[test]
fn validate_runway_visibility_post_runway_report() {
    crate::parse::metar(
        "METAR UNNT 211630Z 08002MPS CAVOK M14/M16 Q1043 R07/810150 R16/////// NOSIG RMK QFE772/1030="
    ).unwrap();
}

#[test]
fn validate_remark_sea_level_pressure() {
    let metar = crate::parse::metar(
        "EDMO 210420Z AUTO 27007KT 9999 // SCT025/// BKN041/// OVC052/// 06/04 Q1011 RMK SLP250="
    ).unwrap();
    dbg!(metar.clone());
    assert_eq!(
        metar.remarks.expect("Failed to get metar remarks")
        .sea_level_pressure.expect("Failed to get sea level pressure")
        .get::<uom::si::pressure::hectopascal>(), 250.0)
}

// The following are real world samples that failed for one reason or another and are used as sanity checks

#[test]
fn validate_sample_1() {
    crate::parse::metar(
        "EDMO 210420Z AUTO 27007KT 9999 // SCT025/// BKN041/// OVC052/// 06/04 Q1011="
    ).unwrap();
}

// #[test]06
// fn validate_sample_2() {
//     crate::parse::metar(
//         "YBBN 211630Z 04007KT 9999 FEW016 BKN050 23/20 Q1018 RF00/0/002/4="
//     ).unwrap();
// }
