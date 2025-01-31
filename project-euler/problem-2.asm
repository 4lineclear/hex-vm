    mov	    dx, 2
    push    1
    push    2
loop:
    pop	    cx
    pop	    bx
    mov	    ax, 0
    add	    ax, cx
    add	    ax, bx
    push    cx
    push    ax
    push    ax
    mod	    2
    cmp	    ax, 0
    pop	    ax
    je	    be
check:
    cmp	    ax, 4000000
    jl	    loop
    jmp	    100
be:
    add	    dx, ax
    jmp	    check
end:
