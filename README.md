# primality_check_v1

This is a simple program in RUST to check primality of a number using a relatively brute force mechanism.

The program checks if the input number "N" is a prime, by checking the following:

1. If the entered number "N" is 1 or less then return "false"

2. If the entered number "N" is equal to 2 then return "true"

3. Else it implements the brute force algorithm of checking the divisibility of the given number in the range 2 to SQRT(N). If it is divisible the returned value is "false", else "true" is returned
