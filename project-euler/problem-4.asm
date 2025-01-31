start:
    mov     bx, 1000
outer:
    mov     cx, bx
    dec     bx
    cmp     bx, 0
    je      end
inner:
    dec     cx
    cmp     cx, 0
    je      outer
do_check:
    mov     ax, cx
    mul     bx
    mov     si, ax	; si = num
    push    bx
    push    cx
    mov     bx, 0	; bx = rev
check_loop:
    mov     cx, ax 	; cx = num
    mod     10	  	; ax = dig
    mov     dx, ax	; dx = dig
    mov     ax, bx	; ax = rev
    mul     10		; ax = rev * 10
    add     ax, dx	; ax = rev * 10 + dig
    mov     bx, ax	; bx = rev * 10 + dig
    mov     ax, si	; ax = num
    cmp	    ax, bx	; ax == bx
    je      end		; end if ax == bx
    mov     ax, cx	; ax = curr
    div     10		; ax = ax / 10 
    cmp     ax, 0	; ax == 0
    jg      check_loop	; loop if ax > 0
check_end:
    pop     cx
    pop     bx
    jmp     inner
end:
