#![allow(non_snake_case, dead_code, unused_variables, non_camel_case_types)]

use crate::InstructionDecoder::{
    Instruction, 
    Generic_Op,
    R_Type, 
    I_Type, 
    S_Type, 
    B_Type,
    U_Type,
    J_Type
};

/// index  | register | ABI   | Use                                 | preserved
///   0x0  |   x0     | zero  | hardwired to `0`                    |    n/a
///   0x1  |   x1     |  ra   | return address for jumps            |    no
///   0x2  |   x2     |  sp   | stack pointer                       |    yes
///   0x3  |   x3     |  gp   | global pointer                      |    n/a
///   0x4  |   x4     |  tp   | thread pointer                      |    n/a
///   0x5  |   x5     |  t0   | temporary register                  |    no
///   0x6  |   x6     |  t1   | temporary register                  |    no
///   0x7  |   x7     |  t2   | temperary register                  |    no
///   0x8  |   x8     | s0/fp | saved register 0 / frame pointer    |    yes
///   0x9  |   x9     |  s1   | saved register 1                    |    yes
///   0xa  |   x10    |  a0   | return value or function argument 0 |    no
///   0xb  |   x11    |  a1   | return value or function argument 1 |    no
///   0xc  |   x12    |  a2   | function argument 2                 |    no
///   0xd  |   x13    |  a3   | function argument 3                 |    no
///   0xe  |   x14    |  a4   | function argument 4                 |    no
///   0xf  |   x15    |  a5   | function argument 5                 |    no
///  0x10  |   x16    |  a6   | function argument 6                 |    no
///  0x11  |   x17    |  a7   | function argument 7                 |    no
///  0x12  |   x18    |  s2   | saved register 2                    |    yes
///  0x13  |   x19    |  s3   | saved register 3                    |    yes
///  0x14  |   x20    |  s4   | saved register 4                    |    yes
///  0x15  |   x21    |  s5   | saved register 5                    |    yes
///  0x16  |   x22    |  s6   | saved register 6                    |    yes
///  0x17  |   x23    |  s7   | saved register 7                    |    yes
///  0x18  |   x24    |  s8   | saved register 8                    |    yes
///  0x19  |   x25    |  s9   | saved register 9                    |    yes
///  0x1a  |   x26    |  s10  | saved register 10                   |    yes
///  0x1b  |   x27    |  s11  | saved register 11                   |    yes
///  0x1c  |   x28    |  t3   | temporary register 3                |    no
///  0x1d  |   x29    |  t4   | temporary register 4                |    no
///  0x1e  |   x30    |  t5   | temporary register 5                |    no
///  0x1f  |   x31    |  t6   | temporary register 6                |    no
///  0x20? |   pc     |       | program counter                     |    n/a

pub mod InstructionDecoder {
    #[derive(Debug)]
    pub struct Generic_Op {
        pub opcode: usize,
        pub raw: usize,
    }

    impl Generic_Op {
        pub fn new(data: usize) -> Self {
            Self {
                opcode: data & 0x7F,
                raw: data,
            }
        }

        pub fn default() -> Self {
            Self { opcode: 0, raw: 0 }
        }
    }

    #[derive(Debug)]
    pub struct R_Type {
        pub funct7: usize,
        pub rs3: usize,
        pub funct2: usize,
        pub rs2: usize,
        pub rs1: usize,
        pub funct3: usize,
        pub rd: usize,
        pub opcode: usize,
        pub raw: usize,
    }

