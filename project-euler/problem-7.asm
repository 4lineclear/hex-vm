start:
    mov cx, 2 ; cx = i = possible prime
    mov dx, 1 ; dx = prime count
main_loop:
    push cx
    push dx
    call check
    pop dx
    pop cx
    cmp ax, 0
    jne main_inc
    cmp dx, 10001
    jge end
    inc dx
main_inc:
    inc cx
    jmp main_loop
check:
    mov dx, cx
    mov cx, 2 ; cx = j = divisor
check_loop:
    mov ax, cx
    mul ax
    cmp ax, dx
    jg  check_success
    mov ax, dx
    mod cx
    cmp ax, 0
    je  check_fail
    inc cx
    jmp check_loop
check_success:
    mov ax, 0
    ret
check_fail:
    mov ax, 1
    ret
end:
