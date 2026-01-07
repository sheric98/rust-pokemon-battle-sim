use crate::{
    common::context::MoveContext,
    core::{
        pokemon::pokemon::Pokemon,
        pokemove::{move_name::MoveName, pokemove::PokeMove},
        util::damage_utils,
    },
};

pub enum Payload {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Bool(bool),
    F32(f32),
    F64(f64),
    VecF32(Vec<f32>),
}

impl Payload {
    pub fn as_u8(&self) -> u8 {
        match self {
            Payload::U8(v) => *v,
            _ => panic!("Payload is not a u8"),
        }
    }

    pub fn as_u16(&self) -> u16 {
        match self {
            Payload::U16(v) => *v,
            _ => panic!("Payload is not a u16"),
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Payload::U32(v) => *v,
            _ => panic!("Payload is not a u32"),
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self {
            Payload::U64(v) => *v,
            _ => panic!("Payload is not a u64"),
        }
    }

    pub fn as_i8(&self) -> i8 {
        match self {
            Payload::I8(v) => *v,
            _ => panic!("Payload is not a i8"),
        }
    }

    pub fn as_i16(&self) -> i16 {
        match self {
            Payload::I16(v) => *v,
            _ => panic!("Payload is not a i16"),
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            Payload::I32(v) => *v,
            _ => panic!("Payload is not a i32"),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Payload::I64(v) => *v,
            _ => panic!("Payload is not a i64"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Payload::Bool(v) => *v,
            _ => panic!("Payload is not a bool"),
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            Payload::F32(v) => *v,
            _ => panic!("Payload is not a f32"),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Payload::F64(v) => *v,
            _ => panic!("Payload is not a f64"),
        }
    }

    pub fn as_vec_f32(&self) -> &Vec<f32> {
        match self {
            Payload::VecF32(v) => v,
            _ => panic!("Payload is not a Vec<f32>"),
        }
    }

    pub fn as_vec_f32_mut(&mut self) -> &mut Vec<f32> {
        match self {
            Payload::VecF32(v) => v,
            _ => panic!("Payload is not a Vec<f32>"),
        }
    }
}

pub struct PayloadMoveQuery {
    pub context: MoveContext,
    pub payload: Payload,
}

impl PayloadMoveQuery {
    pub fn new(context: MoveContext, payload: Payload) -> Self {
        Self { context, payload }
    }

    pub fn u8(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::U8(0),
        }
    }

    pub fn u8_with_default(context: MoveContext, default: u8) -> Self {
        Self {
            context,
            payload: Payload::U8(default),
        }
    }

    pub fn u16(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::U16(0),
        }
    }

    pub fn u16_with_default(context: MoveContext, default: u16) -> Self {
        Self {
            context,
            payload: Payload::U16(default),
        }
    }

    pub fn u32(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::U32(0),
        }
    }

    pub fn u32_with_default(context: MoveContext, default: u32) -> Self {
        Self {
            context,
            payload: Payload::U32(default),
        }
    }

    pub fn u64(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::U64(0),
        }
    }

    pub fn u64_with_default(context: MoveContext, default: u64) -> Self {
        Self {
            context,
            payload: Payload::U64(default),
        }
    }

    pub fn i8(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::I8(0),
        }
    }

    pub fn i8_with_default(context: MoveContext, default: i8) -> Self {
        Self {
            context,
            payload: Payload::I8(default),
        }
    }

    pub fn i16(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::I16(0),
        }
    }

    pub fn i16_with_default(context: MoveContext, default: i16) -> Self {
        Self {
            context,
            payload: Payload::I16(default),
        }
    }

    pub fn i32(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::I32(0),
        }
    }

    pub fn i32_with_default(context: MoveContext, default: i32) -> Self {
        Self {
            context,
            payload: Payload::I32(default),
        }
    }

    pub fn i64(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::I64(0),
        }
    }

    pub fn i64_with_default(context: MoveContext, default: i64) -> Self {
        Self {
            context,
            payload: Payload::I64(default),
        }
    }

    pub fn f32(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::F32(0.0),
        }
    }

    pub fn f32_with_default(context: MoveContext, default: f32) -> Self {
        Self {
            context,
            payload: Payload::F32(default),
        }
    }

    pub fn f64(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::F64(0.0),
        }
    }

    pub fn f64_with_default(context: MoveContext, default: f64) -> Self {
        Self {
            context,
            payload: Payload::F64(default),
        }
    }

    pub fn vec_f32(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::VecF32(Vec::new()),
        }
    }

    pub fn vec_f32_with_default(context: MoveContext, default: Vec<f32>) -> Self {
        Self {
            context,
            payload: Payload::VecF32(default),
        }
    }

    pub fn bool(context: MoveContext) -> Self {
        Self {
            context,
            payload: Payload::Bool(false),
        }
    }

    pub fn bool_with_default(context: MoveContext, default: bool) -> Self {
        Self {
            context,
            payload: Payload::Bool(default),
        }
    }

    pub fn get_u8(&self) -> u8 {
        self.payload.as_u8()
    }

    pub fn get_u16(&self) -> u16 {
        self.payload.as_u16()
    }

    pub fn get_u32(&self) -> u32 {
        self.payload.as_u32()
    }

    pub fn get_u64(&self) -> u64 {
        self.payload.as_u64()
    }

    pub fn get_i8(&self) -> i8 {
        self.payload.as_i8()
    }

    pub fn get_i16(&self) -> i16 {
        self.payload.as_i16()
    }

    pub fn get_i32(&self) -> i32 {
        self.payload.as_i32()
    }

    pub fn get_i64(&self) -> i64 {
        self.payload.as_i64()
    }

    pub fn get_f32(&self) -> f32 {
        self.payload.as_f32()
    }

    pub fn get_f64(&self) -> f64 {
        self.payload.as_f64()
    }

    pub fn get_bool(&self) -> bool {
        self.payload.as_bool()
    }

    pub fn get_vec_f32(&mut self) -> &mut Vec<f32> {
        self.payload.as_vec_f32_mut()
    }

    pub fn as_combined_modifier(&self) -> u32 {
        damage_utils::rounded_damage_from_modifiers(self.payload.as_vec_f32())
    }
}
