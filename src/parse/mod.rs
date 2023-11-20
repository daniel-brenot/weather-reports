mod parser;

pub use parser::weather_reports::metar;


/// Convenience function for converting a parsing error
/// into a [codespan_reporting::diagnostic::Diagnostic] for displaying to a user.
#[cfg(feature = "codespan_helpers")]
pub fn into_diagnostic(
    err: &peg::error::ParseError<peg::str::LineCol>,
) -> codespan_reporting::diagnostic::Diagnostic<()> {
    let expected_count = err.expected.tokens().count();
    let label_msg = if expected_count == 0 {
        "unclear cause".to_string()
    } else if expected_count == 1 {
        format!("expected {}", err.expected.tokens().next().unwrap())
    } else {
        let tokens = {
            let mut tokens = err.expected.tokens().collect::<Vec<_>>();
            tokens.sort_unstable();
            tokens
        };
        let mut acc = "expected one of ".to_string();
        for token in tokens.iter().take(expected_count - 1) {
            acc += token;
            acc += ", ";
        }
        acc += "or ";
        acc += tokens.last().unwrap();
        acc
    };
    codespan_reporting::diagnostic::Diagnostic::error()
        .with_message("could not parse report")
        .with_labels(vec![codespan_reporting::diagnostic::Label::primary(
            (),
            err.location.offset..err.location.offset,
        )
        .with_message(label_msg)])
}
