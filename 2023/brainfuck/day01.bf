Part 1 | Calculate the answer
1) Go through the input line by line
2) Add the first digit*10 to a 2byte sum
3) Always record the last digit seen
4) At the end of a line add the last digit to the sum
5) Input is done when a 0 is read

000 = The `sum` high byte
001 = The `sum` low bye
002 = `last_digit`
003 = `first_digit_unencountered` (starts true)
004 = `more_input` flag (starts true)

>>>+>+
[
  >,
  Check for null (end of input)
  [->+>+<<]>>[-<<+>>]+>+<<[[-]>-<]>[->-<<<<->>>]>
  [-<<<
    Check for the line break delimiter between lines
    [->+>+<<]>>>
    compare character to newline
    ++++++++++ [-<->]+<[>-<[-]]+>
    if compare == 0
    [-<->
      add second digit
      <<[-]
      <<<<[
        -<+[->>>>+>+<<<<<]>>>>>[-<<<<<+>>>>>]
        +<[>-<[-]]>[-<<<<<<+>>>>>>]
        <<<<
      ]
      >+>>>>>
    ]<
    if compare != 0
    [-
      Now check if the character is in the number ascii range
      ++++++[-<-------->]< minus 48
      [->+>+<<]>>[-<<+>>]  copy before comparison
      ++++++++++           `compare_against` number (a digit will now be less than 10)
      >>>+                 `is_digit` flag (starts true)
      <<<<
      [-
        >-[->+>+<<]>>[-<<+>>] substract from the 'lt marker' and copy it
        +<[>-<[-]]>
        'lt marker' is 0 before the number
        [ ->-<<<<[-]>>> ]
        <<<
      ]
      >[-] wipe 'lt marker'
      if is_digit
      >>>[-
        <<<<< <<<<[-]>>>> [-<+<<<+>>>>] set `last_digit`
        <<< [-
          >>[->++++++++++<]> multiply first digit by 10
          [
            -<<<<<+[->>>>>>+>+<<<<<<<]>>>>>>>[-<<<<<<<+>>>>>>>]
            +<[[-]>-<]>[-<<<<<<<<+>>>>>>>>]<<
          ]
          <<<
        ]>>>>>>>>
      ]
    <<<<
    ]
    <[-] cleanup up remaining nondigit character
    >>
  ]<<<<
]
<-<<< cleanup while moving back to 000

The 2byte sum is now in 000 and 001

Part 2 | Print the 2byte sum
1) Repeatedly subtract 10 from the number while counting the quotient
2) When there is no number left record the remainder (the next digit)
3) Offset the tape to make room for another digit
3) Repeat until the quoteint is 0 start walking back and printing the digits

000 = The constant `more_digits` = 0 marking the end of the digits array
N 1 = The nth digit
N 2 = `more_digits` = 1

[->>>>>>>>+<<<<<<<<]>[->>>>>>>>+<<<<<<<<]
>>>+
[
  >+
  [
    >+>++++++++++< the divisor and loop flag
    [>-
      compare the low byte to 0
      >>[->+>+<<]>>[-<<+>>]+>+<<[[-]>-<]>[->-<
        compare the high byte to 0
        <<<[->+>+<<]>>[-<<+>>]+>+<<[[-]>-<]>[->-<
          <<<<<<<<<+++++++++>>>>>>[-<<<<<<->>>>>>] store the remainder
          <-<-<->>>>>>
        ]>[-
          <<-<->>>
        ]
      ]>[-
        <<<->>>
      ]

      <<<<<<<<[->>>>>>+>+<<<<<<<]>>>>>>>[-<<<<<<<+>>>>>>>]<
      [-
        compare the remaining divisor to 0
        <<< [->>>+>+<<<<]>>>>[-<<<<+>>>>]+>+><<<[[-]>-<]>[->-<
          Add 1 to the 2byte quotient while handling overflow of the low byte
          <<<<<<<<+
          [->>>>>>>+>+<<<<<<<<]>>>>>>>>[-<<<<<<<<+>>>>>>>>]+<[[-]>-<]>[-
            <<<<<<<<<+>>>>>>>>>
          ]<<<<<<<<

          >>>->>>>>
        ]>[-

        ]
        <<
      ]<<<<
    ]<
  ]<

  Check if both bytes of the quotient are not 0
  >>><<<<
  [->+>+<<]>>[-<<+>>]<[[-]>>>+<<<] <<
  [->>+>+<<<]>>>[-<<<+>>>]<[[-]>>>+<<<] >>>
  [[-]
    The quotient is not 0
    Shift the tape right by 2 and place a marker that there is a digit here
    Also set the main loop flag on at its new position
    <<<<<[->>>>>>>>+<<<<<<<<]>[->>>>>>>>+<<<<<<<<]
    <+>>>>+>
  ]
  <
]

Now walk back through the list of digits and print them in the ascii range
Then print a final line break and finish
<<<<+
[+++++[-<++++++++>]<.[-]<]
++++++++++.[-]
