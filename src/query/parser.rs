use crate::error::{StratumError, Result};
use crate::record::RecordId;

/// A parsed AQSL (Akasha Query Specification Language) query.
#[derive(Debug, Clone)]
pub struct AqslQuery {
    pub clauses: Vec<Clause>,
    pub order: OrderDir,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub enum Clause {
    /// Filter by time range
    TimeRange { from: TimeRef, to: TimeRef },
    /// Filter by minimum time (AFTER keyword)
    TimeAfter(TimeRef),
    /// Filter by maximum time (BEFORE keyword)
    TimeBefore(TimeRef),
    /// Semantic similarity search
    SimilarTo { embedding: Vec<f32>, threshold: f32 },
    /// Causal descendants (CAUSED BY)
    CausedBy { id: RecordId, depth: usize },
    /// Causal ancestors (CAUSES)
    Causes { id: RecordId, depth: usize },
    /// Schema filter
    Schema(String),
    /// Metadata tag filter
    Tag { key: String, value: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderDir {
    TimeAsc,
    TimeDesc,
    RelevanceDesc,
}

#[derive(Debug, Clone)]
pub enum TimeRef {
    /// Nanoseconds since Unix epoch
    Nanos(i64),
    /// ISO 8601 string
    Iso(String),
    /// NOW() function
    Now,
    /// NOW() - interval_ns
    NowMinus(i64),
}

impl TimeRef {
    pub fn to_nanos(&self) -> i64 {
        match self {
            TimeRef::Nanos(n) => *n,
            TimeRef::Now => chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
            TimeRef::NowMinus(delta) => {
                chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) - delta
            }
            TimeRef::Iso(s) => {
                s.parse::<chrono::DateTime<chrono::Utc>>()
                    .map(|dt| dt.timestamp_nanos_opt().unwrap_or(0))
                    .unwrap_or(0)
            }
        }
    }
}

/// Parse an AQSL query string into a structured query plan.
///
/// # Grammar (simplified)
/// ```text
/// FIND records
/// [WHERE clause [AND clause ...]]
/// [ORDER BY (time ASC | time DESC | relevance)]
/// [LIMIT n]
/// [OFFSET n]
///
/// clause:
///   time BETWEEN <time_ref> AND <time_ref>
///   time AFTER <time_ref>
///   time BEFORE <time_ref>
///   similar_to embedding([f32, ...]) [WITH threshold f32]
///   caused_by <hex_id> [WITH depth n]
///   causes <hex_id> [WITH depth n]
///   schema = "<name>"
///   tag <key> = "<value>"
/// ```
pub fn parse(input: &str) -> Result<AqslQuery> {
    let tokens = tokenize(input.trim());
    let mut p = Parser::new(tokens);
    p.parse_query()
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Keyword(String),
    Ident(String),
    StringLit(String),
    Number(f64),
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comma,
    Equals,
    EOF,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }
            '[' => { tokens.push(Token::LBracket); chars.next(); }
            ']' => { tokens.push(Token::RBracket); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '=' => { tokens.push(Token::Equals); chars.next(); }
            '"' | '\'' => {
                let quote = c;
                chars.next();
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    chars.next();
                    if c2 == quote { break; }
                    s.push(c2);
                }
                tokens.push(Token::StringLit(s));
            }
            '-' | '0'..='9' => {
                let mut s = String::new();
                if c == '-' { s.push(c); chars.next(); }
                while let Some(&c2) = chars.peek() {
                    if c2.is_ascii_digit() || c2 == '.' {
                        s.push(c2); chars.next();
                    } else { break; }
                }
                if let Ok(n) = s.parse::<f64>() {
                    tokens.push(Token::Number(n));
                }
            }
            _ if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() || c2 == '_' || c2 == '.' || c2 == ':' {
                        s.push(c2); chars.next();
                    } else { break; }
                }
                let upper = s.to_uppercase();
                let kw = ["FIND", "RECORDS", "WHERE", "AND", "OR", "ORDER", "BY",
                    "LIMIT", "OFFSET", "TIME", "BETWEEN", "AFTER", "BEFORE",
                    "SIMILAR_TO", "EMBEDDING", "WITH", "THRESHOLD", "CAUSED_BY",
                    "CAUSES", "SCHEMA", "TAG", "ASC", "DESC", "RELEVANCE",
                    "NOW", "DEPTH"];
                if kw.contains(&upper.as_str()) {
                    tokens.push(Token::Keyword(upper));
                } else {
                    tokens.push(Token::Ident(s));
                }
            }
            _ => { chars.next(); }
        }
    }
    tokens.push(Token::EOF);
    tokens
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens.get(self.pos).cloned().unwrap_or(Token::EOF);
        if self.pos < self.tokens.len() { self.pos += 1; }
        t
    }

    fn expect_keyword(&mut self, kw: &str) -> Result<()> {
        match self.advance() {
            Token::Keyword(k) if k == kw => Ok(()),
            t => Err(StratumError::QueryParse(format!("expected keyword {kw}, got {t:?}"))),
        }
    }

    fn parse_query(&mut self) -> Result<AqslQuery> {
        self.expect_keyword("FIND")?;
        self.expect_keyword("RECORDS")?;

        let mut clauses = Vec::new();
        let mut order = OrderDir::TimeDesc;
        let mut limit = 100usize;
        let mut offset = 0usize;

        while self.peek() != &Token::EOF {
            match self.peek() {
                Token::Keyword(k) if k == "WHERE" => {
                    self.advance();
                    clauses.push(self.parse_clause()?);
                    while matches!(self.peek(), Token::Keyword(k) if k == "AND") {
                        self.advance();
                        clauses.push(self.parse_clause()?);
                    }
                }
                Token::Keyword(k) if k == "ORDER" => {
                    self.advance();
                    self.expect_keyword("BY")?;
                    order = self.parse_order()?;
                }
                Token::Keyword(k) if k == "LIMIT" => {
                    self.advance();
                    if let Token::Number(n) = self.advance() {
                        limit = n as usize;
                    }
                }
                Token::Keyword(k) if k == "OFFSET" => {
                    self.advance();
                    if let Token::Number(n) = self.advance() {
                        offset = n as usize;
                    }
                }
                _ => { self.advance(); }
            }
        }

        Ok(AqslQuery { clauses, order, limit, offset })
    }

    fn parse_clause(&mut self) -> Result<Clause> {
        match self.peek().clone() {
            Token::Keyword(k) if k == "TIME" => {
                self.advance();
                match self.peek().clone() {
                    Token::Keyword(k) if k == "BETWEEN" => {
                        self.advance();
                        let from = self.parse_time_ref()?;
                        self.expect_keyword("AND")?;
                        let to = self.parse_time_ref()?;
                        Ok(Clause::TimeRange { from, to })
                    }
                    Token::Keyword(k) if k == "AFTER" => {
                        self.advance();
                        Ok(Clause::TimeAfter(self.parse_time_ref()?))
                    }
                    Token::Keyword(k) if k == "BEFORE" => {
                        self.advance();
                        Ok(Clause::TimeBefore(self.parse_time_ref()?))
                    }
                    t => Err(StratumError::QueryParse(format!("unexpected token after TIME: {t:?}")))
                }
            }
            Token::Keyword(k) if k == "SIMILAR_TO" => {
                self.advance();
                self.expect_keyword("EMBEDDING")?;
                self.advance(); // LParen
                self.advance(); // LBracket
                let embedding = self.parse_float_list()?;
                self.advance(); // RBracket
                self.advance(); // RParen

                let threshold = if matches!(self.peek(), Token::Keyword(k) if k == "WITH") {
                    self.advance();
                    self.expect_keyword("THRESHOLD")?;
                    if let Token::Number(n) = self.advance() { n as f32 } else { 0.7 }
                } else {
                    0.7
                };
                Ok(Clause::SimilarTo { embedding, threshold })
            }
            Token::Keyword(k) if k == "CAUSED_BY" => {
                self.advance();
                let hex_id = match self.advance() {
                    Token::StringLit(s) | Token::Ident(s) => s,
                    t => return Err(StratumError::QueryParse(format!("expected hex ID, got {t:?}"))),
                };
                let id = parse_hex_id(&hex_id)?;
                let depth = if matches!(self.peek(), Token::Keyword(k) if k == "WITH") {
                    self.advance();
                    self.expect_keyword("DEPTH")?;
                    if let Token::Number(n) = self.advance() { n as usize } else { 5 }
                } else { 5 };
                Ok(Clause::CausedBy { id, depth })
            }
            Token::Keyword(k) if k == "CAUSES" => {
                self.advance();
                let hex_id = match self.advance() {
                    Token::StringLit(s) | Token::Ident(s) => s,
                    t => return Err(StratumError::QueryParse(format!("expected hex ID, got {t:?}"))),
                };
                let id = parse_hex_id(&hex_id)?;
                let depth = if matches!(self.peek(), Token::Keyword(k) if k == "WITH") {
                    self.advance();
                    self.expect_keyword("DEPTH")?;
                    if let Token::Number(n) = self.advance() { n as usize } else { 5 }
                } else { 5 };
                Ok(Clause::Causes { id, depth })
            }
            Token::Keyword(k) if k == "SCHEMA" => {
                self.advance();
                self.advance(); // =
                let schema = match self.advance() {
                    Token::StringLit(s) | Token::Ident(s) => s,
                    t => return Err(StratumError::QueryParse(format!("expected schema name, got {t:?}"))),
                };
                Ok(Clause::Schema(schema))
            }
            Token::Keyword(k) if k == "TAG" => {
                self.advance();
                let key = match self.advance() {
                    Token::Ident(s) | Token::StringLit(s) => s,
                    t => return Err(StratumError::QueryParse(format!("expected tag key, got {t:?}"))),
                };
                self.advance(); // =
                let value = match self.advance() {
                    Token::StringLit(s) | Token::Ident(s) => s,
                    t => return Err(StratumError::QueryParse(format!("expected tag value, got {t:?}"))),
                };
                Ok(Clause::Tag { key, value })
            }
            t => Err(StratumError::QueryParse(format!("unexpected clause start: {t:?}")))
        }
    }

    fn parse_time_ref(&mut self) -> Result<TimeRef> {
        match self.peek().clone() {
            Token::StringLit(s) => {
                self.advance();
                Ok(TimeRef::Iso(s))
            }
            Token::Number(n) => {
                self.advance();
                Ok(TimeRef::Nanos(n as i64))
            }
            Token::Keyword(k) if k == "NOW" => {
                self.advance();
                if matches!(self.peek(), Token::Number(_)) {
                    if let Token::Number(n) = self.advance() {
                        return Ok(TimeRef::NowMinus(n as i64));
                    }
                }
                Ok(TimeRef::Now)
            }
            t => Err(StratumError::QueryParse(format!("expected time reference, got {t:?}")))
        }
    }

    fn parse_float_list(&mut self) -> Result<Vec<f32>> {
        let mut vals = Vec::new();
        while let Token::Number(n) = self.peek().clone() {
            self.advance();
            vals.push(n as f32);
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(vals)
    }

    fn parse_order(&mut self) -> Result<OrderDir> {
        match self.peek().clone() {
            Token::Keyword(k) if k == "TIME" => {
                self.advance();
                match self.peek().clone() {
                    Token::Keyword(k) if k == "ASC" => { self.advance(); Ok(OrderDir::TimeAsc) }
                    _ => Ok(OrderDir::TimeDesc),
                }
            }
            Token::Keyword(k) if k == "RELEVANCE" => {
                self.advance();
                Ok(OrderDir::RelevanceDesc)
            }
            _ => Ok(OrderDir::TimeDesc),
        }
    }
}

