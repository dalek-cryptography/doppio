# Curve Selection

Our goal is to provide a prime-order group defined over the
ristretto255 scalar field \\(\mathbb F_q\\), where 
\\[ 
q = 2^{252} + 27742317777372353535851937790883648493 
\\] 
is the order of the ristretto255 group, so that group operations can
be efficiently performed inside of a Bulletproof (or some other
discrete-log based proof system implemented using ristretto255).

We will proceed in two steps: first, we select a friendly elliptic
curve defined over the ristretto255 scalar field, and then we will use
Decaf on that curve to construct a prime-order group from the
non-prime-order curve.

Although Ristretto extends the Decaf technique to cover curves with
cofactor \\(8\\), it does so at a theoretical cost of complexity and a
practical cost of an extra sign check.  While the cost of a sign check
is quite small in the machine cost model for software implementations,
it is much more significant in the circuit cost model ([as
noted][daira_sign] by Daira Hopwood).  Sean Bowe [proposed][sean_curve] a
cofactor-\\(8\\) curve for this setting, but for the reasons above it
is preferable to select a curve of cofactor \\(4\\).

The [conceptual diagram][conceptual_diagram] on the Ristretto website
shows that Decaf and Ristretto do not operate on just one curve, but
on a family of curves related by isogenies (INSERT BETTER DIAGRAM AND
REWORK THIS).

While Decaf provides a prime-order group with a canonical encoding and
built-in validation, it is concievable that a user might want to use
the underlying curve directly.  For this reason we would like to
(twist security) 

Unfortunately, as noted on page 15 of the
[Costello-Smith survey][costello_smith], when \\(q \equiv 1 \pmod
4\\), it's not possible for both the curve and its quadratic twist to
have minimal cofactor \\(4\\); one can have cofactor \\(4\\) but the
other must have cofactor \\(8\\).  This means (XXX explanation of
unification of curve and twist, fit into diagram to be drawn)

To select curve parameters, we follow an analogous procedure as in the
[selection of Ed448][ed448], setting \\(a = 1\\) and searching for
\\(d\\) nonsquare with minimal absolute value such that
\\[
\mathcal E_{a,d} : ax^2 + y^2 = 1 + dx^2y^2
\\]
has cofactor \\(4\\) and its quadratic twist has cofactor \\(8\\).
Using the `derive.sage` script contained in this repo (modified from
the script used to select JubJub), we find two pairs of small parameters:
* \\( (a, d) = (1, -63071) \\)
* \\( (a, d) = (1, 63072) \\)
and choose the one with \\(d\\) least in absolute value, \\(d = -63071\\).

This Edwards curve is 4-isogenous to the Montgomery curve with \\(A = 252286\\). 
As noted in the [Decaf paper], results of [Ahmadi and
Granger][ahmadi_granger] show that the curves \\(\mathcal E_{a,d}\\)
and \\(\mathcal E_{-a, d-a}\\) are both \\(2\\)-isogenous to the same
Jacobi quartic and thus \\(4\\)-isogenous to each other.  Starting
from \\((a,d) = (1, -63071)\\), we obtain \\((-a, d-a) = (-1,-63071-1)
= (-1, -63072)\\), which is the quadratic twist of the second
parameter pair (and explains its presence).  

This means that a Decaf implementation will be able to use any of the curves
\\[
\mathcal E : x^2 + y^2 = 1 - 63071x^2y^2 \\\\
\hat {\mathcal E }: -x^2 + y^2 = 1 - 63072x^2y^2 \\\\
\mathcal M : v^2 = u(u^2 + 252286u + 1)
\\]
to implement Decaf, depending on which curve model is optimal in the relevant cost model.
For instance, a software implementation may prefer using \\(\hat
{\mathcal E}\\) while inside of a circuit \\(\mathcal M\\) may be
better.

[sean_curve]: https://twitter.com/ebfull/status/1087571257057406976
[daira_sign]: https://github.com/zcash/zcash/issues/3924#issuecomment-493775590
[conceptual_diagram]: https://ristretto.group/details/conceptual_diagram.html
[costello_smith]: https://arxiv.org/abs/1703.01863
[decaf_paper]: http://eprint.iacr.org/2015/673
[ahmadi_granger]: http://eprint.iacr.org/2011/135
