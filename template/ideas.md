
Ideas for a simple templating library
=====================================

* Library is parameterized by the opening and closing delimiter characters which
are used for control sequences.
* Library exposes an iface of things which can be bound to template variables
  * Support json values and values with a conversion to string

Syntax of Control Sequences
===========================

    timestamp

Insert the current time in ISO format

    for WORD1 in WORD2

Loop over the array bound to WORD2 with WORD1 bound to each value in turn

    index

The current array index

    for WORD1, WORD2 in WORD3

Loop over the map bound to WORD3 with WORD1 bound to each key and WORD2 to each value

    enter WORD

Use the map bound to WORD as the scope for looking up bindings

    if not? exists? WORD
    else if not? exists? WORD
    else

Condition stuff

    end

The end of any block-structured thinger

    error MESSAGE

Print an error message and fail

    WORD

The value bound to WORD

