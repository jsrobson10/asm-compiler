
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InstType {
	Stop = 0,
	CopyV = 1,
	CopyP = 2,
	StoreV = 3,
	StoreP = 4,
	Load = 5,
	Math = 6,
	MathS = 7,
	JumpV = 8,
	JumpP = 9,
	JumpIf = 10,
	JumpIfNot = 11,
	Nop = 15,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MathType {
	Zero = 0,
	And = 1,
	Or = 2,
	Nand = 3,
	Nor = 4,
	Xor = 5,
	Mul = 6,
	Equal = 8,
	Nequal = 9,
	Lthan = 10,
	Gthan = 11,
	Add = 12,
	Addc = 13,
	Sub = 14,
	Subc = 15,
}

impl InstType {
	pub fn is_math(self) -> bool {
		return match self {
			InstType::Math | InstType::MathS | InstType::JumpIf | InstType::JumpIfNot => true,
			_ => false,
		}
	}
	pub fn arg_count(self) -> u8 {
		return match self {
			InstType::Stop => 0,
			InstType::Math => 3,
			InstType::MathS => 3,
			InstType::CopyV => 2,
			InstType::CopyP => 2,
			InstType::StoreV => 2,
			InstType::StoreP => 2,
			InstType::Load => 2,
			InstType::JumpV => 1,
			InstType::JumpP => 1,
			InstType::JumpIf => 3,
			InstType::JumpIfNot => 3,
			InstType::Nop => 0,
		}
	}
}

pub fn get_named_math_type(mname: &[u8]) -> Option<MathType> {
	return match mname {
		b"zero" => Some(MathType::Zero),
		b"xor" => Some(MathType::Xor),
		b"and" => Some(MathType::And),
		b"or" => Some(MathType::Or),
		b"nand" => Some(MathType::Nand),
		b"nor" => Some(MathType::Nor),
		b"mul" => Some(MathType::Mul),
		b"gthan" => Some(MathType::Gthan),
		b"lthan" => Some(MathType::Lthan),
		b"equal" => Some(MathType::Equal),
		b"nequal" => Some(MathType::Nequal),
		b"add" => Some(MathType::Add),
		b"sub" => Some(MathType::Sub),
		b"addc" => Some(MathType::Addc),
		b"subc" => Some(MathType::Subc),
		_ => None,
	}
}

pub fn get_named_inst_type(iname: &[u8]) -> Option<InstType> {
	return match iname {
		b"stop" => Some(InstType::Stop),
		b"copy_v" => Some(InstType::CopyV),
		b"copy_p" => Some(InstType::CopyP),
		b"store_v" => Some(InstType::StoreV),
		b"store_p" => Some(InstType::StoreP),
		b"load" => Some(InstType::Load),
		b"math" => Some(InstType::Math),
		b"math_s" => Some(InstType::MathS),
		b"jump_v" => Some(InstType::JumpV),
		b"jump_p" => Some(InstType::JumpP),
		b"jump_if" => Some(InstType::JumpIf),
		b"jump_if_not" => Some(InstType::JumpIfNot),
		b"nop" => Some(InstType::Nop),
		_ => None,
	}
}

