
display_ptr = .
a_0 = .
a_1 = .
b_0 = .
b_1 = .
c_0 = .
c_1 = .

	copy_v 1, b_0
	copy_v 0xc0, display_ptr
main:
	math_pp.add a_0, b_0, c_0
	math_pp.addc a_1, b_1, c_1
	copy_p b_0, a_0
	copy_p b_1, a_1
	copy_p c_0, b_0
	copy_p c_1, b_1
	deref_ps a_0, display_ptr
	math_pv.add display_ptr, 1, display_ptr
	deref_ps a_1, display_ptr
	math_pv.add display_ptr, 1, display_ptr
	jump_nz display_ptr, main

