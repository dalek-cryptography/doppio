# Adapted from the JubJub script.
#
# Instead of searching for a curve of order 8 with twist order 4,
# search for a curve of order 4 with twist order 8.
q = 2**252 + 27742317777372353535851937790883648493
Fq = GF(q)

# We wish to find a Montgomery curve with B = 1 and A the smallest such
# that (A - 2) / 4 is a small integer.
def get_A(n):
   return (n * 4) + 2

# A = 2 is invalid (singular curve), so we start at i = 1 (A = 6)
i = 1

while True:
    A = Fq(get_A(i))
    i = i + 1

    # We also want that A^2 - 4 is nonsquare.
    if ((A^2) - 4).is_square():
        continue

    ec = EllipticCurve(Fq, [0, A, 0, 1, 0])
    o = ec.order()

    if (o % 4 == 0):
        o = o // 4
        if is_prime(o):
            twist = ec.quadratic_twist()
            otwist = twist.order()
            if (otwist % 8 == 0):
                otwist = otwist // 8
                if is_prime(otwist):
                    print "A = %s" % A
                    exit(0)
