BITS 64

section .data
	msg db "Hello from syscall"
	len equ $ - msg

section .text
    global _start

_start:
    nop
    nop
    nop
    nop
    nop
    nop
    nop
    nop
    nop
    mov     rax, 1 
    mov	    rdi, 1
    lea     rsi, [rel msg]
    mov     rdx, len
    syscall

    mov     rax, 60
    xor     rdi, rdi
    syscall
