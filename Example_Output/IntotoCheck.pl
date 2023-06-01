:- consult('IntotoAsserts.pl').
main:-
    intoto:validate('12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2','fetch'),
    intoto:validate('12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2','extract'),
    intoto:validate('12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2','modify'),
    intoto:validate('12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2','build'),
    nl.
