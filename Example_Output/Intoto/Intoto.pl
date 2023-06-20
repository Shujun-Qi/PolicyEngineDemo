:-consult(Policy/Intoto.pl).

#Facts from DemoData/Intoto/build.link.
/home/lukas/demo/grep-2.12/.pc/.quilt_series:materials("9afbb183d1b683d2770aecb9b379093804ccc56027f07ecf7fc252d5b93a8df2").
/home/lukas/demo/grep-2.12/.pc/02-man_rgrep.patch/doc/grep.in.1:materials("0c933cbdacb739761f7cf9be1d9c0543b8a5cb0c1cf4031fc7add8e81146c1a6").
/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/src/pcresearch.c:materials("08e24565e42cf5e2aeff2ca65cc4e3a8de5fa65260ea1a2d1ae872b45c3b6a25").
/home/lukas/demo/grep-2.12/.pc/70-man_apostrophe.patch/doc/grep.in.1:materials("eee01bf4de92df307b7f1b6fa740e1c0e0c8c28d6a12b6939ddec0708699da25").
/home/lukas/demo/grep-2.12/.pc/04-446854-grep.1.patch/doc/grep.in.1:materials("90b74e6c5542b0db506c372f77c45ceab8d7289432cfe37bce11d611a4827a4d").
/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/configure:materials("fda1f6c62f081999b74177f560e14db0b0b2f93cd632ed4a02f6ac2582fc27bf").
/home/lukas/demo/grep-2.12/.pc/.version:materials("53c234e5e8472b6ac51c1ae1cab3fe06fad053beb8ebfd8977b010655bfdd3c3").
/home/lukas/demo/grep-2.12/.pc/.quilt_patches:materials("0623de532bc23399e87e6c1914e8e90e999efbfd26b6b956666a493893739f0d").
/home/lukas/demo/grep-2.12/.pc/80-587930-man-ere-reference.patch/doc/grep.in.1:materials("30378876bf1c1a13449c993fb7a6406089f7f77a34f96fed5c6c742e75a395b6").
9404120044764687080:materials(/home/lukas/demo/grep-2.12/.pc/.quilt_series,/home/lukas/demo/grep-2.12/.pc/02-man_rgrep.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/src/pcresearch.c,/home/lukas/demo/grep-2.12/.pc/70-man_apostrophe.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/04-446854-grep.1.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/configure,/home/lukas/demo/grep-2.12/.pc/.version,/home/lukas/demo/grep-2.12/.pc/.quilt_patches,/home/lukas/demo/grep-2.12/.pc/80-587930-man-ere-reference.patch/doc/grep.in.1).
2210284986068062705:byproducts(0,"dpkg-buildpackage: source package grep\ndpkg-buildpackage: source ve..."," dpkg-source --before-build grep-2.12\n fakeroot debian/rules clean\n...").
17046612084282012057:link("link","build",["dpkg-buildpackage","-us","-uc"],9404120044764687080,2210284986068062705,6086514543335466803).
10982446533290072216:sig("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2","RSASSA-PSS","3877a06dee3f6e140e20d5100964adb38c1c63dd89803087204d561c0adbfed17c2...").
3999759271652749985:sign(17046612084282012057,[10982446533290072216]).

#Facts from DemoData/Intoto/extract.link.
/home/lukas/demo/grep_2.12.orig.tar.bz2:materials("0119987171cd60b87c89557524fc6636bdad770dae71917adcaef6abffb1be67").
/home/lukas/demo/grep_2.12-2.dsc:materials("a6a63fd21da40d11ce6ae2bc6f633bd27cd206c2348b2ef306e1b68120f7e58e").
/home/lukas/demo/grep_2.12-2.debian.tar.bz2:materials("37887d8aecec66e75365abd8c41be94f75e7a3acf1d6b27fc121584f47281525").
6710812177529316956:materials(/home/lukas/demo/grep_2.12.orig.tar.bz2,/home/lukas/demo/grep_2.12-2.dsc,/home/lukas/demo/grep_2.12-2.debian.tar.bz2).
16286064594038341640:byproducts(0,"dpkg-source: info: extracting grep in grep-2.12\ndpkg-source: info: ...","gpgv: Signature made Sun 13 May 2012 08:23:12 AM EDT using DSA key ...").
15559580635576652301:link("link","extract",["dpkg-source","-x","grep_2.12-2.dsc"],6710812177529316956,16286064594038341640,6086514543335466803).
3528252443882684044:sig("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2","RSASSA-PSS","9b89cf0a74a1d53359460982cdc3dcbdef2b291a284b977fadb839501d7324fe581...").
9232462238871463086:sign(15559580635576652301,[3528252443882684044]).

