use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref MAP: Mutex<HashMap<Value,String>> = Mutex::new(HashMap::new());
    static ref COUNT: Mutex<u32> = Mutex::new(0);
}

// trait maptrait {
//     fn get(&self, k: &Value) -> Option<&String>;
//     fn insert(&self, k: Value, v:String);

// }

// impl maptrait for Mutex<HashMap<Value,String>> {
//     fn get(&self,k: &Value) -> Option<&String> {
//         self.lock().unwrap().get(k)
//     }
//     fn insert(&self, k: Value, v:String) {

//     }
// }


pub trait GenerateAsm {
    fn generate(&self,dfg:Option<&DataFlowGraph>) -> String;
}

impl GenerateAsm for koopa::ir::Program {
    fn generate(&self,_dfg:Option<&DataFlowGraph>) -> String {
        let mut ans=String::from("  .text\n");
        for &func in self.func_layout() {
            ans.push_str(self.func(func).generate(None).as_str());
        }
        ans
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self,_dfg:Option<&DataFlowGraph>) -> String {
        let mut ans=format!("  .globl {}\n{}:\n",&self.name()[1..],&self.name()[1..]);
        
        for (&_bb, node) in self.layout().bbs() {
            
            for &inst in node.insts().keys() {
                ans.push_str(inst.generate(Some(self.dfg())).as_str());
            }
        }
        ans
    }
}


use koopa::ir::{ValueKind, dfg::DataFlowGraph, BinaryOp,Value};

impl GenerateAsm for koopa::ir::entities::Value {
    fn generate(&self,dfg:Option<&DataFlowGraph>) -> String {
        let value_data=dfg.unwrap().value(*self);
        if let Some(_varname)=MAP.lock().unwrap().get(self) {
            return String::from("");
        }
        match value_data.kind() {
            ValueKind::Integer(int) => {
                match int.value() {
                    0 => {
                        MAP.lock().unwrap().insert(*self, String::from("x0"));
                        String::from("")
                    },
                    _ => {
                        let count = *COUNT.lock().unwrap();
                        *COUNT.lock().unwrap() += 1;
                        MAP.lock().unwrap().insert(*self, format!("t{}",count));
                        format!("  li    {}, {}\n",MAP.lock().unwrap().get(self).unwrap(),int.value())
                    }
                }
            }
            ValueKind::Return(ret) => {
                if let Some(s)=MAP.lock().unwrap().get(&ret.value().unwrap()) {
                    format!("  mv    a0, {}\n  ret",s)
                }
                else {
                    if let ValueKind::Integer(v)=dfg.unwrap().value(ret.value().unwrap()).kind() {
                        format!("  li    a0, {}\n  ret",v.value())
                    }
                    else {
                        format!("")
                    }
                }
                // println!("{:#?}",MAP.lock().unwrap().get(&ret.value().unwrap()));
                // format!("  mv    a0, {}\n  ret",MAP.lock().unwrap().get(&ret.value().unwrap()).unwrap())
                
            }
            ValueKind::Binary(op) => {
                let count = *COUNT.lock().unwrap();
                *COUNT.lock().unwrap() += 1;
                MAP.lock().unwrap().insert(*self, format!("t{}",count));
                let lval=op.lhs();
                let rval=op.rhs();
                let lexpr=lval.generate(dfg);
                let rexpr=rval.generate(dfg);
                let tmp=MAP.lock().unwrap();
                let selfvar=tmp.get(self).unwrap();
                let lvarstr=tmp.get(&lval).unwrap();
                let rvarstr=tmp.get(&rval).unwrap();
                match op.op() {
                    BinaryOp::Sub => format!("{}{}  sub   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Xor => format!("{}{}  xor   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Eq => format!("{}{}  seqz  {}, {}\n",lexpr,rexpr,selfvar,rvarstr),
                    BinaryOp::NotEq => format!("{}{}  snez  {}, {}\n",lexpr,rexpr,selfvar,rvarstr),
                    BinaryOp::Add => format!("{}{}  add   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Mul => format!("{}{}  mul   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Div => format!("{}{}  mul   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Mod => format!("{}{}  mul   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Lt => format!("{}{}  slt   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Gt => format!("{}{}  sgt   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Le => format!("{}{}  sgt   {}, {}, {}\n  seqz  {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr,selfvar,selfvar),
                    BinaryOp::Ge => format!("{}{}  slt   {}, {}, {}\n  seqz  {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr,selfvar,selfvar),
                    BinaryOp::And => format!("{}{}  and   {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    BinaryOp::Or => format!("{}{}  or    {}, {}, {}\n",lexpr,rexpr,selfvar,lvarstr,rvarstr),
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}
