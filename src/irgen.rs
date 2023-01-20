use crate::ast::*;
use crate::eval::Evaluate;
use std::sync::Mutex;
use std::fmt;
use std::collections::HashMap;

//static COUNT: AtomicUsize=AtomicUsize::new(0);

use lazy_static::lazy_static;

lazy_static! {
    static ref COUNT: Mutex<u32> = Mutex::new(0);
    static ref CONST_MAP: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

pub struct Retpair{
    pub irstr: String,
    pub varstr: String,
}

impl fmt::Display for Retpair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.irstr)
    }
}

pub trait IR {
    fn irdump(&self) -> Retpair;
} 

impl IR for CompUnit {
    fn irdump(&self) -> Retpair {
        self.func_def.irdump()
    }
}

impl IR for Decl {
    fn irdump(&self) -> Retpair {
        match self {
            Self::CDecl(constdecl) => constdecl.irdump(),
            _ => unimplemented!(),
        }
    }
}

impl IR for ConstDecl {
    fn irdump(&self) -> Retpair {
        for i in self.constdefs.iter() {
            i.irdump();
        }
        Retpair { irstr: format!(""), varstr: format!("") }
    }
}

// impl IR for BType {
//     fn irdump(&self) -> Retpair {
//         match self {
//             Self::Int => Retpair { irstr: format!(""), varstr: format!("int") }
//         }
//     }
// }

impl IR for ConstDef {
    fn irdump(&self) -> Retpair {
        CONST_MAP.lock().unwrap().insert(self.ident.clone(), self.constinitval.constexp.eval().unwrap());
        Retpair { irstr: format!(""), varstr: format!("") }
    }
}

impl IR for FuncDef {
    fn irdump(&self) -> Retpair {
        *COUNT.lock().unwrap()=0;
        Retpair{
            irstr: format!("fun @{}(): {} {{\n{}}}\n",self.ident,self.func_type.irdump(),self.block.irdump()),
            varstr: self.ident.clone(),
        } 
    }
}

impl IR for FuncType {
    fn irdump(&self) -> Retpair {
        match self {
            FuncType::Int => Retpair{
                irstr: String::from("i32"),
                varstr: String::from("")
            }
        }
    }
}

impl IR for Block {
    fn irdump(&self) -> Retpair {
        let its=self.items.iter().map(|x| x.irdump().irstr).collect::<Vec<_>>().join("");

        Retpair{
            irstr: format!("%entry: \n{}\n",its),
            varstr: format!("%entry")
        }
    }
}

impl IR for BlockItem {
    fn irdump(&self) -> Retpair {
        match self {
            Self::Decl(decl) => decl.irdump(),
            Self::Stmt(stmt) => stmt.irdump(),
        }
    }
}

impl IR for Stmt {
    fn irdump(&self) -> Retpair {
        match self {
            Self::Assign(lval, exp) => unimplemented!(),
            Self::Return(exp) => unimplemented!(),
        }
        // let ret=self.exp.irdump();
        // Retpair {
        //     irstr: format!("{}  ret {}",ret,ret.varstr),
        //     varstr: format!("")
        // }
    }
}

impl IR for Exp {
    fn irdump(&self) -> Retpair {
        self.lorexp.irdump()
    }
}

impl IR for PrimaryExp {
    fn irdump(&self) -> Retpair {
        match self {
            PrimaryExp::Exp(exp) => exp.irdump(),
            PrimaryExp::Number(num) => Retpair {
                irstr: format!(""),
                varstr: format!("{num}")
            },
            PrimaryExp::LVal(lval) => {
                if let Some(ident) = CONST_MAP.lock().unwrap().get(&lval.ident) {
                    Retpair {
                        irstr: format!(""),
                        varstr: format!("{ident}")
                    }
                }
                else {
                    Retpair {
                        irstr: format!(""),
                        varstr: format!("{}",lval.ident)
                    }
                }
            }
        }
    }
}

impl IR for UnaryExp {
    fn irdump(&self) -> Retpair {
        match self {
            UnaryExp::PExp(primaryexp) => primaryexp.irdump(),
            UnaryExp::UExp(op, unaryexp) => {
                let ret=unaryexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=1;
                Retpair {
                    irstr: format!("{}  %{} = {} {}\n",ret,count,op.irdump(),ret.varstr),
                    varstr: format!("%{}",count)
                }
            }
        }
    }
}

impl IR for UnaryOp {
    fn irdump(&self) -> Retpair {
        match self {
            UnaryOp::Neg => Retpair {
                irstr: String::from("sub 0,"),
                varstr: format!("")
            },
            UnaryOp::Not => Retpair {
                irstr: String::from("eq 0,"),
                varstr: format!("")
            },
            UnaryOp::Inv => Retpair {
                irstr: String::from("xor -1,"),
                varstr: format!("")
            },
            UnaryOp::Pos => Retpair {
                irstr: String::from("add 0,"),
                varstr: format!("")
            },
        }
    }
}

