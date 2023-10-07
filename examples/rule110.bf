#define buff_size `50`
#define to_ascii ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++

#tape array byte buff_size
#tape new_array byte buff_size
#tape temp_arr byte `3`

#tape write_buff byte `1024`
#tape iterator byte `1`

#tape syscall qword `4`
#tape temp byte `1`

#tape rule byte `1`
@{rule} `110`


// putting 1 at the end of buff_size
@{array} &
@{iterator} buff_size -
[

    @{array} >
    @{iterator} -
]
@{array} `1` &
// ---------------------------------

// put new line at the end of buff
@{write_buff} &
@{iterator} buff_size
[

    @{write_buff} >
    @{iterator} -
]
@{write_buff} `10` &

// ---------------------------------

@{syscall} `1` > `1` > {write_buff} > buff_size + & // setup syscall for future usage


@{iterator} buff_size
[

    @{iterator} ^

    // copying to write_buff and converting into ascii
    @{array} &
    @{new_array} &
    @{write_buff} &

    @{iterator} buff_size
    [

        @{array} ^ >
        @{write_buff} _ to_ascii >

        @{iterator} -
    ]

    @{syscall} ?!

    @{iterator} buff_size

    [
        // move to current_cell
        @{array} & 
        @{new_array} & 
        @{iterator} ^
        [
            @{array} <
            @{new_array} <
            @{iterator} -
        ]

        // ------------------------------------------------

        // combine 3 cells

        @{temp} `0`
        @{array} < ^ >  ^  > ^ < 
        @{temp_arr} _  > _ ; > _ ;; & ^ > ^ > ^ &
        @{temp} ||| ^

        // ------------------------------------------------

        // get output from rule

        @{iterator} _ //temp is iterator
        @{rule} ^

        @{iterator}
        [

            @{rule} : 

            @{iterator} -
        ]

        `1` ^ @{rule} \ ^
    
        @{new_array} _

        @{rule} _

        // ------------------------------------------------

        @{iterator} _ - //updating iterator
    ]

    @{array} &
    @{new_array} &

    @{iterator} _ ^ buff_size -
    [
        @{new_array} ^
        @{array} _

        @{new_array}>
        @{array}>

        @{iterator} -
    ]

    @{iterator} _ - //updating iterator
]