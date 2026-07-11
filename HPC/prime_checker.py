import time
import math

N = 27

def is_prime(n):
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

numbers = [x for x in range(0, N+1)]
primes = [x for x in numbers if is_prime(x)]

print(primes)
