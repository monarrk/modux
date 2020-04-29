rule Def_i32
{
	meta:
	ir = "%{} = alloca i32, align 4"
	start = 8
	end = 9

	strings:
	$ = /def i32 [a-zA-Z]/

	condition:
	any of them
}

// TODO
rule Mut_i32
{
	meta:
	ir = "store i32 {}, i32* %{}, align 4"
	start = 8
	end = 9

	strings:
	$store = /mut i32 [a-zA-Z] [0-9]/

	condition:
	$store and Def_i32
}
