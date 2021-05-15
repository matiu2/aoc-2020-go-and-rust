# Find which passwords are valid

## Example input:

    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc

## Format:

    line ::= policy: password
    policy ::= low-high letter
    low ::= the minimum number of letters the pasword must contain
    high ::= the maximum number of letters the pasword must contain
    letter ::= The letter that the policy applies to
    password ::= Just a string of bytes

## For example:

In the above policy document, the second line is invalid, because the password must contain 1-3 'b's - but has 0

