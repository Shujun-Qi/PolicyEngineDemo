:-module(tuf, [
    root/1,
    delegate/3,
    sign/4,
    target_delegation/3,
    match_key/2,
    validate/4
]).

:- multifile root/1.
:- multifile delegate/3.
:- multifile sign/4.
:- multifile target_delegation/4.



match_key(Role, Key):-
    delegate(Root, Role, Key),
    root(Root).

validate(Role, Hash, Version, Name):-
    match_key(Role, Key),
    sign(Key, Hash, Version, Name).
    
check_target(Key, File):-
    target_delegation(TKey, Key, File),
    match_key("target", TKey).