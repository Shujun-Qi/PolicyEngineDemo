:- consult('TufTarget.pl').
main:-
    tuf:match_key('timestamp','8a1c4a3ac2d515dec982ba9910c5fd79b91ae57f625b9cff25d06bf0a61c1758'),
    tuf:match_key('snapshot','59a4df8af818e9ed7abe0764c0b47b4240952aa0d179b5b78346c470ac30278d'),
    tuf:validate('timestamp', '8f88e2ba48b412c3843e9bb26e1b6f8fc9e98aceb0fbaa97ba37b4c98717d7ab', '1', 'snapshot.json').
    tuf:check_target('c8022fa1e9b9cb239a6b362bbdffa9649e61ad2cb699d2e4bc4fdf7930a0e64a','file3.txt'),
    tuf:validate('timestamp', '', '1', 'role1.json').
    tuf:match_key('target','65171251a9aff5a8b3143a813481cb07f6e0de4eb197c767837fe4491b739093'),
    tuf:validate('timestamp', '', '1', 'target.json').
