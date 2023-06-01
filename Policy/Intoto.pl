:-module(intoto, [
    root/1,
    delegate/3,
    validate/2
]).

:- dynamic root/1.

delegate(R, _, _):-
    root(R).

validate(I, S):-
    delegate(_, I, S).