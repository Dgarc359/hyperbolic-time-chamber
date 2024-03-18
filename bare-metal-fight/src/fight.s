.section .text.entrypoint
.global _entrypoint
_entrypoint:
    la a0, welcome_string
    call putstring
    call sbi_console_getchar
    blt a0, x0, _entrypoint
    mv s2, a0
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    mv a0, s2
    call sbi_console_putchar
    call sbi_shutdown
.data
welcome_string:
.string "Welcome to Fight!"

.text

# doesn't start with ".L", is not an internal (local) label
putstring:
    # fucking intel notation!
    lb t3, 0(a0)
    beq t3, x0, .Lreturn
    call sbi_console_putchar
    mv
    # same as:
    #jr ra
# ".L" = internal label
.Lreturn:
    ret

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
