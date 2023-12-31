#![allow(non_snake_case, dead_code, unused_variables, non_camel_case_types)]


pub mod IL {
    pub struct Instruction {
        
    }

    pub struct Graph {
        data: Vec<Instruction>,
    }
}

pub mod InstructionDecoder {
    #[derive(Debug)]
    pub struct Generic_Op {
        opcode: usize,
        raw: usize,
    }

    impl Generic_Op {
        pub fn new(data: usize) -> Self {
            Self {
                opcode: data & 0x7F,
                raw: data,
            }
        }

        pub fn default() -> Self {
            Self {
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct R_Type {
        funct7: usize,
        rs2: usize,
        rs1: usize,
        funct3: usize,
        rd: usize,
        opcode: usize,
        raw: usize,
    }

    impl R_Type {
        pub fn new(raw: usize) -> Self {
            Self {
                funct7: (raw & (0x7F << 24)) >> 24,
                rs2: (raw & (0x1F << 19)) >> 19,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            }
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            Self {
                funct7: (raw & (0x7F << 24)) >> 24,
                rs2: (raw & (0x1F << 19)) >> 19,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            }
        }

        pub fn default() -> Self {
            Self {
                funct7: 0,
                rs2: 0,
                rs1: 0,
                funct3: 0,
                rd: 0,
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct I_Type {
        parsed_imm: usize,
        imm_110: usize,
        rs1: usize,
        funct3: usize,
        rd: usize,
        opcode: usize,
        raw: usize,
    }

    impl I_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_110: (raw & (0x0FFF << 19)) >> 19,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = tmp.imm_110;
            let out = tmp;
            out
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_110: (raw & (0x0FFF << 19)) >> 19,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = tmp.imm_110;
            let out = tmp;
            out
        }

        pub fn default() -> Self {
            Self {
                parsed_imm: 0,
                imm_110: 0,
                rs1: 0,
                funct3: 0,
                rd: 0,
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct S_Type {
        parsed_imm: usize,
        imm_115: usize,
        rs2: usize,
        rs1: usize,
        funct3: usize,
        imm_40: usize,
        opcode: usize,
        raw: usize,
    }

    impl S_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_115: (raw & (0x7F << 24)) >> 24,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                imm_40: (raw & (0x1F) << 6) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = (tmp.imm_115 << 4) | tmp.imm_40;
            let out = tmp;
            out
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_115: (raw & (0x7F << 24)) >> 24,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                imm_40: (raw & (0x1F) << 6) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = (tmp.imm_115 << 4) | tmp.imm_40;
            let out = tmp;
            out
        }

        pub fn default() -> Self {
            Self {
                parsed_imm: 0,
                imm_115: 0,
                rs2: 0,
                rs1: 0,
                funct3: 0,
                imm_40: 0,
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct B_Type {
        parsed_imm: usize,
        imm_12_105: usize,
        rs2: usize,
        rs1: usize,
        funct3: usize,
        imm_41_11: usize,
        opcode: usize,
        raw: usize,
    }

    impl B_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_12_105: (raw & (0x7F << 24)) >> 24,
                rs2: (raw & (0x1F << 19)) >> 19,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                imm_41_11: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = 
                (((tmp.imm_12_105 & 0x40) >> 6) << 12) |
                (((tmp.imm_41_11 & 0x01) >> 0) << 11) |
                (((tmp.imm_12_105 & 0x3F) >> 0) << 5) |
                (((tmp.imm_41_11 & 0x1E) >> 1) << 1);
            let out = tmp;
            out
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_12_105: (raw & (0x7F << 24)) >> 24,
                rs2: (raw & (0x1F << 19)) >> 19,
                rs1: (raw & (0x1F << 14)) >> 14,
                funct3: (raw & (0x07 << 11)) >> 11,
                imm_41_11: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = 
                (((tmp.imm_12_105 & 0x40) >> 6) << 12) |
                (((tmp.imm_41_11 & 0x01) >> 0) << 11) |
                (((tmp.imm_12_105 & 0x3F) >> 0) << 5) |
                (((tmp.imm_41_11 & 0x1E) >> 1) << 1);
            let out = tmp;
            out
        }

        pub fn default() -> Self {
            Self {
                parsed_imm: 0,
                imm_12_105: 0, 
                rs2: 0,
                rs1: 0,
                funct3: 0,
                imm_41_11: 0,
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct U_Type {
        parsed_imm: usize,
        imm_3112: usize,
        rd: usize,
        opcode: usize,
        raw: usize,
    }

    impl U_Type {
        pub fn new(raw: usize) -> Self {
            Self {
                parsed_imm: (raw & (0x0FFFFF << 11)),
                imm_3112: (raw & (0x0FFFFF << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            }
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            Self {
                parsed_imm: (raw & (0x0FFFFF << 11)),
                imm_3112: (raw & (0x0FFFFF << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            }
        }

        pub fn default() -> Self {
            Self {
                parsed_imm: 0,
                imm_3112: 0,
                rd: 0,
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct J_Type {
        parsed_imm: usize,
        imm_20_101_11_1912: usize,
        rd: usize,
        opcode: usize,
        raw: usize,
    }

    impl J_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_20_101_11_1912: (raw & (0x0FFFFF << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            // parse the immediate.
            tmp.parsed_imm = 
                ((tmp.imm_20_101_11_1912 & 0x080000) >> 19) << 20 |
                ((tmp.imm_20_101_11_1912 & 0xFF) >> 0) << 19 |
                ((tmp.imm_20_101_11_1912 & 0x0100) >> 8) << 11 |
                ((tmp.imm_20_101_11_1912 & 0x07FE00) >> 9) << 1;
            let out = tmp;
            out
        }
        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_20_101_11_1912: (raw & (0xFFFFF << 11)) >> 11,
                rd: (raw & (0x1F << 6)) >> 6,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            // parse the immediate.
            tmp.parsed_imm = 
                ((tmp.imm_20_101_11_1912 & 0x080000) >> 19) << 20 |
                ((tmp.imm_20_101_11_1912 & 0xFF) >> 0) << 19 |
                ((tmp.imm_20_101_11_1912 & 0x0100) >> 8) << 11 |
                ((tmp.imm_20_101_11_1912 & 0x07FE00) >> 9) << 1;
            let out = tmp;
            out
        }
        pub fn default() -> Self {
            Self {
                parsed_imm: 0,
                imm_20_101_11_1912: 0,
                rd: 0,
                opcode: 0,
                raw: 0,
            }
        }
    }

    #[derive(Debug)]
    pub enum Instruction {
        LUI(U_Type),
        AUIPC(U_Type),
        JAL(J_Type),
        JALR(I_Type),
        BEQ(B_Type),
        BNE(B_Type),
        BLT(B_Type),
        BGE(B_Type),
        BLUT(B_Type),
        BGEU(B_Type),
        LB(I_Type),
        LH(I_Type),
        LW(I_Type),
        LBU(I_Type),
        LHU(I_Type),
        LWU(I_Type),
        LD(I_Type),
        SB(S_Type),
        SH(S_Type),
        SW(S_Type),
        SD(S_Type),
        ANDI(I_Type),
        SLTI(I_Type), 
        SLTIU(I_Type), 
        XORI(I_Type), 
        ORI(I_Type),
        SLLI(I_Type),
        SRLI(I_Type),
        SRAI(I_Type),
        ADD(R_Type),
        SUB(R_Type),
        SLL(R_Type),
        SLT(R_Type),
        SLTU(R_Type),
        XOR(R_Type),
        SRL(R_Type),
        SRA(R_Type),
        OR(R_Type),
        AND(R_Type),
        FENCE(I_Type),
        ECALL(I_Type),
        EBREAK(I_Type),
        ADDIW(R_Type),
        SLLIW(R_Type),
        SRLIW(R_Type),
        SRAIW(R_Type),
        MUL(R_Type),
        MULH(R_Type),
        MULHSU(R_Type),
        MULHU(R_Type),
        DIV(R_Type),
        DIVU(R_Type),
        REM(R_Type),
        REMU(R_Type),
        ADDW(R_Type),
        SUBW(R_Type),
        SLLW(R_Type),
        SRLW(R_Type),
        SRAW(R_Type),
        MULW(R_Type),
        DIVW(R_Type),
        DIVUW(R_Type),
        REMW(R_Type),
        REMUW(R_Type),
        // TODO: Add support for the `C` Extension.
    }

    #[derive(Debug)]
    pub enum InstructionType {
        R_TYPE(R_Type),
        I_TYPE(I_Type),
        S_TYPE(S_Type),
        B_TYPE(B_Type),
        U_TYPE(U_Type),
        J_TYPE(J_Type),
    }

    pub fn decode_instruction_type(data: usize) -> InstructionType {
        let generic: Generic_Op = Generic_Op::new(data);
        match generic.opcode {
            // RV32I Base Instruction set
            0b0110111 => { // LUI
                return InstructionType::U_TYPE(U_Type::new(generic.raw));
            },
            0b0010111 => { // AUIPC
                return InstructionType::U_TYPE(U_Type::new(generic.raw));
            },
            0b1101111 => { // JAL
                return InstructionType::J_TYPE(J_Type::new(generic.raw));
            },
            0b1100111 => { // JALR
                return InstructionType::I_TYPE(I_Type::new(generic.raw));
            },
            0b1100011 => { // BEQ, BNE, BLT, BGE, BLTU, BGEU
                return InstructionType::B_TYPE(B_Type::new(generic.raw));
            },
            0b0000011 => { // LB, LH, LW, LBU, LHU, LWU, LD
                return InstructionType::I_TYPE(I_Type::new(generic.raw));
            },
            0b0100011 => { // SB, SH, SW, SD
                return InstructionType::S_TYPE(S_Type::new(generic.raw));
            },
            0b0010011 => { // ANDI, SLTI, SLTIU, XORI, ORI, ANDI, 
                           // SLLI SRLI, SRAI, SLLI, SRLI, SRAI 
                           // (The last 6 are a special case that isn't
                           // actually an I-Type, but it is close enough.)
                return InstructionType::I_TYPE(I_Type::new(generic.raw));
            },
            0b0110011 => { // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
                return InstructionType::R_TYPE(R_Type::new(generic.raw));
            },
            0b0001111 => { // FENCE (Not actually I-type, but close enough
                return InstructionType::I_TYPE(I_Type::new(generic.raw));
            }, 
            0b1110011 => { // ECALL, EBREAK 
                           // (Not actually I-type, but close enough)
                return InstructionType::I_TYPE(I_Type::new(generic.raw));
            },
            // RV64I Base InstructionType Set (in addition to RV32I)
            // LWU, LD +=> 0b0000011 
            // SD +=> 0b0100011
            // SLLI, SRLI, SRAI +=> 0b0010011
            0b0011011 => { // ADDIW, SLLIW, SRLIW, SRAIW 
                           //   (rs2 is actually shamt)
                           // MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU
                return InstructionType::R_TYPE(R_Type::new(generic.raw));
            },
            0b0111011 => { // ADDW, SUBW, SLLW, SRLW, SRAW
                return InstructionType::R_TYPE(R_Type::new(generic.raw));
            },
            // RV32M Standard Extension
            // MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU +=> 0b0110011 
            // // RV64M Standard Extension (in addition to RV32M)
            // MULW, DIVW, DIVUW, REMW, REMUW +=> 0b0111011 
            // TODO: Add support for the `C` Extension.
            _ => {
                unimplemented!("{:#?}", generic)
            },
        }
    }
}

fn main() {
    let j = InstructionDecoder::decode_instruction_type(0x00002517);
    println!("{:#x?}", j);
}
