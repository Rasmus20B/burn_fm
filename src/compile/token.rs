
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Table,
    Relationship,
    ValueList,
    Identifier,
    DataType,
    SemiColon,
    Colon,
    Comma,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    OpenQuote,
    CloseQuote,
    Exclamation,
    NewLine,
    Calculation, /* Anything found within a calculation specifier will be compiled in a different layer */
    End,
    EComparison,
    NEComparison,
    LComparison,
    LEComparison,
    GComparison,
    GEComparison,
    CComparison,
    Unique,
    Required,
    Existing,
    FoundIn,
    Context,
    String,
    EOF,
}

pub struct Token {
    pub ttype: TokenType,
    pub text: String,
}
