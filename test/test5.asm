@.data
define hello_world as ["Hello world!", 10] by 8bit

@.text
globalize _start

#_start
(sys_write)
move 1 to rax
move 1 to rdi
move hello_world to rsi
move 13 to rdx
systemcall

(sys_exit)
move 60 to rax
move 0 to rdi
systemcall