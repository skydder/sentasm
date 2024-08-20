<<<<<<< HEAD
main:
move 0 to eax
move 1 to ebx
loop:
=======
@.text
globalize _start

#_start
move 0 to eax
move 1 to ebx

#loop
>>>>>>> new
add ebx to eax
add 1 to ebx
compare ebx to 10
jump to loop if <=
<<<<<<< HEAD
return
=======
move rax to rdi
move 60 to rax
systemcall
(syscall
>>>>>>> new
