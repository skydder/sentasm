@.text
globalize _start

#_start
move 0 to rax
move 1 to rbx

#loop
add rbx to rax
add 1 to rbx
compare 10 with rbx
jump to loop if <=

(systemcall)
move rax to rdi
move 60 to rax
systemcall

