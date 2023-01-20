#[derive(Debug)]
pub struct CompUnit {
  pub func_def: FuncDef,
}

#[derive(Debug)]
pub enum Decl {
    CDecl(ConstDecl),
    VDecl(VarDecl),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub constdefs: Vec<ConstDef>,
}

#[derive(Debug)]
pub enum BType {
    Int,
}

#[derive(Debug)]
pub struct ConstDef {
    pub ident: String,
    pub constinitval: ConstInitVal,
}

#[derive(Debug)]
pub struct ConstInitVal {
    pub constexp: ConstExp,
}

#[derive(Debug)]
pub struct VarDecl {
    pub vardefs: Vec<VarDef>,
}

#[derive(Debug)]
pub struct VarDef {
    pub ident: String,
    pub initval: Option<InitVal>,
}

#[derive(Debug)]
pub struct InitVal {
    pub exp: Exp,
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub block: Block,
}

#[derive(Debug)]
pub enum FuncType{
    Int,
}

#[derive(Debug)]
pub struct Block{
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(LVal, Exp),
    Return(Exp),
}

#[derive(Debug)]
pub struct Exp{
    pub lorexp:Box<LOrExp>,
}

#[derive(Debug)]
pub struct LVal {
    pub ident: String,
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Exp),
    Number(i32),
    LVal(LVal),
}

#[derive(Debug)]
pub enum UnaryExp {
    PExp(PrimaryExp),
    UExp(UnaryOp,Box<UnaryExp>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
    Inv,
}

#[derive(Debug)]
pub enum MulExp {
    UExp(UnaryExp),
    MExp(Box<MulExp>,MulOp,UnaryExp),
}

#[derive(Debug)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum AddExp {
    MExp(MulExp),
    AExp(Box<AddExp>,AddOp,MulExp),
}

#[derive(Debug)]
pub enum AddOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum RelExp {
    AExp(AddExp),
    RExp(Box<RelExp>,RelOp,AddExp),
}

#[derive(Debug)]
pub enum RelOp {
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug)]
pub enum EqExp {
    RExp(RelExp),
    EExp(Box<EqExp>,EqOp,RelExp),
}

#[derive(Debug)]
pub enum EqOp {
    Equ,
    Ne,
}

#[derive(Debug)]
pub enum LAndExp {
    EExp(EqExp),
    LAExp(Box<LAndExp>,EqExp),
}

#[derive(Debug)]
pub enum LOrExp {
    LAExp(LAndExp),
    LOExp(Box<LOrExp>,LAndExp),
}

#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
}