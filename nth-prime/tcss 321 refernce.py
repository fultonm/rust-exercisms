"""
    TCSS 321 Mini Project 3
"""

import collections

# Part A
def largest_prime_factor(the_n):
    """
        Part A:
        Finds the largest prime factor using trial division.
        We start with a list of all prime numbers, and starting from the largest number in the list we check
        if it is divisible by n

        n: A number >= 2 to find the largest prime factor of.
    """
    primes = eratosthenes_seive(the_n)
    for prime in reversed(primes):
        if the_n % prime == 0:
            return prime
    return 'Largest Prime Factor: n must be greater than 2'

# Part B
def calculate_residue_modular_exp(the_b, the_n, the_m):
    """
        Calculates the residue of b^n (mod m) where n is possibly very large.
        We will use modular exponentiation so it is possible for mere processor
        to handle it.

        the_b: An integer
        the_n: An integer, the_b's large exponent
        the_m: The modulus value
    """
    result = 1
    # Binary representation, Omitting the first 2 characters and reverse the string.
    n_2 = bin(the_n)[2::][::-1]
    power = the_b % the_m
    for i in range(0, len(n_2)):
        if n_2[i:i + 1] is '1':
            result = (result * power) % the_m
        power = (power ** 2) % the_m
    return result

# Part C
def inverse():
    """
        Finds the inverse of the mod expression.

        TODO
    """
    return 0

# Part D
def least_common_multiple_of_range(the_n):
    """
        Calculates the least common multiple of the numbers in range 1 to n

        the_n: The last number in the set from 1 to n
    """
    primes = eratosthenes_seive(the_n)
    result = 1
    for prime in primes:
        power = largest_power_lt_n(prime, the_n)
        result *= prime ** power
    return result

# Part E


def sum_even_fibs(the_n):
    """
        Calculates the sum of even Fibonacci numbers up to n.

        the_n: n, The largest Fibonacci number to sum.
    """
    fibs = fibonnaci_sequence(the_n)
    fib_sum = 0
    for _, value in fibs.iteritems():
        if value % 2 == 0:
            fib_sum += value
    return fib_sum

# Bonus


def longest_chain(the_n):
    """
        Determines the longest chain of numbers generated
        by the piecewise function.

        the_n: The value to test.
    """
    longest_length = 0
    for i in range(1, the_n):
        chain = generate_piecewise_chain(i)
        length = len(chain)
        if length > longest_length:
            longest_length = length
    return longest_length

# Helpers


def fibonnaci_sequence(the_n):
    """
        Generates the fibonacci sequence up to n.

        the_n: The wall to stop finding fibonacci numbers after they exceed n.
    """
    fibs = collections.OrderedDict()
    fibs[1] = 1
    fibs[2] = 1
    i = 3
    while fibs[i - 1] < the_n:
        fibs[i] = fibs[i - 2] + fibs[i - 1]
        i += 1
    return fibs

def generate_piecewise_chain(the_n):
    """
        Evaluate the piecewise function and append the value resulting
        from each step in the evaluation.

        the_n: The number to seed the function with.
    """
    our_n = the_n
    chain = []
    while our_n > 1:
        chain.append(our_n)
        if our_n % 2 == 0:
            our_n /= 2
        else:
            our_n = (3 * our_n) + 1
    chain.append(1)
    return chain


def eratosthenes_seive(the_n):
    """
        Calculates the prime numbers up to n using a Seive of Eratosthenes approach.
        We start with a list of all numbers from 0 to n.
        Then for each number p we remove the square of p, and all numbers above p^2 which
        are multiples of p.
        This leaves us with all prime numbers up to n.

        n: The number to stop finding primes
    """
    sqrt_n = the_n ** 0.5
    numbers = range(0, the_n)
    for number in numbers:
        if number < 2:
            continue
        elif number > sqrt_n:
            break
        for i in range(number ** 2, the_n, number):
            numbers[i] = 0
    return [x for x in numbers if x > 1]


def largest_power_lt_n(the_x, the_n):
    """
        Finds the largest power that x can be taken to while
        less than n

        x: The number to take to the largest possible power.
        n: The number to not exceed.
    """
    power = 0
    while the_x ** power < the_n:
        power += 1
    return power - 1


def execute_examples():
    """
        Executes the functions with sample input to check expected output.
    """
    print('A: Largest prime factor of 23508701:')
    print(str(largest_prime_factor(23508701)))
    print('B: Evaluate 98^201 (mod 337):')
    print(str(calculate_residue_modular_exp(98, 201, 337)))
    print('C: Calculate modular inverse:')
    print(str(inverse()))
    print('D: Least common multiple of {1, 2, ..., 20}:')
    print(str(least_common_multiple_of_range(20)))
    print('E: Sum of even fibonaccis up to 4 M:')
    print(str(sum_even_fibs(4000000)))
    print('Longest length of piecewise chain under 1 M:')
    print(str(longest_chain(1000000)))

execute_examples()
