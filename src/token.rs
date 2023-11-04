/// `Token` 構造体は、字句解析器が生成するトークンを表します。
///
/// トークンは、ソースコードを構成する個々の要素（キーワード、識別子、リテラルなど）に相当します。
/// この構造体は、トークンの型、字句、リテラル値、およびトークンが現れるソースコード上の行番号を保持します。
///
/// # フィールド
///
/// * `ty` - `TokenType` 列挙型のインスタンスであり、トークンの型を表します。
/// * `lexeme` - `Vec<char>` 型で、トークンの字句を文字のベクターとして保持します。
/// ＊ `literal` - `Option<Literal>` 型で、トークンに関連つけられたリテラル値を表すオプション値です。
///   これは、トークンがリテラル値を有さない型の場合には None になります。
/// * `line` - `usize` 型で、トークンが見つかったソースコードの行番号を保持します。
///
/// # 例
///
/// ```
/// let token = Token {
///     ty: TokenType::Identifier,
///     lexeme: vec!['f', 'i', 'v', 'e'],
///     literal: Some(Literal::Identifier("five".to_string())),
///     line: 1,
/// };
/// ```
#[derive(PartialEq, Debug)]
pub struct Token {
    /// トークンの型を表します
    pub ty: TokenType,
    /// トークンの字句を保持する文字のベクター
    pub lexeme: Vec<char>,
    /// トークンに関連するリテラル値、リテラルではない場合は `None`
    pub literal: Option<Literal>,
    /// トークンが見つかったソースコードの行番号
    pub line: usize,
}

/// `Literal` 列挙型 Lox 言語で使用する識別子の種類と実際のリテラル値を表します。
///
/// この列挙型は、識別子、文字列リテラル、または数値リテラルを保持することができます。
/// 各列挙子は、それぞれの値を `String` または `f64` として保持します。
///
/// # 例
///
/// ```
/// let identifier = Literal::Identifier("myVar".to_string());
/// let string = Literal::Str("Hello, world!".to_string());
/// let number = Literal::Number(3.14);
/// ```
///
/// Lox 言語においては以下のように識別されます
/// ```
/// var name              = "keisuke";
///     ↓                   ↓
///     Identifier("name")  Str("keisuke")
/// ```
#[derive(PartialEq, Debug)]
pub enum Literal {
    /// 識別子を表す列挙子で、`String`型の値を保持します。
    Identifier(String),
    /// 文字列リテラルを表す列挙子で、`String`型の値を保持します。
    Str(String),
    /// 数値リテラルを表す列挙子で、`f64`型の値を保持します。
    Number(f64),
}

/// `TokenType` 列挙型は、異なる種類のトークンを識別します。
///
/// この列挙型は、単一記号、一つまたは二つの記号によって構成されるトークン、リテラル、キーワード、
/// そしてファイルの終端を表す特別なトークンを区別するために使用されます。
///
/// # 例
///
/// ```
/// let single_char_token = TokenType::Plus;     // +
/// let multi_char_token = TokenType::BangEqual; // !=
/// let literal_token = TokenType::Number;       // 1.23
/// let keyword_token = TokenType::If;           // if
/// let eof_token = TokenType::Eof;              //
/// ```
#[derive(PartialEq, Debug)]
pub enum TokenType {
    // 記号1個のトークン
    LParan,
    RParan,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // 記号1個、または2個によるトークン
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // リテラル
    Identifier,
    String,
    Number,

    // キーワード
    And,
    Or,
    If,
    Else,
    True,
    False,
    For,
    While,
    Nil,
    Fun,
    Return,
    Class,
    Super,
    This,
    Var,
    Print,

    // End of file
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;

        let matching_literal = match self {
            LParan => "(",
            RParan => ")",
            LBrace => "{{",
            RBrace => "}}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            SemiColon => ";",
            Slash => "/",
            Star => "*",
            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Greater => ">",
            GreaterEqual => ">=",
            Less => "<",
            LessEqual => "<=",
            Identifier => "Identifier",
            String => "String",
            Number => "Number",
            And => "and",
            Or => "or",
            If => "if",
            Else => "else",
            True => "true",
            False => "false",
            For => "for",
            While => "while",
            Nil => "nil",
            Fun => "fun",
            Return => "return",
            Class => "class",
            Super => "super",
            This => "this",
            Var => "var",
            Eof => "eof",
            Print => "print",
        };

        write!(f, "{matching_literal}")
    }
}

/// 特定の文字列リテラルに対応する `TokenType` を返します。
///
/// この関数は、与えられた文字列リテラルが言語のキーワードの一つであるかを判断し、
/// それに対応する`TokenType`を返します。もしキーワードに該当しない場合、
/// 一般的な識別子として`TokenType::Identifier`を返します。
///
/// # 引数
///
/// * `literal` - 識別するキーワードの文字列スライス。
///
/// # 例
///
/// ```
/// assert_eq!(match_keywords("if"), Some(TokenType::If));
/// assert_eq!(match_keywords("while"), Some(TokenType::While));
/// assert_eq!(match_keywords("unknown"), None);
/// ```
///
/// # 戻り値
///
/// 対応する`TokenType`列挙子を返します。キーワードでない場合は`TokenType::Identifier`。
pub fn match_keywords(literal: &str) -> Option<TokenType> {
    // TODO: 安定化した後は std::cell::LazyCell と HashMap の組み合わせを使いたい
    let ty = match literal {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    };

    match ty {
        TokenType::Identifier => None,
        _ => Some(ty),
    }
}
