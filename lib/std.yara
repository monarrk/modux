rule number
{
	meta:
	ir = "; test"

	strings:
	$num = /[0-9]/

	condition:
	any of them
}

rule boolean
{
	meta:
	ir = "; test"

	strings:
	$boolean = /(true|false)/

	condition:
	any of them
}

rule char
{
	meta:
	ir = "; test"

	strings:
	$ident = /'[a-zA-Z]'/

	condition:
	any of them
}
