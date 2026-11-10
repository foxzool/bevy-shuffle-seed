import re,collections,time
L=open('logs/10-run-experiments.log').read()
rep,scan=L.split('## scan')
def parse(t):
    d=collections.defaultdict(list)
    for m in re.finditer(r'^([ABC]) seed=(\S+) (.*)$',t,re.M): d[(m[1],m[2])].append(m[3])
    return d
r=parse(rep); s=parse(scan)
print("# analysis generated", time.strftime('%F %T %z'), "from logs/10-run-experiments.log")
print("## ② repeat (5 processes each): distinct outputs per (scenario,seed)")
ok=True
for k,v in sorted(r.items()):
    n=len(set(v)); ok&=(n==1 and len(v)==5); print(k,"runs",len(v),"distinct",n)
print("REPEAT_ALL_IDENTICAL",ok)
print("## ③ scenario B scan")
print("baseline none:",s[('B','none')][0])
saw=collections.Counter(); diff=[]
for (sc,seed),v in s.items():
    if sc!='B' or seed=='none':continue
    out=v[0]; saw[re.search(r'consume_saw=(\S+)',out)[1]]+=1
    if out.index('"consume"')<out.index('"produce"'): diff.append(seed)
print("consume_saw distribution over",sum(saw.values()),"seeds:",dict(saw))
print("seeds where consume ran BEFORE produce (saw 0):",diff)
print("## A: distinct orders over scan + official expectations")
A={seed:v[0].split('=')[1] for (sc,seed),v in s.items() if sc=='A'}
print("distinct A orders:",len(set(A.values())), sorted(set(A.values())))
exp={'100000001':'[0, 10, 20, 21, 11]','100000030':'[0, 10, 21, 20, 11]','100000020':'[0, 10, 20, 11, 21]','100000063':'[0, 10, 21, 11, 20]','100000003':'[0, 10, 11, 20, 21]','100000080':'[0, 10, 11, 21, 20]','100000000':'[0, 11, 10, 20, 21]','100000004':'[0, 11, 10, 21, 20]','none':'[0, 11, 10, 20, 21]'}
for k,e in exp.items(): print(k,"got",A[k],"expected",e,"MATCH" if A[k]==e else "MISMATCH")
print("## ④ scenario C")
C=[(seed,v[0]) for (sc,seed),v in s.items() if sc=='C']
bad=[x for x in C if 'constraints_ok=true' not in x[1]]
print("C runs",len(C),"constraint violations",len(bad),"distinct C orders",len(set(o for _,o in C)))
