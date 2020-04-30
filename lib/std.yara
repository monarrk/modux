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

rule Def_str
{
	meta:
	ir = "@.# = private unnamed_addr constant [3 x i8] c\"#\", align 1"
	location = "header"
	start = "7:17"
	ends = "8:20"

	strings:
	$ = /global [a-zA-Z]: str = "[a-zA-Z0-9]{3}"/

	condition:
	any of them
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

rule Fn_puts_char
{
	meta:
	ir = "call i32 @puts(i8* %#)"
	location = "main"
	starts = "10"
	ends = "11"

	strings:
	$ = /puts_char [a-zA-Z]/

	condition:
	any of them
}

rule Fn_puts_str
{
	meta:
	ir = "%tmp = alloca i8*, align 8\n\tstore i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.#, i64 0, i64 0), i8** %tmp, align 8\n\t%tmp2 = load i8*, i8** %tmp, align 8\n\tcall i32 @puts(i8* %tmp2)"
	location = "main"
	starts = "5"
	ends = "6"

	strings:
	$ = /puts [a-zA-Z]/

	condition:
	any of them
}
