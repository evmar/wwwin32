use iced_x86::Instruction;

use crate::{registers::Flags, x86::X86, StepResult};

use super::helpers::*;

/// This trait is implemented for u32/u16/u8 and lets us write operations generically
/// over all those bit sizes.
///
/// Even when we need size-specific masks like "the high bit"
/// (which is x.shr(I::bits() - 1))
/// that math optimizes down to the appropriate constant.
pub(crate) trait Int: num_traits::PrimInt {
    fn as_usize(self) -> usize;
    fn bits() -> usize;
}
impl Int for u32 {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn bits() -> usize {
        32
    }
}
impl Int for u16 {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn bits() -> usize {
        16
    }
}
impl Int for u8 {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn bits() -> usize {
        8
    }
}

// pub(crate) for use in the test opcode impl.
pub(crate) fn and<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x & y;
    // XXX More flags.
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, (result >> (I::bits() - 1)).is_one());
    flags.set(Flags::OF, false);
    result
}

pub fn and_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate32();
    let (x, flags) = rm32(x86, instr);
    *x = and(*x, y, flags);
    Ok(())
}

pub fn and_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8to32() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = and(*x, y, flags);
    Ok(())
}

pub fn and_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get32(instr.op1_register());
    let (x, flags) = rm32(x86, instr);
    *x = and(*x, y, flags);
    Ok(())
}

pub fn and_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    *x = and(*x, y, flags);
    Ok(())
}

pub fn and_rm16_imm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate16();
    let (x, flags) = rm16(x86, instr);
    *x = and(*x, y, flags);
    Ok(())
}

pub fn and_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = and(*x, y, flags);
    Ok(())
}

fn or<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x | y;
    // XXX More flags.
    flags.set(Flags::ZF, result.is_zero());
    result
}

pub fn or_rm32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    *x = or(*x, y, flags);
    Ok(())
}

pub fn or_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate32();
    let (x, flags) = rm32(x86, instr);
    *x = or(*x, y, flags);
    Ok(())
}

pub fn or_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8to32() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = or(*x, y, flags);
    Ok(())
}

pub fn or_rm16_imm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate16();
    let (x, flags) = rm16(x86, instr);
    *x = or(*x, y, flags);
    Ok(())
}

pub fn or_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = or(*x, y, flags);
    Ok(())
}

fn shl<I: Int + num_traits::WrappingShl>(x: I, y: u8, flags: &mut Flags) -> I {
    if y == 0 {
        return x;
    }
    // Carry is the highest bit that will be shifted out.
    let cf = (x.shr(I::bits() - y as usize) & I::one()).is_one();
    let val = x.wrapping_shl(y.as_usize() as u32);
    flags.set(Flags::CF, cf);
    let msb = val.shr(I::bits() - 1).is_one();
    flags.set(Flags::SF, msb);
    // OF undefined for shifts != 1, but this matches what Windows machine does, and also docs:
    // "For left shifts, the OF flag is set to 0 if the mostsignificant bit of the result is the
    // same as the CF flag (that is, the top two bits of the original operand were the same) [...]"
    flags.set(
        Flags::OF,
        x.shr(I::bits() - 1).is_one() ^ (x.shr(I::bits() - 2) & I::one()).is_one(),
    );
    flags.set(Flags::ZF, val.is_zero());

    val
}

pub fn shl_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm32(x86, instr);
    *x = shl(*x, y, flags);
    Ok(())
}

pub fn shl_rm32_cl(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.ecx as u8;
    let (x, flags) = rm32(x86, instr);
    *x = shl(*x, y, flags);
    Ok(())
}

pub fn shl_rm8_cl(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.ecx as u8;
    let (x, flags) = rm8(x86, instr);
    *x = shl(*x, y, flags);
    Ok(())
}

pub fn shl_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = shl(*x, y, flags);
    Ok(())
}

fn shr<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    if y == 0 {
        return x; // Don't affect flags.
    }
    flags.set(Flags::CF, ((x >> (y - 1) as usize) & I::one()).is_one());
    let val = x >> y as usize;
    flags.set(Flags::SF, false); // ?
    flags.set(Flags::ZF, val.is_zero());

    // Note: OF state undefined for shifts > 1 bit, but the following behavior
    // matches what my Windows box does in practice.
    flags.set(Flags::OF, (x >> (I::bits() - 1)).is_one());
    val
}

pub fn shr_rm32_cl(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.ecx as u8;
    let (x, flags) = rm32(x86, instr);
    *x = shr(*x, y, flags);
    Ok(())
}

pub fn shr_rm32_1(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm32(x86, instr);
    *x = shr(*x, 1, flags);
    Ok(())
}

pub fn shr_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm32(x86, instr);
    *x = shr(*x, y, flags);
    Ok(())
}

pub fn shr_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = shr(*x, y, flags);
    Ok(())
}

