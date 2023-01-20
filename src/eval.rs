use crate::ast::*;

pub trait Evaluate {
    fn eval(&self) -> Option<i32>;
}

impl Evaluate for Exp {
    fn eval(&self) -> Option<i32> {
        self.lorexp.eval()
    }
}

impl Evaluate for PrimaryExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::Exp(exp) => exp.eval(),
            Self::Number(number) => Some(*number),
            _ => None, // 常量声明中，右边不允许出现左值
        }
    }
}

impl Evaluate for UnaryExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::PExp(primaryexp) => primaryexp.eval(),
            Self::UExp(unaryop, unaryexp) => unaryexp.eval().map(|exp| match unaryop {
                UnaryOp::Inv => !exp,
                UnaryOp::Neg => -exp,
                UnaryOp::Not => (exp == 0) as i32,
                UnaryOp::Pos => exp,
            })
        }
    }
}

impl Evaluate for MulExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::UExp(unaryexp) => unaryexp.eval(),
            Self::MExp(mulexp, mulop, unaryexp) => match (mulexp.eval(), unaryexp.eval()) {
                (Some(lhs), Some(rhs)) => match mulop {
                    MulOp::Div => Some(lhs/rhs),
                    MulOp::Mod => Some(lhs%rhs),
                    MulOp::Mul => Some(lhs*rhs),
                },
                _ => None,
            }
        }
    }
}

impl Evaluate for AddExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::MExp(mulexp) => mulexp.eval(),
            Self::AExp(addexp, addop, mulexp) => match (addexp.eval(), mulexp.eval()) {
                (Some(lhs), Some(rhs)) => match addop {
                    AddOp::Add => Some(lhs+rhs),
                    AddOp::Sub => Some(lhs-rhs),
                },
                _ => None,
            }
        }
    }
}

impl Evaluate for RelExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::AExp(addexp) => addexp.eval(),
            Self::RExp(relexp, relop, addexp) => match (relexp.eval(), addexp.eval()) {
                (Some(lhs), Some(rhs)) => match relop {
                    RelOp::Ge => Some((lhs>=rhs) as i32),
                    RelOp::Gt => Some((lhs>rhs) as i32),
                    RelOp::Le => Some((lhs<=rhs) as i32),
                    RelOp::Lt => Some((lhs<rhs) as i32),
                },
                _ => None,
            }
        }
    }
}

impl Evaluate for EqExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::RExp(relexp) => relexp.eval(),
            Self::EExp(eqexp, eqop, relexp) => match (eqexp.eval(), relexp.eval()) {
                (Some(lhs), Some(rhs)) => match eqop {
                    EqOp::Equ => Some((lhs==rhs) as i32),
                    EqOp::Ne => Some((lhs!=rhs) as i32),
                },
                _ => None,
            }
        }
    }
}

impl Evaluate for LAndExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::EExp(eqexp) => eqexp.eval(),
            Self::LAExp(landexp, eqexp) => match (landexp.eval(), eqexp.eval()) {
                (Some(lhs), Some(rhs)) => Some((lhs != 0 && rhs != 0) as i32),
                _ => None,
            }
        }
    }
}

impl Evaluate for LOrExp {
    fn eval(&self) -> Option<i32> {
        match self {
            Self::LAExp(landexp) => landexp.eval(),
            Self::LOExp(lorexp, landexp) => match (lorexp.eval(), landexp.eval()) {
                (Some(lhs), Some(rhs)) => Some((lhs != 0 || rhs != 0) as i32),
                _ => None,
            }
        }
    }
}

impl Evaluate for ConstExp {
    fn eval(&self) -> Option<i32> {
        self.exp.eval()
    }
}