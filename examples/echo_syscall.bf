#define QR >>>>>>>> // move pointer by a QWORD
#define QA ++++++++ // add QWORD size


//Read syscall
'        //rax 0
QR
'        //rdi 0
QR
'$ QA QA //rsi pionter plus 16
QR
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++ //rdx 64
&?

//Write syscall
'+        //rax 1 
QR
'+        //rdi 1
&?