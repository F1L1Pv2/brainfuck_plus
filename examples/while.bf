#define add10 +++++ +++++
#define toascii add10 add10 add10 add10 ++++ ++++

`10` > `" \n"` &
[
    ^ > // copy current index
    _ - toascii . // sub 1 from it and then convert to ascii digit and write it 
    > . // write new line 
    &- // comeback to iterator and decrease it
]