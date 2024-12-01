
i = 0xf0
m = 0xf1
one = 0xf2
two = 0xf3
display_ptr = 0xf3
display_add = 0xf4
prime_at = 0xf5


	copy_v 1, one
	copy_v 7, m
	copy_v 2, display_add
	copy_v 0xc1, display_ptr
loop:
	math.add i, one, i
	math.mul i, m, 0xff
	store_p 0xff, display_ptr
	math.add display_ptr, display_add, display_ptr
	jump_v loop

