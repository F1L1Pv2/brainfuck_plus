#tape msg byte 14
#tape syscall qword 4

@{msg} "Hello, World!\n"

@{syscall}

1
>
1
>
{msg}
>
14

&?!
