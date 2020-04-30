rule Def_i32
{
	meta:
	ir = "%# = alloca i32, align 4"
	location = "main"
	starts = "4"
	ends = "5"

	strings:
	$ = /def [a-zA-Z]: i32/

	condition:
	any of them
}

rule Mut_i32
{
	meta:
	ir = "store i32 #, i32* %#, align 4"
	location = "main"
	starts = "9:0"
	ends = "10:1"

	strings:
	$store = /[a-zA-Z]: i32 = [0-9]/

	condition:
	$store and Def_i32
}

rule Def_char
{
	meta:
	ir = "%# = alloca i8, align 1"
	location = "main"
	starts = "4"
	ends = "5"

	strings:
	$ = /def [a-zA-Z]: i8/

	condition:
	any of them
}

rule Mut_char
{
	meta:
	ir = "store i8 #, i8* %#, align 1"
	location = "main"
	starts = "8:0"
	ends = "9:1"

	strings:
	$store = /[a-zA-Z]: i8 = [0-9]/

	condition:
	$store and Def_char
}

rule Fn_define_puts
{
	meta:
	ir = "declare i32 @puts(i8* nocapture) nounwind"
	location = "header"
	start = ""
	end = ""

	strings:
	$ = "init puts"

	condition:
	any of them
}

rule Fn_puts
{
	meta:
	ir = "call i32 @puts(i8* %#)"
	location = "main"
	starts = "5"
	ends = "6"

	strings:
	$ = /puts [a-zA-Z]/

	condition:
	any of them
}
