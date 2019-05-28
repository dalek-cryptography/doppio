# Adapted from the JubJub script.
q = 2**252 + 27742317777372353535851937790883648493
Fq = GF(q)

# We wish to find a Montgomery curve with B = 1 and A the smallest such
# that (A - 2) / 4 is a small integer.
def get_A(n):
   return (n * 4) + 2

# A = 2 is invalid (singular curve), so we start at i = 1 (A = 6)
i = 1

# Instead of searching for a curve of order 8 with twist order 4,
# allow searching for a curve of order 4 with twist order 8, or vv
desired_cofactor = 8
desired_twist_cofactor = 4

def check(i):
    A = Fq(get_A(i))

    # We also want that A^2 - 4 is nonsquare.
    if ((A^2) - 4).is_square():
        return False

    ec = EllipticCurve(Fq, [0, A, 0, 1, 0])
    o = ec.order()

    if (o % desired_cofactor == 0):
        o = o // desired_cofactor
        if is_prime(o):
            twist = ec.quadratic_twist()
            otwist = twist.order()
            if (otwist % desired_twist_cofactor == 0):
                otwist = otwist // desired_twist_cofactor
                if is_prime(otwist):
                    print("A = %s" % A)
                    return True
    if i % 1000 == 0:
        print("did not find curve at i = %s" % i)
    return False

import multiprocessing as mp

def check_parallel(start, end):
    candidates = range(start, end)
    with mp.Pool(8) as p:
        # Just discard the results and rely on stdout
        # instead of syncing progress across threads
        p.map(check, candidates)
    
