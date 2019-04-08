# doppio

Work-in-progress repo for an embedded curve over the ristretto255 scalar field, for use in Bulletproofs.

The curve was selected by Sean Bowe (cf https://twitter.com/ebfull/status/1087571257057406976).

# Goals

Eventually, this repo aims to construct "doppio", a parameterization of Ristretto that can be implemented using an embedded curve defined over the scalar field of the ristretto255 group.  This provides a prime-order group that can be efficiently implemented inside a Bulletproof.