fn sar<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    if y.is_zero() {
        return x;
    }
    flags.set(Flags::CF, x.shr(y.as_usize() - 1).bitand(I::one()).is_one());
    flags.set(Flags::OF, false);
    // There's a random "u32" type in the num-traits signed_shr signature, so cast here.
    let result = x.signed_shr(y.as_usize() as u32);

    flags.set(Flags::SF, result.shr(I::bits() - 1).is_one());
    flags.set(Flags::ZF, result.is_zero());
    result
}

pub fn sar_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = sar(*x, y, flags);
    Ok(())
}

pub fn sar_rm32_cl(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.ecx as u8 as u32;
    let (x, flags) = rm32(x86, instr);
    *x = sar(*x, y, flags);
    Ok(())
}

pub fn sar_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8() as u8;
    let (x, flags) = rm8(x86, instr);
    *x = sar(*x, y, flags);
    Ok(())
}

fn ror(x: u32, y: u8, flags: &mut Flags) -> u32 {
    let result = x.rotate_right(y as u32);
    let msb = (result & 0x8000_0000) != 0;
    flags.set(Flags::CF, msb);
    flags.set(Flags::OF, msb ^ ((result & 04000_0000) != 0));
    result
}

pub fn ror_rm32_cl(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.ecx as u8;
    let (x, flags) = rm32(x86, instr);
    *x = ror(*x, y, flags);
    Ok(())
}

pub fn ror_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm32(x86, instr);
    *x = ror(*x, y, flags);
    Ok(())
}

fn xor<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x ^ y;
    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is undefined.
    flags.remove(Flags::OF);
    flags.remove(Flags::CF);
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, (result >> (I::bits() - 1)).is_one());
    result
}

pub fn xor_rm32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    *x = xor(*x, y, flags);
    Ok(())
}

pub fn xor_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate32();
    let (x, flags) = rm32(x86, instr);
    *x = xor(*x, y, flags);
    Ok(())
}

pub fn xor_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8to32() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = xor(*x, y, flags);
    Ok(())
}

pub fn xor_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = xor(*x, y, flags);
    Ok(())
}

pub fn xor_r8_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr);
    let (x, flags) = rm8(x86, instr);
    *x = xor(*x, y, flags);
    Ok(())
}

pub fn xor_rm8_r8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get8(instr.op1_register());
    let (x, flags) = rm8(x86, instr);
    *x = xor(*x, y, flags);
    Ok(())
}

fn add<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I, flags: &mut Flags) -> I {
    addc(x, y, I::zero(), flags)
}

fn addc<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I, z: I, flags: &mut Flags) -> I {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let y = y.wrapping_add(&z);
    let result = x.wrapping_add(&y);
    flags.set(Flags::CF, result < x || (y.is_zero() && !z.is_zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, (result >> (I::bits() - 1)).is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  0  1
    //   1  1  0
    let of = !(((x ^ !y) & (x ^ result)) >> (I::bits() - 1)).is_zero();
    flags.set(Flags::OF, of);
    result
}

pub fn add_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, &instr);
    let (x, flags) = rm32(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get32(instr.op1_register());
    let (x, flags) = rm32(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm32_r32_2(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get32(instr.op1_register());
    let (x, flags) = rm32(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate32();
    let (x, flags) = rm32(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8to32() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm16_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8to16() as u16;
    let (x, flags) = rm16(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_r16_rm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm16(x86, instr);
    let (x, flags) = rm16(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm8_r8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get8(instr.op1_register());
    let (x, flags) = rm8(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn add_r8_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr);
    let (x, flags) = rm8(x86, instr);
    *x = add(*x, y, flags);
    Ok(())
}

pub fn adc_rm8_r8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr);
    let carry = x86.flags.contains(Flags::CF);
    let (x, flags) = rm8(x86, instr);
    *x = addc(*x, y, carry as u8, flags);
    Ok(())
}

pub fn adc_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let carry = x86.flags.contains(Flags::CF);
    let (x, flags) = rm8(x86, instr);
    *x = addc(*x, y, carry as u8, flags);
    Ok(())
}

fn sbb<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    b: bool,
    flags: &mut Flags,
) -> I {
    let mut y = y;
    if b {
        y = y.wrapping_add(&I::one());
    }
    let (result, carry) = x.overflowing_sub(&y);
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    flags.set(Flags::CF, carry || (b && y == I::zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, (result >> (I::bits() - 1)).is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  1  1
    //   1  0  0
    let of = !(((x ^ y) & (x ^ result)) >> (I::bits() - 1)).is_zero();
    flags.set(Flags::OF, of);
    result
}

// pub(crate) for use in the cmp opcode impl.
pub(crate) fn sub<
    I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd,
>(
    x: I,
    y: I,
    flags: &mut Flags,
) -> I {
    sbb(x, y, false, flags)
}

pub fn sub_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8to32() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = sub(*x, y, flags);
    Ok(())
}

pub fn sub_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate32();
    let (x, flags) = rm32(x86, instr);
    *x = sub(*x, y, flags);
    Ok(())
}

pub fn sub_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = x86.regs.get32(instr.op1_register());
    let (x, flags) = rm32(x86, instr);
    *x = sub(*x, y, flags);
    Ok(())
}

pub fn sub_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    *x = sub(*x, y, flags);
    Ok(())
}

pub fn sub_r8_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = op1_rm8(x86, instr);
    let (x, flags) = rm8(x86, instr);
    *x = sub(*x, y, flags);
    Ok(())
}

