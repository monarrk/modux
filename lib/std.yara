rule Def_i32
{
	meta:
	ir = "%# = alloca i32, align 4"
	args = 1
	starts = "8"
	ends = "9"

	strings:
	$ = /def i32 [a-zA-Z]/

	condition:
	any of them
}

rule Mut_i32
{
	meta:
	ir = "store i32 #, i32* %#, align 4"
	args = 2
	starts = "9:0"
	ends = "10:1"

	strings:
	$store = /[a-z]: i32 = [0-9]/

	condition:
	$store and Def_i32
}
