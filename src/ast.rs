use crate::scanner::KeywordType;
use log::debug;

/// Enum for the types of binary operations supported.
#[derive(Debug)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOpType {
    /// Function to create BinaryOpType from KeywordType.
    /// Return None if the keyword does not match any BinaryOpType.
    pub fn from_keyword(keyword: &KeywordType) -> Option<BinaryOpType> {
        match keyword {
            KeywordType::Add => {
                debug!("Creating BinaryOpType::Add from KeywordType::Add");
                Some(BinaryOpType::Add)
            }
            KeywordType::Sub => {
                debug!("Creating BinaryOpType::Sub from KeywordType::Sub");
                Some(BinaryOpType::Sub)
            }
            KeywordType::Mul => {
                debug!("Creating BinaryOpType::Mul from KeywordType::Mul");
                Some(BinaryOpType::Mul)
            }
            KeywordType::Div => {
                debug!("Creating BinaryOpType::Div from KeywordType::Div");
                Some(BinaryOpType::Div)
            }
            _ => None,
        }
    }
}

/// Enum to represent nodes in an AST.
/// Nodes can be numbers or binary operations.
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
    /// Function to create a Node from a number.
    pub fn from_num(num: i64) -> Node {
        debug!("Creating Node::NUM from i64: {}", num);
        Self::NUM(num)
    }

    /// Function to create a Node from a binary operation.
    pub fn from_binop(op: BinaryOpType, lhs: Node, rhs: Node) -> Node {
        debug!(
            "Creating Node::BINOP with op: {:?}, lhs: {:?}, rhs: {:?}",
            op, lhs, rhs
        );
        Self::BINOP {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
