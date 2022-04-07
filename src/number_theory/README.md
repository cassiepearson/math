# Number Theory Module

#### Overview 
Most of this module is implemented in the form of trait objects. This means that all of the trait objects can operate on all types compatible with the Integer trait. This trait is inclusive of all positive and negative integers. Not all functions in these modules are actually useful on integers that could be negative. However, the goal is for the traits to be compatible with the entire integer number line.

##### What is this module?
This module provides numerous common mathematical functions from number theory. Some will be used in various parts of the cryptography. Others are less useful, but I put them in anyways.

#### Methods

##### Handling Overflowing Operations

This module deals primarily with modular arithmetic which by definition is bounded in size by the maximum size of the modulo. Meaning it is impossible for these operations to fail by overflow. Many of the other operations deal with decomposition from an original such as the continued fraction or by finding a series less than the original such as Euler totient. These will also be bound by the size of the inputs and are guaranteed not to fail.

Only one operation has a chance of overflow: factorial. This means that only the factorial and wilson's primailty ccheck have a chance of overflow. Since wrapping on a factorial (using modular arithmetic) defeats the purpose of the output and renders it meaningless without some knowledge of the number of times wrapped; the operation is checked. Factorial may fail due to overflow but will return a NumberTheoryErr detailing when it fails to multiply further.