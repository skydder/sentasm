section .data
	global GL.L.3
	GL.L.3 db 110, 117, 109, 10, 0
section .data
	global GL.L.2
	GL.L.2 db 102, 105, 122, 122, 10, 0
section .data
	global GL.L.1
	GL.L.1 db 98, 117, 122, 122, 10, 0
section .data
	global GL.L.0
	GL.L.0 db 102, 105, 122, 122, 98, 117, 122, 122, 10, 0
section .text
	global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	lea rax, [rbp-16]
	push rax
	mov rax, 1
	pop rdi
	mov [rdi], eax
.L.begin.1:
	mov rax, 40
	push rax
	lea rax, [rbp-16]
	movsxd rax, dword[rax]
	pop rdi
	cmp eax, edi
	setl al
	movsx rax, al
	cmp rax, 0
	je .L.end.1
	lea rax, [rbp-12]
	push rax
	mov rax, 0
	push rax
	mov rax, 3
	push rax
	lea rax, [rbp-16]
	movsxd rax, dword[rax]
	pop rdi
	cdq 
	idiv edi
	mov eax, edx
	pop rdi
	cmp eax, edi
	sete al
	movsx rax, al
	pop rdi
	mov [rdi], eax
	lea rax, [rbp-8]
	push rax
	mov rax, 0
	push rax
	mov rax, 5
	push rax
	lea rax, [rbp-16]
	movsxd rax, dword[rax]
	pop rdi
	cdq 
	idiv edi
	mov eax, edx
	pop rdi
	cmp eax, edi
	sete al
	movsx rax, al
	pop rdi
	mov [rdi], eax
	lea rax, [rbp-4]
	push rax
	lea rax, [rbp-8]
	movsxd rax, dword[rax]
	push rax
	lea rax, [rbp-12]
	movsxd rax, dword[rax]
	pop rdi
	and eax, edi
	pop rdi
	mov [rdi], eax
	lea rax, [rbp-4]
	movsxd rax, dword[rax]
	cmp rax, 0
	je .L.else.2
	mov rax, GL.L.0
	push rax
	pop rdi
	mov rax, 0
	call printf
	jmp .L.end.2
.L.else.2:
	lea rax, [rbp-8]
	movsxd rax, dword[rax]
	cmp rax, 0
	je .L.else.3
	mov rax, GL.L.1
	push rax
	pop rdi
	mov rax, 0
	call printf
	jmp .L.end.3
.L.else.3:
	lea rax, [rbp-12]
	movsxd rax, dword[rax]
	cmp rax, 0
	je .L.else.4
	mov rax, GL.L.2
	push rax
	pop rdi
	mov rax, 0
	call printf
	jmp .L.end.4
.L.else.4:
	mov rax, GL.L.3
	push rax
	pop rdi
	mov rax, 0
	call printf
.L.end.4:
.L.end.3:
.L.end.2:
	lea rax, [rbp-16]
	push rax
	mov rax, 1
	push rax
	lea rax, [rbp-16]
	movsxd rax, dword[rax]
	pop rdi
	add eax, edi
	pop rdi
	mov [rdi], eax
	jmp .L.begin.1
.L.end.1:
.L.return.main:
	mov rsp, rbp
	pop rbp
	ret 

extern printf