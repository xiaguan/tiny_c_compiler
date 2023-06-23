use crate::scanner::KeywordType;

#[derive(Debug)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOpType {
    pub fn from_keyword(keyword: &KeywordType) -> Option<BinaryOpType> {
        match keyword {
            KeywordType::Add => Some(BinaryOpType::Add),
            KeywordType::Sub => Some(BinaryOpType::Sub),
            KeywordType::Mul => Some(BinaryOpType::Mul),
            KeywordType::Div => Some(BinaryOpType::Div),
            _ => None,
        }
    }
}

/// The abstraction node of the AST.
/// It could be a expression
#[derive(Debug)]
pub enum Node {
    NUM(i64),
    BINOP {
        op: BinaryOpType,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Node {
    pub fn from_num(num: i64) -> Node {
        Self::NUM(num)
    }

    pub fn from_binop(op: BinaryOpType, lhs: Node, rhs: Node) -> Node {
        Self::BINOP {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
