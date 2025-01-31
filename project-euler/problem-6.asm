start:
    call square_sum
    push ax
    call sum_square
    mul ax
    pop dx
    sub ax, dx
    jmp end
; square then sum
square_sum:
    mov cx, 1
    mov dx, 0
sqsu_loop:
    mov ax, cx
    mul ax
    add dx, ax
    inc cx
    cmp cx, 100
    jle sqsu_loop
    mov ax, dx
    ret
; sum then square
sum_square:
    mov cx, 1
    mov dx, 0
susq_loop:
    add dx, cx
    inc cx
    cmp cx, 100
    jle susq_loop
    mov ax, dx
    ret
end:
