.section .text.entrypoint
.global _entrypoint
_entrypoint:
    la sp, _initial_stack_pointer
    la a0, welcome_string
    call putstring
    li a0, 123
    call putint
    li a0, '\n'
    call sbi_console_putchar
    li a0, 90218
    call putint
    li a0, '\n'
    call sbi_console_putchar
    li a0, 1800345899
    call putint
    li a0, '\n'
    call sbi_console_putchar
.Lgetchar_loop:
    call sbi_console_getchar
    blt a0, x0, .Lgetchar_loop
    mv s2, a0
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    # mv a0, s2
    # call sbi_console_putchar
    call sbi_shutdown
.data
welcome_string:
.string "Welcome to Fight!"

.text

# sp and s1 and s2
# V
# ??????????s0s0s1s1s2s2rara

# VARIABLES!
# stack reg var why
# 4     ra: we need to save this too
# 4     s0: n: the input number
# 4     s1: array_of_digits: the beginning of the digits
# 4     s2: next_digit_to_output: a pointer into the array of digits
# -     a0: ch: a character to output
# -     t0: temp char code to output
# 10        the array
# 4 + 4 + 4 + 4 + 10 = 26



















putint:
    # Reserve 26 bytes of stack
    addi sp, sp, -26
    sw s0, 10(sp)
    sw s1, 14(sp)
    sw s2, 18(sp)
    sw ra, 22(sp)
    # now we can do the actual function
    mv s0, a0 # s0: n: the input number
    mv s1, sp # s1: array_of_digits
    mv s2, sp # s2: next_digit_to_output
    li t1, 10 # t1 gets to be 10. It's ten now.
.Lgetcharcode_loop:
    # let temp = character_code_of('0') + (n % 10);
    rem t0, s0, t1
    addi t0, t0, '0'
    # next_digit_to_output[0] = temp;
    sb t0, 0(s2)
    # next_digit_to_output += 1;
    addi s2, s2, 1
    # n = n / 10;
    div s0, s0, t1
    # ... while(n > 0)
    bgt s0, x0, .Lgetcharcode_loop
    # oh! we're doing a(nother) loop! -SB
.Lputintsupersickdothingsloop:
    # next_digit_to_output -= 1;
    addi s2, s2, -1
    # ch = next_digit_to_output[0];
    lb a0, 0(s2)
    # sbi_console_putchar(ch);
    call sbi_console_putchar
    # while (next_digit_to_output > array_of_digits)
    bgt s2, s1, .Lputintsupersickdothingsloop
    # all done with the actual function
    lw s0, 10(sp)
    lw s1, 14(sp)
    lw s2, 18(sp)
    lw ra, 22(sp)
    addi sp, sp, 26
    ret
    

putstring:
    # Reserve 8 bytes on the stack.
    addi sp, sp, -8
    # byte 0-3 on the stack: 
    sw s0, 0(sp)
    # byte 4-7 on the stack: our return address being saved for later
    sw ra, 4(sp)
    # put a0 (first argument) into s0 (a register that things we call are not
    # allowed to fuck up)
    mv s0, a0
    # now that it's in s0, we can call `sbi_console_putchar` without it being
    # "lost"
.Lputstr_loop:
    # let ch = read_byte(pointer);
    lb a0, 0(s0)
    # if(ch == 0) break;
    beq a0, x0, .Lputstr_end_of_loop
    # sbi_console_putchar(ch);
    #mv a0, a0 # set up its first argument
    call sbi_console_putchar
    # pointer += 1;
    addi s0, s0, 1
    # (and the loop comes back around)
    j .Lputstr_loop
.Lputstr_end_of_loop:
    # restore s0 and the return address
    lw s0, 0(sp)
    lw ra, 4(sp)
    # restore the stack pointer
    addi sp, sp, 8
    # return!
    ret

# function putstring(pointer) {
#     do {
#         let ch = read_byte(pointer);
#         if(ch == 0) break;
#         sbi_console_putchar(ch);
#         pointer += 1;
#     } while(true);
# }














# doesn't start with ".L", is not an internal (local) label
# putstring:
#     mv t5, a0
#     mv t4, ra # save return address
# .Lputstr_loop:
#     # fucking intel notation!
#     lb t3, 0(t5)
#     beq t3, x0, .Lreturn
#     mv a0, t3
#     call sbi_console_putchar
#     addi t5, t5, 1
#     j .Lputstr_loop
#     # call THING is the same as:
#     #jal ra, THING
#     # ret is the same as:
#     #jr ra
# # ".L" = internal label
# .Lreturn:
#     mv ra, t4 # restore return address
    # ret

sbi_shutdown:
    # When making a Supervisor Binary Interface call...
    # a6: Function ID (always 0 for our purposes)
    li a6, 0
    # a7: Extension ID (8 = "shut the fuck down")
    li a7, 8
    # Call the Supervisor Binary Interface and--
    ecall

sbi_console_putchar:
    # a6: Function ID (always 0 for our purposes)
    li a6, 0
    # a7: Extension ID (1 = "put the fuck char")
    li a7, 1
    # Call the Supervisor Binary Interface
    ecall
    # and return!
    ret

sbi_console_getchar:
    # a6: Function ID (always 0 for our purposes)
    li a6, 0
    # a7: Extension ID (2 = "get the fuck char")
    li a7, 2
    # Call the Supervisor Binary Interface
    ecall
    # and return!
    ret