fn parse_hex_id(s: &str) -> Result<RecordId> {
    let bytes = hex::decode(s.trim())
        .map_err(|_| StratumError::QueryParse(format!("invalid hex ID: {s}")))?;
    if bytes.len() != 32 {
        return Err(StratumError::QueryParse(format!(
            "hex ID must be 64 hex chars (32 bytes), got {} bytes", bytes.len()
        )));
    }
    let mut id = [0u8; 32];
    id.copy_from_slice(&bytes);
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_time_between() {
        let q = parse("FIND records WHERE time BETWEEN \"2024-01-01T00:00:00Z\" AND \"2024-12-31T23:59:59Z\" LIMIT 50").unwrap();
        assert_eq!(q.limit, 50);
        assert!(matches!(&q.clauses[0], Clause::TimeRange { .. }));
    }

    #[test]
    fn parse_similar_to() {
        let q = parse("FIND records WHERE similar_to embedding([0.1, 0.2, 0.3]) WITH threshold 0.8").unwrap();
        assert!(matches!(&q.clauses[0], Clause::SimilarTo { threshold, .. } if (*threshold - 0.8).abs() < 0.01));
    }

    #[test]
    fn parse_schema_filter() {
        let q = parse("FIND records WHERE schema = \"payment.v1\"").unwrap();
        assert!(matches!(&q.clauses[0], Clause::Schema(s) if s == "payment.v1"));
    }

    #[test]
    fn parse_order_by_time_asc() {
        let q = parse("FIND records ORDER BY time ASC LIMIT 10").unwrap();
        assert_eq!(q.order, OrderDir::TimeAsc);
        assert_eq!(q.limit, 10);
    }

    #[test]
    fn parse_multiple_clauses() {
        let q = parse("FIND records WHERE time AFTER \"2024-01-01T00:00:00Z\" AND schema = \"sensor.v1\" LIMIT 20").unwrap();
        assert_eq!(q.clauses.len(), 2);
    }

    #[test]
    fn parse_invalid_query_returns_error() {
        let result = parse("SELECT * FROM records");
        assert!(result.is_err());
    }
}
