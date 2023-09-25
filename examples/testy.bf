#define baller
#ifdef baller
  #define lol `"baller is defined\n"`
#else
  #define lol `"baller isnt defined\n"`
#endif

#define QR >>>>>>>> // move pointer by a QWORD
#define QA ++++++++ // add QWORD size

QR QR QR QR

lol

& //go back to addr 0

`1` //putting write syscall id 1 into rax
QR
`1` //putting stdout id 1 into rdi
QR
'$ QA QA //putting msg addr into rsi
QR
`64`  //putting msg len into rdx

&? //performing syscall