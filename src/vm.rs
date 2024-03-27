use std::cmp::Ordering;
use std::collections::HashMap;
use crate::bytecode::ByteCode;
use crate::value::Value;
use crate::parse::ParseProto;
use crate::coordinate;
use plotters::prelude::*;
use plotters::style::full_palette::{BROWN, GREY, ORANGE, PURPLE};


// ANCHOR: print
// "print" function in Lua's std-lib.
// It supports only 1 argument and assumes the argument is at index:1 on stack.
fn start_draw(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    turtle.draw = true;
   // println!("The deatil of the turtle is ({},{}), the color is {}, the degree is {}, and turtle drawing is {}", 
                //turtle.x, turtle.y, turtle.color, turtle.head_degree,turtle.draw);
    0
}

fn stop_draw(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    turtle.draw = false;
    //println!("The deatil of the turtle is ({},{}), the color is {}, the degree is {}, and turtle drawing is {}", 
                //turtle.x, turtle.y, turtle.color, turtle.head_degree,turtle.draw);
    0
}

fn lib_forward(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    let get_number = if let Value::Integer(val) = state.stack[state.func_index + 1] {
        val as f32
    } else {
        panic!("Expected an Integer value in the stack");
    };
    let old_x = turtle.x;
    let old_y = turtle.y;

    turtle.x += (get_number * (turtle.head_degree as f32).to_radians().sin()) as i32;
    turtle.y -= (get_number * (turtle.head_degree as f32).to_radians().cos()) as i32;

    let color = match_color(turtle.color);
    if turtle.draw {
        root.draw(&PathElement::new(
            vec![(old_x, old_y), (turtle.x, turtle.y)],
            &color,
        )).unwrap();
    }

    0
}
fn lib_back(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    let get_number = if let Value::Integer(val) = state.stack[state.func_index + 1] {
        val as f32
    } else {
        panic!("Expected an Integer value in the stack");
    };

    let old_x = turtle.x;
    let old_y = turtle.y;

    turtle.x -= (get_number * (turtle.head_degree as f32).to_radians().sin()) as i32;
    turtle.y += (get_number * (turtle.head_degree as f32).to_radians().cos()) as i32;

    let color = match_color(turtle.color);
    if turtle.draw {
        root.draw(&PathElement::new(
            vec![(old_x, old_y), (turtle.x, turtle.y)],
            &color,
        )).unwrap();
    }

    0
}
fn lib_left(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    let get_number = if let Value::Integer(val) = state.stack[state.func_index + 1] {
        val
    } else {
        panic!("Expected an Integer value in the stack");
    };

    let old_x = turtle.x;
    let old_y = turtle.y;

    turtle.x -= get_number as i32;

    let color = match_color(turtle.color);
    if turtle.draw {
        root.draw(&PathElement::new(
        vec![(old_x, old_y), (turtle.x, turtle.y)],
                        &color,
        )).unwrap();
    }
    0
}
fn lib_right(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    let get_number = if let Value::Integer(val) = state.stack[state.func_index + 1] {
        val
    } else {
        panic!("Expected an Integer value in the stack");
    };

    let old_x = turtle.x;
    let old_y = turtle.y;

    turtle.x += get_number as i32;

    println!("The deatil of the turtle is ({},{}), the color is {}, the degree is {}, and turtle drawing is {}", 
                turtle.x, turtle.y, turtle.color, turtle.head_degree,turtle.draw);
    let color = match_color(turtle.color);
    if turtle.draw {
        root.draw(&PathElement::new(
            vec![(old_x, old_y), (turtle.x, turtle.y)],
            &color,
        )).unwrap();
    }
    0
}

fn lib_setx(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    let get_number = if let Value::Integer(val) = state.stack[state.func_index + 1] {
        println!("The value of the stack is {}", &val);
        val as f32
    } else {
        panic!("Expected an Integer value in the stack");
    };

    turtle.x = get_number as i32;
    println!("The deatil of the turtle is ({},{}), the color is {}, the degree is {}, and turtle drawing is {}", 
                turtle.x, turtle.y, turtle.color, turtle.head_degree,turtle.draw);
    0
}

fn lib_sety(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    let get_number = if let Value::Integer(val) = state.stack[state.func_index + 1] {
        println!("The value of the stack is {}", &val);
        val as f32
    } else {
        panic!("Expected an Integer value in the stack");
    };

    turtle.y = get_number as i32;
    0
}

fn lib_setpencolor(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    if let Value::Integer(val) = state.stack[state.func_index + 1] {
        turtle.color = val as i32;
        0
    } else {
        panic!("Expected an Integer value in the stack for the color");
    }
}

fn lib_setheading(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    if let Value::Integer(val) = state.stack[state.func_index + 1] {
        turtle.head_degree = val as i32;
        0
    } else {
        panic!("Expected an Integer value in the stack for the head direction");
    }
}
fn lib_make(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    todo!();
    0
}
fn lib_turn(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    if let Value::Integer(val) = state.stack[state.func_index + 1] {
        turtle.head_degree += val as i32;
        0
    } else {
        panic!("Expected an Integer value in the stack for the turn degree");
    }
}

