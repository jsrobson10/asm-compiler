
#[derive(Copy, Clone)]
pub enum InstType {
	Stop    = 0,
	MathPP  = 1,
	MathPV  = 2,
	MathVP  = 3,
	Rand    = 4,
	CopyV   = 5,
	CopyP   = 6,
	DerefL  = 7,
	DerefVS = 8,
	DerefPS = 9,
	JumpV   = 10,
	JumpP   = 11,
	JumpVS  = 12,
	JumpPS  = 13,
	JumpIZ  = 14,
	JumpNZ  = 15,
}

#[derive(Copy, Clone)]
pub enum MathType {
	Zero   = 0,
	Xor    = 1,
	And    = 2,
	Or     = 3,
	Nand   = 4,
	Nor    = 5,
	Gthan  = 8,
	Lthan  = 9,
	Equal  = 10,
	Nequal = 11,
	Add    = 12,
	Sub    = 13,
	Addc   = 14,
	Subc   = 15,
}

pub fn get_named_math_type(mname: &[u8]) -> Option<MathType> {
	return match mname {
		b"zero"   => Some(MathType::Zero),
		b"xor"    => Some(MathType::Xor),
		b"and"    => Some(MathType::And),
		b"or"     => Some(MathType::Or),
		b"nand"   => Some(MathType::Nand),
		b"nor"    => Some(MathType::Nor),
		b"gthan"  => Some(MathType::Gthan),
		b"lthan"  => Some(MathType::Lthan),
		b"equal"  => Some(MathType::Equal),
		b"nequal" => Some(MathType::Nequal),
		b"add"    => Some(MathType::Add),
		b"sub"    => Some(MathType::Sub),
		b"addc"   => Some(MathType::Addc),
		b"subc"   => Some(MathType::Subc),
		_ => None,
	}
}

pub fn get_named_inst_type(iname: &[u8]) -> Option<InstType> {
	return match iname {
		b"stop"     => Some(InstType::Stop),
		b"math_pp"  => Some(InstType::MathPP),
		b"math_pv"  => Some(InstType::MathPV),
		b"math_vp"  => Some(InstType::MathVP),
		b"rand"     => Some(InstType::Rand),
		b"copy_v"   => Some(InstType::CopyV),
		b"copy_p"   => Some(InstType::CopyP),
		b"deref_l"  => Some(InstType::DerefL),
		b"deref_vs" => Some(InstType::DerefVS),
		b"deref_ps" => Some(InstType::DerefPS),
		b"jump_v"   => Some(InstType::JumpV),
		b"jump_p"   => Some(InstType::JumpP),
		b"jump_vs"  => Some(InstType::JumpVS),
		b"jump_ps"  => Some(InstType::JumpPS),
		b"jump_iz"  => Some(InstType::JumpIZ),
		b"jump_nz"  => Some(InstType::JumpNZ),
		_ => None,
	}
}

pub fn get_arg_count(iname: InstType) -> u8 {
	return match iname {
		InstType::Stop    => 1,
		InstType::MathPP  => 3,
		InstType::MathPV  => 3,
		InstType::MathVP  => 3,
		InstType::Rand    => 1,
		InstType::CopyV   => 2,
		InstType::CopyP   => 2,
		InstType::DerefL  => 2,
		InstType::DerefVS => 2,
		InstType::DerefPS => 2,
		InstType::JumpV   => 1,
		InstType::JumpP   => 1,
		InstType::JumpVS  => 2,
		InstType::JumpPS  => 2,
		InstType::JumpIZ  => 2,
		InstType::JumpNZ  => 2,
	}
}