pub fn sub_rm8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = sub(*x, y, flags);
    Ok(())
}

pub fn sbb_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let carry = x86.flags.contains(Flags::CF);
    let y = op1_rm32(x86, instr);
    let (x, flags) = rm32(x86, instr);
    *x = sbb(*x, y, carry, flags);
    Ok(())
}

pub fn sbb_rm32_r32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let carry = x86.flags.contains(Flags::CF);
    let y = x86.regs.get32(instr.op1_register());
    let (x, flags) = rm32(x86, instr);
    *x = sbb(*x, y, carry, flags);
    Ok(())
}

pub fn sbb_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let carry = x86.flags.contains(Flags::CF);
    let y = instr.immediate8to32() as u32;
    let (x, flags) = rm32(x86, instr);
    *x = sbb(*x, y, carry, flags);
    Ok(())
}

pub fn sbb_r8_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let carry = x86.flags.contains(Flags::CF);
    let y = op1_rm8(x86, instr);
    let (x, flags) = rm8(x86, instr);
    *x = sbb(*x, y, carry, flags);
    Ok(())
}

pub fn sbb_r8_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let carry = x86.flags.contains(Flags::CF);
    let y = instr.immediate8();
    let (x, flags) = rm8(x86, instr);
    *x = sbb(*x, y, carry, flags);
    Ok(())
}

pub fn imul_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = op0_rm32(x86, instr) as i32;
    let y = x86.regs.eax as i32;
    let res = (x as i64).wrapping_mul(y as i64) as u64;
    // TODO: flags.
    x86.regs.edx = (res >> 32) as u32;
    x86.regs.eax = res as u32;
    Ok(())
}

fn imul_trunc(x: i32, y: i32, _flags: &mut Flags) -> i32 {
    // TODO: flags.
    x.wrapping_mul(y)
}

pub fn imul_r32_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = x86.regs.get32(instr.op0_register()) as i32;
    let y = op1_rm32(x86, instr) as i32;
    let value = imul_trunc(x, y, &mut x86.flags);
    x86.regs.set32(instr.op0_register(), value as u32);
    Ok(())
}

pub fn imul_r32_rm32_imm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = op1_rm32(x86, instr) as i32;
    let y = instr.immediate32() as i32;
    let value = imul_trunc(x, y, &mut x86.flags);
    x86.regs.set32(instr.op0_register(), value as u32);
    Ok(())
}

pub fn imul_r32_rm32_imm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = op1_rm32(x86, instr) as i32;
    let y = instr.immediate8to32();
    let value = imul_trunc(x, y, &mut x86.flags);
    x86.regs.set32(instr.op0_register(), value as u32);
    Ok(())
}

pub fn idiv_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = (((x86.regs.edx as u64) << 32) | (x86.regs.eax as u64)) as i64;
    let y = op0_rm32(x86, instr) as i32 as i64;
    x86.regs.eax = (x / y) as i32 as u32;
    x86.regs.edx = (x % y) as i32 as u32;
    // TODO: flags.
    Ok(())
}

pub fn div_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let x = ((x86.regs.edx as u64) << 32) | (x86.regs.eax as u64);
    let y = op0_rm32(x86, instr) as u64;
    x86.regs.eax = (x / y) as u32;
    x86.regs.edx = (x % y) as u32;
    // TODO: flags.
    Ok(())
}

pub fn dec_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm32(x86, instr);
    *x = sub(*x, 1, flags);
    Ok(())
}

pub fn dec_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm8(x86, instr);
    *x = sub(*x, 1, flags);
    Ok(())
}

fn inc<I: Int + num_traits::WrappingAdd>(x: I, flags: &mut Flags) -> I {
    // Note this is not add(1) because CF should be preserved.
    let result = x.wrapping_add(&I::one());
    flags.set(Flags::OF, result.is_zero());
    flags.set(Flags::SF, (result >> (I::bits() - 1)).is_one());
    flags.set(Flags::ZF, result.is_zero());
    result
}

pub fn inc_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm32(x86, instr);
    *x = inc(*x, flags);
    Ok(())
}

pub fn inc_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm8(x86, instr);
    *x = inc(*x, flags);
    Ok(())
}

pub fn neg_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm32(x86, instr);
    flags.set(Flags::CF, *x != 0);
    // TODO: other flags registers.
    *x = -(*x as i32) as u32;
    Ok(())
}

pub fn neg_rm8(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, flags) = rm8(x86, instr);
    flags.set(Flags::CF, *x != 0);
    // TODO: other flags registers.
    *x = -(*x as i8) as u8;
    Ok(())
}

pub fn not_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let (x, _flags) = rm32(x86, instr);
    *x = !*x;
    Ok(())
}
