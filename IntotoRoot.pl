:- consult('Policy/intoto.pl').
intoto:root('12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2').
intoto:layout('0xffffc023f058','0xffffc023f428','0xaaaaed8f6f90').
intoto:layoutSigned('0xffffc023edd0','layout','','2017-09-02T12:57:02Z','['0xaaaaed8f5ae0','0xaaaaed8f5b78','0xaaaaed8f5c10','0xaaaaed8f5ca8',]','[]','0xffffc023f428').
intoto:steps('0xffffc023eb98','step','fetch','["dget", "http://cdn.debian.net/debian/pool/main/g/grep/grep_2.12-2.dsc"]','["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"]','1','[["DISALLOW", "*"]]','[["ALLOW", "*"]]').
intoto:steps('0xffffc023eb98','step','extract','["dpkg-source", "-x", "grep_2.12-2.dsc"]','["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"]','1','[["MATCH", "*", "WITH", "PRODUCTS", "FROM", "fetch"]]','[["ALLOW", "*"]]').
intoto:steps('0xffffc023eb98','step','modify','[]','["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"]','1','[["MATCH", "*", "WITH", "PRODUCTS", "FROM", "extract"]]','[["ALLOW", "*"]]').
intoto:steps('0xffffc023eb98','step','build','["dpkg-buildpackage", "-us", "-uc"]','["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"]','1','[["MATCH", "*", "WITH", "PRODUCTS", "FROM", "modify"]]','[["ALLOW", "*"]]').
{"12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2": IntotoKeys { keyid: "12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2", keytype: "rsa", keyval: IntotoKeysVal { private: "", public: "-----BEGIN PUBLIC KEY-----\nMIIBojANBgkqhkiG9w0BAQEFAAOCAY8AMIIBigKC..." } }}
intoto:signatures('0xffffc023eea8','12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2','RSASSA-PSS','9c3a5609e79ea68f9d7aa4b72e5ab1f00b7773f06912b789add45da026f46621b9e...').
