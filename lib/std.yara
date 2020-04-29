rule def_i32
{
	meta:
	ir = "%1 = alloca i32, align 4\nstore i32 0, i32* %1, align 4"

	strings:
	$def = "def i32"

	condition:
	any of them
}