#Facts from DemoData/Intoto/fetch.link.
6086514543335466803:materials().
2226614402344454911:byproducts(1,"dget: retrieving http://cdn.debian.net/debian/pool/main/g/grep/grep...","--2017-08-08 11:05:06--  http://cdn.debian.net/debian/pool/main/g/g...").
16469545966398679882:link("link","fetch",["dget","http://cdn.debian.net/debian/pool/main/g/grep/grep_2.12-2.dsc"],6086514543335466803,2226614402344454911,6086514543335466803).
10497978779902872773:sig("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2","RSASSA-PSS","a0d3237a59a96343b2e2caee2bf0e27a249a01f796c3fec53f184fc2720b44dd7b2...").
17958788776441355237:sign(16469545966398679882,[10497978779902872773]).

#Facts from DemoData/Intoto/modify.link.
/home/lukas/demo/grep-2.12/.pc/.quilt_series:materials("9afbb183d1b683d2770aecb9b379093804ccc56027f07ecf7fc252d5b93a8df2").
/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/src/pcresearch.c:materials("08e24565e42cf5e2aeff2ca65cc4e3a8de5fa65260ea1a2d1ae872b45c3b6a25").
/home/lukas/demo/grep-2.12/.pc/.version:materials("53c234e5e8472b6ac51c1ae1cab3fe06fad053beb8ebfd8977b010655bfdd3c3").
/home/lukas/demo/grep-2.12/.pc/02-man_rgrep.patch/doc/grep.in.1:materials("0c933cbdacb739761f7cf9be1d9c0543b8a5cb0c1cf4031fc7add8e81146c1a6").
/home/lukas/demo/grep-2.12/.pc/80-587930-man-ere-reference.patch/doc/grep.in.1:materials("30378876bf1c1a13449c993fb7a6406089f7f77a34f96fed5c6c742e75a395b6").
/home/lukas/demo/grep-2.12/.pc/70-man_apostrophe.patch/doc/grep.in.1:materials("eee01bf4de92df307b7f1b6fa740e1c0e0c8c28d6a12b6939ddec0708699da25").
/home/lukas/demo/grep-2.12/.pc/04-446854-grep.1.patch/doc/grep.in.1:materials("90b74e6c5542b0db506c372f77c45ceab8d7289432cfe37bce11d611a4827a4d").
/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/configure:materials("fda1f6c62f081999b74177f560e14db0b0b2f93cd632ed4a02f6ac2582fc27bf").
/home/lukas/demo/grep-2.12/.pc/.quilt_patches:materials("0623de532bc23399e87e6c1914e8e90e999efbfd26b6b956666a493893739f0d").
9404120044764687080:materials(/home/lukas/demo/grep-2.12/.pc/.quilt_series,/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/src/pcresearch.c,/home/lukas/demo/grep-2.12/.pc/.version,/home/lukas/demo/grep-2.12/.pc/02-man_rgrep.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/80-587930-man-ere-reference.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/70-man_apostrophe.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/04-446854-grep.1.patch/doc/grep.in.1,/home/lukas/demo/grep-2.12/.pc/03-397262-dlopen-pcre.patch/configure,/home/lukas/demo/grep-2.12/.pc/.quilt_patches).
15055435855460970875:byproducts(null,null,null).
7419300989600828064:link("link","modify",],9404120044764687080,15055435855460970875,6086514543335466803).
17182074763062909189:sig("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2","RSASSA-PSS","6c922765260c71bbe0eef0267c2ad6f226eeec43ef45b5dcba6314f52adba655265...").
4988575101953877571:sign(7419300989600828064,[17182074763062909189]).

#Facts from DemoData/Intoto/root.layout.
4203241373099102994:steps("step","fetch",["dget","http://cdn.debian.net/debian/pool/main/g/grep/grep_2.12-2.dsc"],["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"],1,[["DISALLOW","*"]],[["ALLOW","*"]]).
14138240092749512347:steps("step","extract",["dpkg-source","-x","grep_2.12-2.dsc"],["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"],1,[["MATCH","*","WITH","PRODUCTS","FROM","fetch"]],[["ALLOW","*"]]).
4591744865323227800:steps("step","modify",],["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"],1,[["MATCH","*","WITH","PRODUCTS","FROM","extract"]],[["ALLOW","*"]]).
16355790422116142028:steps("step","build",["dpkg-buildpackage","-us","-uc"],["12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2"],1,[["MATCH","*","WITH","PRODUCTS","FROM","modify"]],[["ALLOW","*"]]).
3621916968137504121:keyval("","-----BEGIN PUBLIC KEY-----\nMIIBojANBgkqhkiG9w0BAQEFAAOCAY8AMIIBigKC...").
12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2:keys("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2","rsa",keyval).
201413837387964744:keys(12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2).
197050818602347943:layout("layout","","2017-09-02T12:57:02Z",[4203241373099102994,14138240092749512347,4591744865323227800,16355790422116142028],],201413837387964744).
13470396812858591841:sig("12c55e46654c682d3ffd3b63492adf422e6812eb1ac41574d083b9e770d1e4c2","RSASSA-PSS","9c3a5609e79ea68f9d7aa4b72e5ab1f00b7773f06912b789add45da026f46621b9e...").
7539901198141123327:sign(197050818602347943,[13470396812858591841]).
