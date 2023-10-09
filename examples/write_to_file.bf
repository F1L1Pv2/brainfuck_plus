#tape msg byte 7
#tape syscall qword 16
#tape file_desc qword 1
#tape file_path byte 8

@{msg} "Wazzup\n"
@{file_path} "./bfile\0"

@{syscall} 2 > {file_path} > 577 > 420 &?
@{file_desc} _ ^ ^
@{syscall} 1 > _ > {msg} > 7 &?
@{syscall} 3 > _ &?

@{syscall} 60 > 0 &?