# Number Theory Module

#### Overview 

##### Why Number Theory?
Number theory is the basis of the study of solvable equations. Solvability of a problem is key to cryptography. Since if a problem is swiftly solvable in one direction, but hard in the other direction, then it makes a good candidate for cryptography. Consider the discrete log problem:

log_b a = c where a,b,c are integers  

so  

b^c = a  

Provided b,c it is very easy to determine a. Provided a, it is very difficult to determine b,c. There are special cases where the problem becomes trivial, but there is no generalized solution. This problem is the basis for both discrete logarithm cryptography and ellipitic curve cryptography. Algorithms such as the older DSA and the more modern RSA are both reliant on this very problem. Number theory provides ways to analyze and consider this problem to determine the extent of solvability. That is to say, what information is required to solve and under what conditions it becomes trivially solvable.

##### What is this module?
This module provides numerous common mathematical functions from number theory. Some will be used in various parts of the cryptography. Others are less useful, but I put them in anyways.
