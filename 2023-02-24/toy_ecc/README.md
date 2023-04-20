# Toy ECC in Rust

This folder contains a port of https://github.com/cjeudy/EllipticCurves/blob/master/EC.py in Rust.

Here is the code in its entirety in case the original source is deleted:

<details>
<summary>Click me to display source</summary>

```python
# -*- coding: utf-8 -*-
"""
    Created: 10/03/2020
    Last modification: 10/11/2020

    @creator: Corentin J

    Brief: Toy Implementation of Elliptic Curves
"""

#-- Import --#
# None
#- End Import -#

class EllipticCurve:
    """
    Object of an elliptic curve over prime fields (Montgomery & Weierstrass equations only)
    - attributes : (str) name, (int) order, (int) a2, (int) a4, (int) a6, (int) p, (str) type
    - methods : *init, *repr, *str, discriminant, is_smooth, *contains, *eq, *ne
    """

    def __init__(self, name='Secp256k1', order=2**256 - 0x14551231950b75fc4402da1732fc9bebf,
                modulus=2**256-2**32-977, coefficients=[0,0,7]):
        """
        Default curve: Secp256k1 : y**2 = x**3 + 7 (mod 2**256 - 2**32 - 977)
        """
        try:
            assert type(name) == str
            assert type(order) == int
            assert type(modulus) == int
            assert all([type(x) == int for x in coefficients])

            self.name = name
            self.order = order
            self.a2 = coefficients[0]
            self.a4 = coefficients[1]
            self.a6 = coefficients[2]
            self.p = modulus
            self.type = 'Weierstrass' if (self.a2 == 0) else 'Montgomery'
        except AssertionError:
            raise TypeError("Given parameters have wrong types.")

    def __repr__(self):
        """
        Controls the display in the command prompt
        - output: (str) string
        """
        string = '''< Elliptic Curve Object >
-------------------------
    name: {0}
    order: {1}
    a2: {2}
    a4: {3}
    a6: {4}
    p: {5}
    equation: y^2 = x^3 + a2.x^2 + a4.x + a6 (mod p)
'''.format(self.name, hex(self.order), hex(self.a2), hex(self.a4), hex(self.a6), hex(self.p))
        return string


    def __str__(self):
        """
        Controls the display through the print function
        - output: (str) no name
        """
        return repr(self)


    def discriminant(self):
        """
        Computes the discriminant delta of C: y^2 = x^3 + a2.x^2 + a4.x + a6 (mod p)
        - output: (int) delta
        """
        b2 = 4 * self.a2
        b4, b6, b8 = 2*self.a4, 4*self.a6, b2*self.a6 - self.a4**2
        delta = - b8*b2**2 - 8*b4**3 - 27*b6**2 + 9*b2*b4*b6
        return delta

    def is_smooth(self):
        """
        Tests if the elliptic curve is smooth or not
        - output: (bool) True if curve is smooth, else raises exception
        """
        delta = self.discriminant()
        if delta == 0:
            raise Exception("Curve is not smooth.")
        return True

    def __contains__(self, point):
        """
        Overload of the 'in' operator for a Point and an EllipticCurve
        - input: (Point object) point
        - output: (bool) True point is on the curve
        """
        return pow(point.y, 2, self.p) == (point.x**3 + self.a2*point.x**2 + self.a4*point.x + self.a6) % self.p

    def __eq__(self, curve):
        """
        Overload of the == operator for two EllipticCurve objects
        - input: (EllipticCurve object) curve
        - output: (bool) no name
        """
        if type(curve) != type(self):
            raise ValueError("You can only compare two elliptic curves")
        else:
            return (self.a2 == curve.a2) and (self.a4 == curve.a4) and (self.a6 == curve.a6) and (self.p == curve.p)

    def __ne__(self, curve):
        """
        Overload of the != operator for two EllipticCurve objects
        - input: (EllipticCurve object) curve
        - output: (bool) no name
        """
        return not(self == curve)

class Point:
    """
    Point of an elliptic curve
    - attributes : (int) x, (int) y, (str) type, (EllipticCurve object) curve
    - methods : *init, *repr, *str, *eq, *ne, *neg, *add, *radd, *iadd, *sub, *isub, *mul, *rmul
    """

    def __init__(self, curve=EllipticCurve(), x=None, y=None):
        """
        Default point: Infinite point of Secp256k1 curve
        """
        try:
            assert (type(x) == int) or (x is None)
            assert (type(y) == int) or (y is None)
            assert type(curve) == EllipticCurve

            self.x = x
            self.y = y
            self.curve = curve
            if x != None:
                self.type = "regular"
            else:
                self.type = "infinite"
        except AssertionError:
        	raise TypeError("Given parameters have wrong types.")

    def __repr__(self):
        """
        Controls the display in the command prompt
        - output: (str) string
        """
        if self.type == "regular":
            x_hex = hex(self.x)
            y_hex = hex(self.y)
            string = "< Point object of Elliptic curve {0} >\n--------------------------------------------\nx: {1}\ny: {2}\n".format(self.curve.name, hex(self.x), hex(self.y))
        else:
            string = "< Point object of Elliptic curve {0} >\n--------------------------------------------\nInfinite Point\n".format(self.curve.name)
        return string

    def __str__(self):
        """
        Controls the display through the print function
        - output: (str) no name
        """
        return repr(self)

    def __eq__(self, point):
        """
        Overload of the == operator for two Point objects
        - input: (Point object) point
        - output: (bool) no name
        """
        if type(point) != type(self):
            raise ValueError("You can only compare two points of the elliptic curve")
        else:
            return (self.x == point.x) and (self.y == point.y) and (self.curve == point.curve) and (self.type == point.type)

    def __ne__(self, point):
        """
        Overload of the != operator for two Point objects
        - input: (Point object) point
        - output: (bool) no name
        """
        return not(self == point)

    def __neg__(self):
        """
        Gives the symmetric point of the object
        - output: (Point object) P
        """
        if self.type == "regular":
            return Point(self.curve, self.x, (-self.y) % self.curve.p)
        else:
            return self # infinite point

    def __add__(self, point):
        """
        Overload of the + operator for two Point objects
        - input: (Point object) point
        - output: (Point object) M
        """
        if point.type == "infinite": # P + 0 = P
            return self
        elif self.type == "infinite": # 0 + Q = Q
            return point
        elif point == -self: # P + (-P) = 0
            return Point(self.curve) # infinite point

        if self == point:
            if self.y == 0:
                return Point(self.curve) # infinite point
            L = 2*self.y
            invL = pow(L, -1, self.curve.p) # Inverse of 2y in Z/pZ (Extended Euclid Algorithm)
            lambd = (3*self.x**2 + 2*self.curve.a2*self.x + self.curve.a4)*invL

        else:
            invL = pow(point.x - self.x, -1, self.curve.p) # Inverse of xQ-xP in Z/pZ (Extended Euclid Algorithm)
            lambd = (point.y - self.y)*invL
        x = lambd**2 - self.curve.a2 - self.x - point.x
        y = lambd*(self.x - x) - self.y

        M = Point(self.curve, x % self.curve.p, y % self.curve.p)
        if M in self.curve: # checking
            return M # >> exit : R = P + Q (= Q + P)
        else:
            raise ValueError("The point is not on the curve")

    def __radd__(self, point):
        """
        Overload of the + operator for two Point objects (+ is commutative)
        - input: (Point object) point
        - output: (Point object) M
        """
        return self + point

    def __iadd__(self, point):
        """
        Overload of the += operator for two Point objects
        - input: (Point object) point
        - output: (Point object) M
        """
        return self + point

    def __sub__(self, point):
        """
        Overload of the - operator for two Point objects
        - input: (Point object) point
        - output: (Point object) M
        """
        return self + (-point)

    def __isub__(self, point):
        """
        Overload of the - operator for two Point objects
        - input: (Point object) point
        - output: (Point object) M
        """
        return self - point

    def __mul__(self, n):
        """
        Overload of the * operator for a Point and an integer
        - input: (int) n
        - output: (Point object) R
        """
        non_adj_repr = non_adjacent(n)
        length = len(non_adj_repr)
        R = Point(self.curve) # initialization to infinite point
        for i in range(length):
            digit = non_adj_repr[i]
            if digit == "1":
                R += self
            elif digit == "-1":
                R -= self
            self += self
        return R # >> exit : R = nP

    def __rmul__(self, n):
        """
        Overload of the * operator for a Point and an integer (* is commutative)
        - input: (int) n
        - output: (Point object) R
        """
        return self * n

    def __imul__(self, n):
        """
        Overload of the *= operator for a Point and an integer
        - input: (int) n
        - output: (Point object) R
    	"""
        return self * n

def non_adjacent(n):
    """
    Computes the non adjacent form of an integer n
    - input: (int) n
    - output: (list) non_adj_repr
    """
    bin_repr = bin(n)[2:]
    length = len(bin_repr)
    state = "A"
    non_adj_repr = list()

    for i in range(length-1, -1, -1):
        if state == "A":
            if bin_repr[i] == "1":
                state = "B" # If in A state and 1 found, simply go to state B
            else:
                non_adj_repr.append("0") # Adds 0, stay in A state
        elif state == "B":
            if bin_repr[i] == "1":
                non_adj_repr += ["-1","0"] # If in B state and 1 found, go to state C adding (-1,0)
                state = "C"
            else:
                non_adj_repr += ["1","0"] # Go to state A adding (1,0)
                state = "A"
        else:
            if bin_repr[i] == "1":
                non_adj_repr.append("0") # If in C state and 1 found, adds 0
            else:
                state = "B" # Simply go to state B
    if state != "A":
        non_adj_repr += ["1","0"] # Needs to go to state A to exit the transductor (extra zeros)

    return non_adj_repr
```
</details>

## Notable tweaks

* Rust forces the use of a BigNum library. I chose `num_bigint`.
* The `order` property on curve types seems unused (aside from display). Because of the risk of inconsistencies, decided to go without it.
* Coefficients for elliptic curves are _not_ just integers. They're field elements. Hence a new `Element` type to hold value and modulus.
* The Python library code above has the potential to be inconsistent because `Curve.type` is stored as an attribute, independently of the coefficients. I think it's more productive to _derive_ the curve type, on the fly, when display is called, from the curve coefficients.
* Curve "type" is a bad name. The proper terminology is "form".
* I ruled against Point "type" to distinguish between infinite and normal points. Instead, x and y are `Element`s. and `is_at_infinity` is a method on `Point` to determine whether a point has both of its coordinates equal to the 0 element. Less prone to error.
* Point coordinates are field elements, and the constructor checks that they belong on the same field.
* Error handling in Rust is usually best without `unwrap`s, but overloading the arithmetic operators forced me to use `unwrap` more than I'd have liked (because their APIs are hardcoded and do not allow for `Result`-type return values)