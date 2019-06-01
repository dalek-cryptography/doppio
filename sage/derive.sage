# Adapted from the JubJub script.
q = 2**252 + 27742317777372353535851937790883648493
Fq = GF(q)

# We want to find an Edwards curve with a small 'd' parameter.
# To check conditions on the order, we transfer to the isogenous Montgomery
# curve.
def get_A(d):
    # a = 1, so 2 - 4*d/a = 2 - 4*d
    return 2 - 4*d

# Instead of searching for a curve of order 8 with twist order 4,
# allow searching for a curve of order 4 with twist order 8, or vv
desired_cofactor = 4
desired_twist_cofactor = 8

import random
from datetime import datetime

def check(d):
    # We want d to be nonsquare
    if Fq(d).is_square():
        return False

    A = Fq(get_A(d))

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
                    print("FOUND CURVE d = %s", d)
                    return True

    if random.random() < 0.001:
        print("%s: did not find curve at d = %s" % (datetime.now(), d))

    return False

def check_abs(d):
    return (check(d), check(-d))

import multiprocessing as mp

def check_parallel(start, end):
    candidates = range(start, end)
    with mp.Pool(8) as p:
        # Just discard the results and rely on stdout
        # instead of syncing progress across threads
        p.map(check_abs, candidates, chunksize=1)
    