impl IR for MulExp {
    fn irdump(&self) -> Retpair {
        match self {
            MulExp::UExp(unaryexp) => unaryexp.irdump(),
            MulExp::MExp(mulexp, op, unaryexp) => {
                let retleft=mulexp.irdump();
                let retright=unaryexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=1;
                Retpair {
                    irstr: format!("{}{}  %{} = {} {}, {}\n",retleft,retright,count,op.irdump(),retleft.varstr,retright.varstr),
                    varstr: format!("%{}",count)
                }
            }
        }
    }
}

impl IR for MulOp {
    fn irdump(&self) -> Retpair {
        match self {
            MulOp::Div => Retpair { irstr: String::from("div"), varstr: String::from("") },
            MulOp::Mod => Retpair { irstr: String::from("mod"), varstr: String::from("") },
            MulOp::Mul => Retpair { irstr: String::from("mul"), varstr: String::from("") }
        }
    }
}

impl IR for AddExp {
    fn irdump(&self) -> Retpair {
        match self {
            AddExp::MExp(mulexp) => mulexp.irdump(),
            AddExp::AExp(addexp, op, mulexp) => {
                let retleft=addexp.irdump();
                let retright=mulexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=1;
                Retpair {
                    irstr: format!("{}{}  %{} = {} {}, {}\n",retleft,retright,count,op.irdump(),retleft.varstr,retright.varstr),
                    varstr: format!("%{}",count)
                }
            }
        }
    }
}

impl IR for AddOp {
    fn irdump(&self) -> Retpair {
        match self {
            AddOp::Add => Retpair { irstr: String::from("add"), varstr: String::from("") },
            AddOp::Sub => Retpair { irstr: String::from("sub"), varstr: String::from("") },
        }
    }
}

impl IR for RelExp {
    fn irdump(&self) -> Retpair {
        match self {
            RelExp::AExp(addexp) => addexp.irdump(),
            RelExp::RExp(relexp, op, addexp) => {
                let retleft=relexp.irdump();
                let retright=addexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=1;
                Retpair {
                    irstr: format!("{}{}  %{} = {} {}, {}\n",retleft,retright,count,op.irdump(),retleft.varstr,retright.varstr),
                    varstr: format!("%{}",count)
                }
            }
        }
    }
}

impl IR for RelOp {
    fn irdump(&self) -> Retpair {
        match self {
            RelOp::Lt => Retpair { irstr: String::from("lt"), varstr: String::from("") },
            RelOp::Gt => Retpair { irstr: String::from("gt"), varstr: String::from("") },
            RelOp::Le => Retpair { irstr: String::from("le"), varstr: String::from("") },
            RelOp::Ge => Retpair { irstr: String::from("ge"), varstr: String::from("") },
        }
    }
}

impl IR for EqExp {
    fn irdump(&self) -> Retpair {
        match self {
            EqExp::RExp(relexp) => relexp.irdump(),
            EqExp::EExp(eqexp, op, relexp) => {
                let retleft=eqexp.irdump();
                let retright=relexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=1;
                Retpair {
                    irstr: format!("{}{}  %{} = {} {}, {}\n",retleft,retright,count,op.irdump(),retleft.varstr,retright.varstr),
                    varstr: format!("%{}",count)
                }
            }
        }
    }
}

impl IR for EqOp {
    fn irdump(&self) -> Retpair {
        match self {
            EqOp::Equ => Retpair { irstr: String::from("eq"), varstr: String::from("") },
            EqOp::Ne => Retpair { irstr: String::from("ne"), varstr: String::from("") },
        }
    }
}

impl IR for LAndExp {
    fn irdump(&self) -> Retpair {
        match self {
            LAndExp::EExp(eqexp) => eqexp.irdump(),
            LAndExp::LAExp(landexp, eqexp) => {
                let retleft=landexp.irdump();
                let retright=eqexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=3;
                Retpair {
                    irstr: format!("{}{}  %{} = ne 0, {}\n  %{} = ne 0, {}\n  %{} = and %{}, %{}\n",retleft,retright,count,retleft.varstr,count+1,retright.varstr,count+2,count,count+1),
                    varstr: format!("%{}",count+2)
                }
            }
        }
    }
}

impl IR for LOrExp {
    fn irdump(&self) -> Retpair {
        match self {
            LOrExp::LAExp(landexp) => landexp.irdump(),
            LOrExp::LOExp(lorexp, landexp) => {
                let retleft=lorexp.irdump();
                let retright=landexp.irdump();
                let count=*COUNT.lock().unwrap();
                *COUNT.lock().unwrap()+=3;
                Retpair {
                    irstr: format!("{}{}  %{} = ne 0, {}\n  %{} = ne 0, {}\n  %{} = or %{}, %{}\n",retleft,retright,count,retleft.varstr,count+1,retright.varstr,count+2,count,count+1),
                    varstr: format!("%{}",count+2)
                }
            }
        }
    }
}