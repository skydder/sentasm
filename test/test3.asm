@.text
globalize _start

#_start
move 0 to rax
move 1 to rbx

#loop
add rbx to rax
add 1 to rbx
compare rbx to 10
jump to loop if <=
move rax to rdi
move 60 to rax
systemcall
(syscall)