    impl R_Type {
        pub fn new(raw: usize) -> Self {
            Self {
                funct7: (raw & (0x7F << 25)) >> 25,
                rs3: (raw & (0b11111 << 27)) >> 27,
                funct2: (raw & (0b11 << 25)) >> 25,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            }
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            Self {
                funct7: (raw & (0x7F << 25)) >> 25,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
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
        pub parsed_imm: usize,
        pub imm_110: usize,
        pub rs1: usize,
        pub funct3: usize,
        pub rd: usize,
        pub opcode: usize,
        pub raw: usize,
    }

    impl I_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_110: (raw & (0x0FFF << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
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
                imm_110: (raw & (0x0FFF << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
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
        pub parsed_imm: usize,
        pub imm_115: usize,
        pub rs2: usize,
        pub rs1: usize,
        pub funct3: usize,
        pub imm_40: usize,
        pub opcode: usize,
        pub raw: usize,
    }

    impl S_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_115: (raw & (0x7F << 25)) >> 25,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                imm_40: (raw & (0x1F) << 7) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = (tmp.imm_115 << 5) | tmp.imm_40;
            let out = tmp;
            out
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_115: (raw & (0x7F << 25)) >> 25,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                imm_40: (raw & (0x1F) << 7) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = (tmp.imm_115 << 7) | tmp.imm_40;
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
        pub parsed_imm: usize,
        pub imm_12_105: usize,
        pub rs2: usize,
        pub rs1: usize,
        pub funct3: usize,
        pub imm_41_11: usize,
        pub opcode: usize,
        pub raw: usize,
    }

    impl B_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_12_105: (raw & (0x7F << 25)) >> 25,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                imm_41_11: (raw & (0x1F << 7)) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = (((tmp.imm_12_105 & 0x40) >> 6) << 12)
                | (((tmp.imm_41_11 & 0x01) >> 0) << 11)
                | (((tmp.imm_12_105 & 0x3F) >> 0) << 5)
                | (((tmp.imm_41_11 & 0x1E) >> 1) << 1);
            let out = tmp;
            out
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_12_105: (raw & (0x7F << 25)) >> 25,
                rs2: (raw & (0x1F << 20)) >> 20,
                rs1: (raw & (0x1F << 15)) >> 15,
                funct3: (raw & (0x07 << 12)) >> 12,
                imm_41_11: (raw & (0x1F << 7)) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            tmp.parsed_imm = (((tmp.imm_12_105 & 0x40) >> 6) << 12)
                | (((tmp.imm_41_11 & 0x01) >> 0) << 11)
                | (((tmp.imm_12_105 & 0x3F) >> 0) << 5)
                | (((tmp.imm_41_11 & 0x1E) >> 1) << 1);
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
        pub parsed_imm: usize,
        pub imm_3112: usize,
        pub rd: usize,
        pub opcode: usize,
        pub raw: usize,
    }

    impl U_Type {
        pub fn new(raw: usize) -> Self {
            Self {
                parsed_imm: (raw & (0x0FFFFF << 12)) >> 12,
                imm_3112: (raw & (0x0FFFFF << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            }
        }

        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            Self {
                parsed_imm: (raw & (0x0FFFFF << 12)) >> 12,
                imm_3112: (raw & (0x0FFFFF << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
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
        pub parsed_imm: usize,
        pub imm_20_101_11_1912: usize,
        pub rd: usize,
        pub opcode: usize,
        pub raw: usize,
    }

    impl J_Type {
        pub fn new(raw: usize) -> Self {
            let mut tmp = Self {
                parsed_imm: 0,
                imm_20_101_11_1912: (raw & (0x0FFFFF << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            // parse the immediate.
            tmp.parsed_imm = ((tmp.imm_20_101_11_1912 & 0x080000) >> 19) << 20
                | ((tmp.imm_20_101_11_1912 & 0xFF) >> 0) << 19
                | ((tmp.imm_20_101_11_1912 & 0x0100) >> 8) << 11
                | ((tmp.imm_20_101_11_1912 & 0x07FE00) >> 9) << 1;
            let out = tmp;
            out
        }
        pub fn new_from_generic(op: Generic_Op) -> Self {
            let raw = op.raw;
            let mut tmp = Self {
                parsed_imm: 0,
                imm_20_101_11_1912: (raw & (0xFFFFF << 12)) >> 12,
                rd: (raw & (0x1F << 7)) >> 7,
                opcode: (raw & (0x7F << 0)) >> 0,
                raw: raw,
            };
            // parse the immediate.
            tmp.parsed_imm = ((tmp.imm_20_101_11_1912 & 0x080000) >> 19) << 20
                | ((tmp.imm_20_101_11_1912 & 0xFF) >> 0) << 19
                | ((tmp.imm_20_101_11_1912 & 0x0100) >> 8) << 11
                | ((tmp.imm_20_101_11_1912 & 0x07FE00) >> 9) << 1;
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
        // RV32I Base Instruction Set
        LUI(U_Type),
        AUIPC(U_Type),
        JAL(J_Type),
        JALR(I_Type),
        BEQ(B_Type),
        BNE(B_Type),
        BLT(B_Type),
        BGE(B_Type),
        BLTU(B_Type),
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
        ADDI(I_Type),
        SLTI(I_Type),
        SLTIU(I_Type),
        XORI(I_Type),
        ORI(I_Type),
        ANDI(I_Type),
        SLLI(R_Type),
        SRLI(R_Type),
        SRAI(R_Type),
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
        // RV64I Base Instruction Set
        ADDIW(I_Type),
        SLLIW(R_Type),
        SRLIW(R_Type),
        SRAIW(R_Type),
        ADDW(R_Type),
        SUBW(R_Type),
        SLLW(R_Type),
        SRLW(R_Type),
        SRAW(R_Type),
        // RV32/RV64 Zifencei Extension
        FENCEI(I_Type),
        // RV32/RV64 Zicsr Extension
        CSRRW(I_Type),
        CSRRS(I_Type),
        CSRRC(I_Type),
        CSRRWI(I_Type),
        CSRRSI(I_Type),
        CSRRCI(I_Type),
        // RV32M Extension
        MUL(R_Type),
        MULH(R_Type),
        MULHSU(R_Type),
        MULHU(R_Type),
        DIV(R_Type),
        DIVU(R_Type),
        REM(R_Type),
        REMU(R_Type),
        // RV64M Extension
        MULW(R_Type),
        DIVW(R_Type),
        DIVUW(R_Type),
        REMW(R_Type),
        REMUW(R_Type),
        // RV32A Extension
        LRW(R_Type),
        SCW(R_Type),
        AMOSWAPW(R_Type),
        AMOADDW(R_Type),
        AMOXORW(R_Type),
        AMOANDW(R_Type),
        AMOORW(R_Type),
        AMOMINW(R_Type),
        AMOMAXW(R_Type),
        AMOMINUW(R_Type),
        AMOMAXUW(R_Type),
        // RV64A Extension
        LRD(R_Type),
        SCD(R_Type),
        AMOSWAPD(R_Type),
        AMOADDD(R_Type),
        AMOXORD(R_Type),
        AMOANDD(R_Type),
        AMOORD(R_Type),
        AMOMIND(R_Type),
        AMOMAXD(R_Type),
        AMOMINUD(R_Type),
        AMOMAXUD(R_Type),
        // RV32F Extension
        FLW(I_Type),
        FSW(S_Type),
        FMADDS(R_Type),
        FMSUBS(R_Type),
        FNMSUBS(R_Type),
        FNMADDS(R_Type),
        FADDS(R_Type),
        FSUBS(R_Type),
        FMULS(R_Type),
        FDIVS(R_Type),
        FSQRT(R_Type),
        FSGNJS(R_Type),
        FSGNJNS(R_Type),
        FSGNJXS(R_Type),
        FMINS(R_Type),
        FMAXS(R_Type),
        FCVTWS(R_Type),
        FCVTWUS(R_Type),
        FMVXW(R_Type),
        FEQS(R_Type),
        FLTS(R_Type),
        FLES(R_Type),
        FCLASSS(R_Type),
        FCVTSW(R_Type),
        FCVTSWU(R_Type),
        FMVWX(R_Type),
        // RV64F Extension
        FCVTLS(R_Type),
        FCVTLUS(R_Type),
        FCVTSL(R_Type),
        FCVTSLU(R_Type),
        FLD(R_Type),
        // RV32D Extension
        FSD(R_Type),
        FMADDD(R_Type),
        FMSUBD(R_Type),
        FNMSUBD(R_Type),
        FNMADDD(R_Type),
        FADDD(R_Type),
        FSUBD(R_Type),
        FMULD(R_Type),
        FDIVD(R_Type),
        FSQRTD(R_Type),
        FSGNJD(R_Type),
        FSGNJND(R_Type),
        FSGNJXD(R_Type),
        FMIND(R_Type),
        FMAXD(R_Type),
        FCVTSD(R_Type),
        FCVTDS(R_Type),
        FEQD(R_Type),
        FLTD(R_Type),
        FLED(R_Type),
        FCLASSD(R_Type),
        FCVTWD(R_Type),
        FCVTWUD(R_Type),
        FCVTDW(R_Type),
        FCVTDWU(R_Type),
        // RV64D Extension
        FCVTLD(R_Type),
        FCVTLUD(R_Type),
        FMVXD(R_Type),
        FCVTDL(R_Type),
        FCVTDLU(R_Type),
        FMVDX(R_Type),
        // TODO: Add support for the `C` Extension.
    }

    impl Instruction {
        pub fn parse_instruction(inst: usize) -> Option<Self> {
            let R_TYPE: R_Type = R_Type::new(inst);
            let I_TYPE: I_Type = I_Type::new(inst);
            let S_TYPE: S_Type = S_Type::new(inst);
            let B_TYPE: B_Type = B_Type::new(inst);
            let U_TYPE: U_Type = U_Type::new(inst);
            let J_TYPE: J_Type = J_Type::new(inst);

            let opcode = R_TYPE.opcode;
            let funct7 = R_TYPE.funct7;
            let funct6 = (funct7 & 0b1111110) >> 1;
            let funct3 = R_TYPE.funct3;
            let imm110 = I_TYPE.imm_110;
            let funct5 = (funct7 & 0b1111100) >> 2;
            let rs2    = R_TYPE.rs2;
            let funct2 = funct7 & 0b0000011;
            let rs3    = funct5;
            if (opcode & 0b11) != 0b11 {
                // opcode is a compressed opcode, and should throw an error.
                unimplemented!("16 bit compressed op: {opcode}");
            }

            match (imm110, funct6, funct7, funct3, opcode) {
                // `RV32I` Base Instructions
                (            _,       _,         _,     _, 0b0110111) => {
                    return Some(Instruction::LUI(U_TYPE));
                },
                (            _,       _,         _,     _, 0b0010111) => {
                    return Some(Instruction::AUIPC(U_TYPE));
                },
                (            _,       _,         _,     _, 0b1101111) => {
                    return Some(Instruction::JAL(J_TYPE));
                },
                (            _,       _,         _, 0b000, 0b1100111) => {
                    return Some(Instruction::JALR(I_TYPE));
                },
                (            _,       _,         _, 0b000, 0b1100011) => {
                    return Some(Instruction::BEQ(B_TYPE));
                },
                (            _,       _,         _, 0b001, 0b1100011) => {
                    return Some(Instruction::BNE(B_TYPE));
                },
                (            _,       _,         _, 0b100, 0b1100011) => {
                    return Some(Instruction::BLT(B_TYPE));
                },
                (            _,       _,         _, 0b101, 0b1100011) => {
                    return Some(Instruction::BGE(B_TYPE));
                },
                (            _,       _,         _, 0b110, 0b1100011) => {
                    return Some(Instruction::BLTU(B_TYPE));
                },
                (            _,       _,         _, 0b111, 0b1100011) => {
                    return Some(Instruction::BGEU(B_TYPE));
                },
                (            _,       _,         _, 0b000, 0b0000011) => {
                    return Some(Instruction::LB(I_TYPE));
                },
                (            _,       _,         _, 0b001, 0b0000011) => {
                    return Some(Instruction::LH(I_TYPE));
                },
                (            _,       _,         _, 0b010, 0b0000011) => {
                    return Some(Instruction::LW(I_TYPE));
                },
                (            _,       _,         _, 0b100, 0b0000011) => {
                    return Some(Instruction::LBU(I_TYPE));
                },
                (            _,       _,         _, 0b101, 0b0000011) => {
                    return Some(Instruction::LHU(I_TYPE));
                },
                (            _,       _,         _, 0b000, 0b0100011) => {
                    return Some(Instruction::SB(S_TYPE));
                },
                (            _,       _,         _, 0b001, 0b0100011) => {
                    return Some(Instruction::SH(S_TYPE));
                },
                (            _,       _,         _, 0b010, 0b0100011) => {
                    return Some(Instruction::SW(S_TYPE));
                },
                (            _,       _,         _, 0b000, 0b0010011) => {
                    return Some(Instruction::ADDI(I_TYPE));
                },
                (            _,       _,         _, 0b010, 0b0010011) => {
                    return Some(Instruction::SLTI(I_TYPE));
                },
                (            _,       _,         _, 0b011, 0b0010011) => {
                    return Some(Instruction::SLTIU(I_TYPE));
                },
                (            _,       _,         _, 0b100, 0b0010011) => {
                    return Some(Instruction::XORI(I_TYPE));
                },
                (            _,       _,         _, 0b110, 0b0010011) => {
                    return Some(Instruction::ORI(I_TYPE));
                },
                (            _,       _,         _, 0b111, 0b0010011) => {
                    return Some(Instruction::ANDI(I_TYPE));
                },
                (            _,       _, 0b0000000, 0b001, 0b0010011) => {
                    return Some(Instruction::SLLI(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b101, 0b0010011) => {
                    return Some(Instruction::SRLI(R_TYPE));
                },
                (            _,       _, 0b0100000, 0b101, 0b0010011) => {
                    return Some(Instruction::SRAI(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b000, 0b0110011) => {
                    return Some(Instruction::ADD(R_TYPE));
                },
                (            _,       _, 0b0100000, 0b000, 0b0110011) => {
                    return Some(Instruction::SUB(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b001, 0b0110011) => {
                    return Some(Instruction::SLL(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b010, 0b0110011) => {
                    return Some(Instruction::SLT(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b011, 0b0110011) => {
                    return Some(Instruction::SLTU(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b100, 0b0110011) => {
                    return Some(Instruction::XOR(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b101, 0b0110011) => {
                    return Some(Instruction::SRL(R_TYPE));
                },
                (            _,       _, 0b0100000, 0b101, 0b0110011) => {
                    return Some(Instruction::SRA(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b110, 0b0110011) => {
                    return Some(Instruction::OR(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b111, 0b0110011) => {
                    return Some(Instruction::AND(R_TYPE));
                },
                (            _,       _,         _, 0b000, 0b0001111) => {
                    return Some(Instruction::FENCE(I_TYPE));
                },
                (0b00000000000,       _,         _, 0b000, 0b1110011) => {
                    return Some(Instruction::ECALL(I_TYPE));
                },
                (0b00000000001,       _,         _, 0b000, 0b1110011) => {
                    return Some(Instruction::EBREAK(I_TYPE));
                },
                // `RV64I` Base Instructions
                (            _,       _,         _, 0b110, 0b0000011) => {
                    return Some(Instruction::LWU(I_TYPE));
                },
                (            _,       _,         _, 0b011, 0b0000011) => {
                    return Some(Instruction::LD(I_TYPE));
                },
                (            _,       _,         _, 0b011, 0b0100011) => {
                    return Some(Instruction::SD(S_TYPE));
                },
                (            _,       _,         _, 0b000, 0b0011011) => {
                    return Some(Instruction::ADDIW(I_TYPE));
                },
                (            _,       _, 0b0000000, 0b001, 0b0011011) => {
                    return Some(Instruction::SLLIW(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b101, 0b0011011) => {
                    return Some(Instruction::SRLIW(R_TYPE));
                },
                (            _,       _, 0b0100000, 0b101, 0b0011011) => {
                    return Some(Instruction::SRAIW(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b000, 0b0111011) => {
                    return Some(Instruction::ADDW(R_TYPE));
                },
                (            _,       _, 0b0100000, 0b000, 0b0111011) => {
                    return Some(Instruction::SUBW(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b001, 0b0111011) => {
                    return Some(Instruction::SLLW(R_TYPE));
                },
                (            _,       _, 0b0000000, 0b101, 0b0111011) => {
                    return Some(Instruction::SRLW(R_TYPE));
                },
                (            _,       _, 0b0100000, 0b101, 0b0111011) => {
                    return Some(Instruction::SRAW(R_TYPE));
                },
                // `Zifencei` Extension
                (            _,       _,         _, 0b001, 0b0001111) => {
                    return Some(Instruction::FENCEI(I_TYPE));
                },
                // `Zicsr` Extension
                (            _,       _,         _, 0b001, 0b1110011) => {
                    return Some(Instruction::CSRRW(I_TYPE));
                },
                (            _,       _,         _, 0b010, 0b1110011) => {
                    return Some(Instruction::CSRRS(I_TYPE));
                },
                (            _,       _,         _, 0b011, 0b1110011) => {
                    return Some(Instruction::CSRRC(I_TYPE));
                },
                (            _,       _,         _, 0b101, 0b1110011) => {
                    return Some(Instruction::CSRRWI(I_TYPE));
                },
                (            _,       _,         _, 0b110, 0b1110011) => {
                    return Some(Instruction::CSRRSI(I_TYPE));
                },
                (            _,       _,         _, 0b111, 0b1110011) => {
                    return Some(Instruction::CSRRCI(I_TYPE));
                },
                // `M` Extension
                (            _,       _, 0b0000001, 0b000, 0b0110011) => {
                    return Some(Instruction::MUL(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b001, 0b0110011) => {
                    return Some(Instruction::MULH(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b010, 0b0110011) => {
                    return Some(Instruction::MULHSU(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b011, 0b0110011) => {
                    return Some(Instruction::MULHU(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b100, 0b0110011) => {
                    return Some(Instruction::DIV(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b101, 0b0110011) => {
                    return Some(Instruction::DIVU(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b110, 0b0110011) => {
                    return Some(Instruction::REM(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b111, 0b0110011) => {
                    return Some(Instruction::REMU(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b000, 0b0111011) => {
                    return Some(Instruction::MULW(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b100, 0b0111011) => {
                    return Some(Instruction::DIVW(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b101, 0b0111011) => {
                    return Some(Instruction::DIVUW(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b110, 0b0111011) => {
                    return Some(Instruction::REMW(R_TYPE));
                },
                (            _,       _, 0b0000001, 0b111, 0b0111011) => {
                    return Some(Instruction::REMUW(R_TYPE));
                },
                // `A` Extension
                (            _,       _,         _, 0b010, 0b0101111) => {
                    return Some(Instruction::LRW(R_TYPE));
                },
                _ => unimplemented!(),
            }
        }
    }
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
        0b0110111 => {
            // LUI
            return InstructionType::U_TYPE(U_Type::new(generic.raw));
        }
        0b0010111 => {
            // AUIPC
            return InstructionType::U_TYPE(U_Type::new(generic.raw));
        }
        0b1101111 => {
            // JAL
            return InstructionType::J_TYPE(J_Type::new(generic.raw));
        }
        0b1100111 => {
            // JALR
            return InstructionType::I_TYPE(I_Type::new(generic.raw));
        }
        0b1100011 => {
            // BEQ, BNE, BLT, BGE, BLTU, BGEU
            return InstructionType::B_TYPE(B_Type::new(generic.raw));
        }
        0b0000011 => {
            // LB, LH, LW, LBU, LHU, LWU, LD
            return InstructionType::I_TYPE(I_Type::new(generic.raw));
        }
        0b0100011 => {
            // SB, SH, SW, SD
            return InstructionType::S_TYPE(S_Type::new(generic.raw));
        }
        0b0010011 => {
            // ADDI, SLTI, SLTIU, XORI, ORI, ANDI,
            // SLLI SRLI, SRAI, SLLI, SRLI, SRAI
            // (The last 6 are a special case that isn't
            // actually an I-Type, but it is close enough.)
            return InstructionType::I_TYPE(I_Type::new(generic.raw));
        }
        0b0110011 => {
            // ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
            return InstructionType::R_TYPE(R_Type::new(generic.raw));
        }
        0b0001111 => {
            // FENCE (Not actually I-type, but close enough
            return InstructionType::I_TYPE(I_Type::new(generic.raw));
        }
        0b1110011 => {
            // ECALL, EBREAK
            // (Not actually I-type, but close enough)
            return InstructionType::I_TYPE(I_Type::new(generic.raw));
        }
        // RV64I Base InstructionType Set (in addition to RV32I)
        // LWU, LD +=> 0b0000011
        // SD +=> 0b0100011
        // SLLI, SRLI, SRAI +=> 0b0010011
        0b0011011 => {
            // ADDIW, SLLIW, SRLIW, SRAIW
            //   (rs2 is actually shamt)
            // MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU
            return InstructionType::R_TYPE(R_Type::new(generic.raw));
        }
        0b0111011 => {
            // ADDW, SUBW, SLLW, SRLW, SRAW
            return InstructionType::R_TYPE(R_Type::new(generic.raw));
        }
        // RV32M Standard Extension
        // MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU +=> 0b0110011
        // // RV64M Standard Extension (in addition to RV32M)
        // MULW, DIVW, DIVUW, REMW, REMUW +=> 0b0111011
        // TODO: Add support for the `C` Extension.
        _ => {
            unimplemented!("{:#?}", generic)
        }
    }
}

fn main() {
    let j = Instruction::parse_instruction(0x00002e17);
    println!("{:#x?}", j);
}