fn lib_xor(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
        println!("X value of the turtle is {}",turtle.x);
        0
}

fn lib_yor(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    println!("Y value of the turtle is {}",turtle.y);
    0
}

fn lib_color(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    println!("color of the turtle is {}",turtle.color);
    0
}

fn lib_head(state: &mut ExeState, turtle:&mut coordinate, root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) -> i32 {
    println!("heading of the turtle is {}",turtle.head_degree);
    0
}
// ANCHOR_END: print
fn match_color(value: i32) -> plotters::prelude::RGBColor {
    match value {
        1 => plotters::prelude::RGBColor(0, 0, 0), // BLACK
        2 => plotters::prelude::RGBColor(0, 0, 255), // BLUE
        3 => plotters::prelude::RGBColor(0, 255, 255), // CYAN
        4 => plotters::prelude::RGBColor(0, 128, 0), // GREEN
        5 => plotters::prelude::RGBColor(255, 0, 0), // RED
        6 => plotters::prelude::RGBColor(255, 0, 255), // MAGENTA
        7 => plotters::prelude::RGBColor(255, 255, 0), // YELLOW
        8 => plotters::prelude::RGBColor(255, 255, 255), // WHITE
        9 => plotters::prelude::RGBColor(165, 42, 42), // BROWN
        10 => plotters::prelude::RGBColor(210, 180, 140), // TAN
        11 => plotters::prelude::RGBColor(34, 139, 34), // FOREST
        12 => plotters::prelude::RGBColor(0, 255, 255), // AQUA
        13 => plotters::prelude::RGBColor(250, 128, 114), // SALMON
        14 => plotters::prelude::RGBColor(128, 0, 128), // PURPLE
        15 => plotters::prelude::RGBColor(255, 165, 0), // ORANGE
        16 => plotters::prelude::RGBColor(128, 128, 128), // GREY
        _ => panic!("Not a valid color"), // default color
    }
}
// ANCHOR: state
pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec::<Value>,
    func_index: usize,
}
// ANCHOR_END: state

// ANCHOR: new
impl ExeState {
    pub fn new(turtle:&mut coordinate) -> Self {
        let mut globals = HashMap::new();
        globals.insert(String::from("PENDOWN"), Value::Function(start_draw));
        globals.insert(String::from("PENUP"), Value::Function(stop_draw));
        globals.insert(String::from("FORWARD"), Value::Function(lib_forward));
        globals.insert(String::from("BACK"), Value::Function(lib_back));
        globals.insert(String::from("SETX"), Value::Function(lib_setx));
        globals.insert(String::from("SETY"), Value::Function(lib_sety));
        globals.insert(String::from("LEFT"), Value::Function(lib_left));
        globals.insert(String::from("RIGHT"), Value::Function(lib_right));
        globals.insert(String::from("SETPENCOLOR"), Value::Function(lib_setpencolor));
        globals.insert(String::from("SETHEADING"), Value::Function(lib_setheading));
        globals.insert(String::from("TURN"), Value::Function(lib_turn));
        globals.insert(String::from("MAKE"), Value::Function(lib_make));
        globals.insert(String::from("XOR"), Value::Function(lib_xor));
        globals.insert(String::from("YOR"), Value::Function(lib_yor));
        globals.insert(String::from("COLOR"), Value::Function(lib_color));
        globals.insert(String::from("HEADING"), Value::Function(lib_head));
        ExeState {
            globals,
            stack: Vec::new(),
            func_index: 0,
        }
    }
// ANCHOR_END: new

// ANCHOR: execute
    pub fn execute(&mut self, proto: &ParseProto, turtle:&mut coordinate,root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>) {
        for code in proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name) => {
                    let name = &proto.constants[name as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Move(dst, src) => {
                    let v = self.stack[src as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Call(func, _) => {
                    self.func_index = func as usize;
                    let func = &self.stack[self.func_index];
                    if let Value::Function(f) = func {
                        f(self, turtle,root);
                    } else {
                        panic!("invalid function call: {func:?}");
                    }
                }
                ByteCode::Operate(func) =>{
                    self.func_index = func as usize;
                    let func = &self.stack[self.func_index];
                    if let Value::Function(f) = func {
                        f(self,turtle,root);
                    } else {
                        panic!("invalid function operate: {func:?}");
                    }
                }
                ByteCode::SetGlobal(value_index, name_index) => {
                    if value_index as usize >= proto.constants.len() || name_index as usize >= proto.constants.len() {
                        panic!("Constant index out of bounds");
                    }
    
                    let name = &proto.constants[name_index as usize];
                    let value = &proto.constants[value_index as usize];
                    if let Value::String(key) = name {
                        // 从常量表中取出变量值
                        self.globals.insert(key.clone(), value.clone());
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
            }
        }
    }
// ANCHOR_END: execute

// ANCHOR: set_stack
    fn set_stack(&mut self, dst: u8, v: Value) {
        let dst = dst as usize;
        match dst.cmp(&self.stack.len()) {
            Ordering::Equal => self.stack.push(v),
            Ordering::Less => self.stack[dst] = v,
            Ordering::Greater => panic!("fail in set_stack"),
        }
    }

    
// ANCHOR_END: set_stack
}