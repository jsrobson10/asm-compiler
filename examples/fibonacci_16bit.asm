
counter = 0xf0
counter_inc = 0xf1
counter_mask = 0xf2
counter_end = 0xf3
display_start0 = 0xf4
display_start1 = 0xf5
display_ptr = .
a0 = 0xf6
a1 = 0xf7
b0 = 0xf8
b1 = 0xf9
c0 = 0xfa
c1 = 0xfb

	copy_v 0xc1, display_start0
	copy_v 0xc0, display_start1
	copy_v 0, a0
	copy_v 0, a1
	copy_v 1, b0
	copy_v 0, b1
	copy_v 0, counter
	copy_v 2, counter_inc
	copy_v 31, counter_mask
	copy_v 64, counter_end
scan:
	math.add a0, b0, c0
	math.addc a1, b1, c1
	copy_p b0, a0
	copy_p b1, a1
	copy_p c0, b0
	copy_p c1, b1
	math.and counter, counter_mask, 0xff
	math.add 0xff, display_start0, display_ptr
	store_p a0, display_ptr
	math.add 0xff, display_start1, display_ptr
	store_p a1, display_ptr
	math.add counter, counter_inc, counter
	jump_if.lthan counter, counter_end, scan

