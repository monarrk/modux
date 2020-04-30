; Header
target triple = "x86_64-unknown-freebsd"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

	@.s = private unnamed_addr constant [3 x i8] c"abc", align 1
	declare i32 @puts(i8* nocapture) nounwind

; Main
define dso_local i32 @main() #0 {
	%a = alloca i32, align 4
	store i32 1, i32* %a, align 4
	%b = alloca i8, align 1
	store i8 7, i8* %b, align 1
	call i32 @puts(i8* %b)
	%tmp = alloca i8*, align 8
	store i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.s, i64 0, i64 0), i8** %tmp, align 8
	%tmp2 = load i8*, i8** %tmp, align 8
	call i32 @puts(i8* %tmp2)

	ret i32 0
}