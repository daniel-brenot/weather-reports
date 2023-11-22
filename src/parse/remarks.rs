use uom::si::f64::Pressure;
use uom::si::pressure::kilopascal;

#[derive(Clone, PartialEq, Debug)]
pub struct Remarks {
    pub sea_level_pressure: Option<Pressure>,
    pub unknown_remarks: Vec<String>
}

enum RemarkOptions<'input> {
    SeaLevelPressure(Pressure),
    UnknownRemark(&'input str)
}

peg::parser! {
    pub grammar metar_remarks() for str {

        rule digit() -> &'input str = quiet!{$(['0'..='9'])} / expected!("digit");
        rule letter() -> &'input str = quiet!{$(['A'..='Z'])} / expected!("letter");
        rule remark_char() -> &'input str = letter() / digit() / $("/");
        rule remark_prefix() -> &'input str = quiet!{$(":RMK" / "R MK"/ "RMK" / "REMARK")} / expected!("remark_prefix");

        rule required_whitespace() =
            quiet!{
                (
                    (whitespace_char()+ ("/"+ whitespace_char())+)
                    / (whitespace_char()+ ("M" whitespace_char())+)
                    / whitespace_char()+
                )+
            }
            / expected!("whitespace");
        
        rule whitespace_char() -> &'input str = $(
            " "
            / "\r\n"
            / "\n"
            / "\t"
            / ">"
        );

        pub rule remarks() -> Remarks = remark_prefix() required_whitespace()+ remarks:remark() ** required_whitespace() "="? {
            let mut sea_level_pressure: Option<Pressure> = None;
            let mut unknown_remarks: Vec<String> = Vec::new();
            for remark in remarks {
                match remark {
                    RemarkOptions::SeaLevelPressure(p) => sea_level_pressure = Some(p),
                    RemarkOptions::UnknownRemark(uk) => unknown_remarks.push(uk.to_string())
                }
            }
            Remarks {
                sea_level_pressure,
                unknown_remarks
            }
        }

        rule remark() -> RemarkOptions<'input> = sea_level_pressure() / unknown_remark();

        rule sea_level_pressure() -> RemarkOptions<'input> = "SLP" slp:$(digit()+) {
            let mut slpval = slp.parse::<f64>().unwrap();
            if slpval > 500.0 {
                slpval = ((slpval / 10.0) + 900.0) / 10.0;
            } else {
                slpval = ((slpval / 10.0) + 1000.0) / 10.0;
            }
            RemarkOptions::SeaLevelPressure(
                Pressure::new::<kilopascal>(slpval)
            )
        }

        rule unknown_remark() -> RemarkOptions<'input> = remark: $(remark_char()+) {
            RemarkOptions::UnknownRemark(remark)
        }
    }
}