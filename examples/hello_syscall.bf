#define QR 8`> // move pointer by a QWORD
#define QA 8`+ // add QWORD size

QR QR QR QR

"Hello, World!\n"

& //go back to addr 0

1 //putting write syscall id 1 into rax
QR
1 //putting stdout id 1 into rdi
QR
'$ QA QA //putting msg addr into rsi
QR
14  //putting msg len into rdx

&?! //performing syscall