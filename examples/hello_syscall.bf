>>>>>>>> >>>>>>>> >>>>>>>> >>>>>>>>

'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++
> //H
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +
> //e
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++
> //l
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++
> //l
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +
> //o
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++
> //coma
'++++++++++ ++++++++++ ++++++++++ ++
> //space
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +++++++
> //W
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +
> //o
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++
> //r
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++
> //l
'++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++
> //d
'++++++++++ ++++++++++ ++++++++++ +++
> //explanation mark
'++++++++++ //new line

& //go back to addr 0

'+ //putting write syscall id 1 into rax
>>>>>>>>
'+ //putting stdout id 1 into rdi
>>>>>>>>
'$ ++++++++ ++++++++ //putting msg addr into rsi
>>>>>>>>
'+++++ +++++ ++++ //putting msg len into rdx

&

? //performing syscall