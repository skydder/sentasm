@.data
	globalize GL.L.3
	define GL.L.3 as [110,117,109,10,0] by 8bit
@.data
	globalize GL.L.2
	define GL.L.2 as [102,105,122,122,10,0] by 8bit
@.data
	globalize GL.L.1
	define GL.L.1 as [98,117,122,122,10,0] by 8bit
@.data
	globalize GL.L.0
	define GL.L.0 as [102,105,122,122,98,117,122,122,10,0] by 8bit
@.text
	globalize main
#main
	push rbp
	move rsp to rbp
	substract 16 from rsp
	load_effective_address *(rbp-16) to rax
	push rax
	move 1 to rax
	pop rdi
	move eax to *(rdi)
#.L.begin.1
	move 40 to rax
	push rax
	load_effective_address *(rbp-16) to rax
	move *(rax by 32bit) to rax with sign_extention
	pop rdi
	compare edi with eax
	set_byte to al if <
	move al to rax with sign_extention
	compare 0 with rax
	jump to .L.end.1 if ==
	load_effective_address *(rbp-12) to rax
	push rax
	move 0 to rax
	push rax
	move 3 to rax
	push rax
	load_effective_address *(rbp-16) to rax
	move *(rax by 32bit) to rax with sign_extention
	pop rdi
	extend_acc_reg by 32bit
	divide by edi as signed
	move edx to eax
	pop rdi
	compare edi with eax
	set_byte to al if ==
	move al to rax with sign_extention
	pop rdi
	move eax to *(rdi)
	load_effective_address *(rbp-8) to rax
	push rax
	move 0 to rax
	push rax
	move 5 to rax
	push rax
	load_effective_address *(rbp-16) to rax
	move *(rax by 32bit) to rax with sign_extention
	pop rdi
	extend_acc_reg by 32bit
	divide by edi as signed
	move edx to eax
	pop rdi
	compare edi with eax
	set_byte to al if ==
	move al to rax with sign_extention
	pop rdi
	move eax to *(rdi)
	load_effective_address *(rbp-4) to rax
	push rax
	load_effective_address *(rbp-8) to rax
	move *(rax by 32bit) to rax with sign_extention
	push rax
	load_effective_address *(rbp-12) to rax
	move *(rax by 32bit) to rax with sign_extention
	pop rdi
	and eax with edi
	pop rdi
	move eax to *(rdi)
	load_effective_address *(rbp-4) to rax
	move *(rax by 32bit) to rax with sign_extention
	compare 0 with rax
	jump to .L.else.2 if ==
	move GL.L.0 to rax
	push rax
	pop rdi
	move 0 to rax
	call printf
	jump to .L.end.2
#.L.else.2
	load_effective_address *(rbp-8) to rax
	move *(rax by 32bit) to rax with sign_extention
	compare 0 with rax
	jump to .L.else.3 if ==
	move GL.L.1 to rax
	push rax
	pop rdi
	move 0 to rax
	call printf
	jump to .L.end.3
#.L.else.3
	load_effective_address *(rbp-12) to rax
	move *(rax by 32bit) to rax with sign_extention
	compare 0 with rax
	jump to .L.else.4 if ==
	move GL.L.2 to rax
	push rax
	pop rdi
	move 0 to rax
	call printf
	jump to .L.end.4
#.L.else.4
	move GL.L.3 to rax
	push rax
	pop rdi
	move 0 to rax
	call printf
#.L.end.4
#.L.end.3
#.L.end.2
	load_effective_address *(rbp-16) to rax
	push rax
	move 1 to rax
	push rax
	load_effective_address *(rbp-16) to rax
	move *(rax by 32bit) to rax with sign_extention
	pop rdi
	add edi to eax
	pop rdi
	move eax to *(rdi)
	jump to .L.begin.1
#.L.end.1
#.L.return.main
	move rbp to rsp
	pop rbp
	return
