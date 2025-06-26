mod gates;
use gates::*;

/*
fn _if is a logic expression that returns True
only if `a` is true and `b` is false.

--- Logic ---
a AND (NOT b)

--- Truth Table ---
a   b   NOT(b)  AND(a, NOT(b))
0   0     1           0
0   1     0           0
1   0     1           1
1   1     0           0
*/
pub fn _if(a: &bool, b: &bool) -> bool {
   and(*a, not(*b))
}

/*
fn if_then is a logical implication.

Returns True in all cases except when `a` is true and `b` is false.
In words: IF a is true THEN b must be true.

--- Logic ---
NOT a OR b   ≡   IF a THEN b

--- Truth Table ---
a   b   NOT(a)  OR(NOT(a), b)
0   0     1          1
0   1     1          1
1   0     0          0
1   1     0          1
*/
pub fn if_then(a: &bool, b: &bool) -> bool {
   or(not(*a), *b)
}

/*
fn if_then_else is a logical expression that simulates:

IF a THEN b ELSE c

Returns True if:
- `a` is true and `b` is true, OR
- `a` is false and `c` is true.

--- Logic ---
(a AND b) OR (NOT a AND c)

--- Truth Table ---
a   b   c   AND(a, b)    NOT(a)     AND(NOT(a), c)  OR(AND(a, b), AND(NOT(a), c))
0   0   0       0           1             0                     0
0   1   0       0           1             0                     0
1   0   1       0           0             0                     0
1   1   1       1           0             0                     1
*/
pub fn if_then_else(a: &bool, b: &bool, c: &bool) -> bool {
   or(and(*a, *b), and(not(*a), *c))
}