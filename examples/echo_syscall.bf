#define QR >>>>>>>> // move pointer by a QWORD
#define QA ++++++++ // add QWORD size


//Read syscall
`0`        //rax 0
QR
`0`        //rdi 0
QR
'$ QA QA //rsi pionter plus 16
QR
`64` //rdx 64
&?

//Write syscall
`1`        //rax 1 
QR
`1`        //rdi 1
&?