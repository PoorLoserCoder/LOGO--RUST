use std::fs::File;
use crate::lex::{Lex, Token};
use crate::bytecode::ByteCode;
use crate::value::{self, Value};

// ANCHOR: proto
#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec::<Value>,
    pub byte_codes: Vec::<ByteCode>,

    locals: Vec::<String>,
    lex: Lex,
}
// ANCHOR_END: proto

impl ParseProto {
    pub fn load(input: File) -> ParseProto {
        let mut proto = ParseProto {
            constants: Vec::new(),
            byte_codes: Vec::new(),
            locals: Vec::new(),
            lex: Lex::new(input),
        };

        proto.chunk();

        //println!("constants: {:?}", &proto.constants);
        //println!("byte_codes:");
        //for c in proto.byte_codes.iter() {
           // println!("  {:?}", c);
        //}

        proto
    }

    fn chunk(&mut self) {
        loop {
            match self.lex.next() {
                Token::Name(name) if name != "PENDOWN" && name != "MAKE"=> {
                    //println!("Function check this name: {:?}", name);
                    self.function_call(name);
                }
                Token::Name(name) if name == "PENDOWN" || name == "PENUP" || name == "XCOR" ||
                                          name == "YCOR" || name == "HEADING" || name == "COLOR" => {
                    //println!("Function check this name: {:?}", name);
                    self.single_function_call(name);
                }
                Token::Name(name) if name == "MAKE"=> {
                    //println!("Function check this name: {:?}", name);
                    self.single_function_call(name);
                    match self.lex.next() {
                        Token::String(var_name) => {
                            //println!("This should be a variable name: {:?}", var_name);
                            match self.lex.next() {
                                Token::Integer(value) => {
                                    //println!("This should be a variable value: {:?}", value);
                                    self.make_variable(var_name, Value::Integer(value));
                                }
                                _ => panic!("expected Integer as a value"),
                            }
                        }
                        _ => panic!("expected variable name, plz give me a string"),
                    }
                }
                Token::Eos => break,
                Token::Integer(s) => {
                    //println!("Function checks this Integer: {:?}", s);
                    self.function_call(s.to_string());
                }
                t => {
                    panic!("unexpected token {:?}", t)
                }

            }
        }
    }

    fn function_call(&mut self, name: String) {
        let ifunc = self.locals.len();
        let iarg = ifunc + 1;
    
        let code = self.load_var(ifunc, name);
        self.byte_codes.push(code);

        match self.lex.next() {

            Token::Integer(s) => {

                let code = self.load_const(iarg, Value::Integer(s));
                self.byte_codes.push(code);
                self.byte_codes.push(ByteCode::Call(ifunc as u8, 1));
                //println!("Followed by {:?}", s);
            }
            Token::Name(s) => {
                //println!("Followed by {:?}", s.clone());
                let code = self.load_const(iarg, Value::String(s));
                self.byte_codes.push(code);
                self.byte_codes.push(ByteCode::Operate(ifunc as u8));
            }
            Token::Eos =>{
                self.byte_codes.push(ByteCode::Call(ifunc as u8,1));
                //print!("Followed by EOS\n");
            }

            _ => panic!("expected string"),
        }
    

    }

fn single_function_call(&mut self, name: String) {
    let ifunc = self.locals.len();

    let code = self.load_var(ifunc, name);
    self.byte_codes.push(code);
    self.byte_codes.push(ByteCode::Operate(ifunc as u8))
    }



    fn load_const(&mut self, dst: usize, c: Value) -> ByteCode {
        ByteCode::LoadConst(dst as u8, self.add_const(c) as u16)
    }
      
    fn load_var(&mut self, dst: usize, name: String) -> ByteCode {
        if let Some(i) = self.get_local(&name) {
            // local variable
            ByteCode::Move(dst as u8, i as u8)
        } else {
            // global variable
            let ic = self.add_const(Value::String(name));
            ByteCode::GetGlobal(dst as u8, ic as u8)
        }
    }

    fn get_local(&self, name: &str) -> Option<usize> {
        self.locals.iter().rposition(|v| v == name)
    }

    fn add_const(&mut self, c: Value) -> usize {
        let constants = &mut self.constants;
        constants.iter().position(|v| v == &c)
            .unwrap_or_else(|| {
                constants.push(c);
                constants.len() - 1
            })
    }

    fn make_variable(&mut self, name: String, value: Value) {
        // 将变量名添加到常量表中
        let name_index = self.add_const(Value::String(name));
    
        // 将变量值添加到常量表中
        let value_index = self.add_const(value);
    
        // 生成 SetGlobal 字节码，将变量值的索引和变量名的索引存储在字节码中
        self.byte_codes.push(ByteCode::SetGlobal(value_index as u8, name_index as u16));
    }
}