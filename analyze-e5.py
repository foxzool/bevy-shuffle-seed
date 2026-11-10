import re,collections,time
L=open('logs/21-run-e5.log').read()
rep,scan=L.split('## scan')
def parse(t):
    d=collections.defaultdict(list)
    for m in re.finditer(r'^([BF]) seed=(\S+) (.*)$',t,re.M): d[(m[1],m[2])].append(m[3])
    return d
r=parse(rep); s=parse(scan)
print("# analysis generated", time.strftime('%F %T %z'), "from logs/21-run-e5.log")
print("## repeat (5 processes each): distinct outputs per (scenario,seed)")
ok=True
for k,v in sorted(r.items()):
    n=len(set(v)); ok&=(n==1 and len(v)==5); print(k,"runs",len(v),"distinct",n)
print("REPEAT_ALL_IDENTICAL",ok)
for sc in "BF":
    print(f"## scenario {sc} scan")
    print("baseline none:",s[(sc,'none')][0])
    saw=collections.Counter(); before=0; allsaw=collections.Counter()
    for (x,seed),v in s.items():
        if x!=sc: continue
        out=v[0]; val=re.search(r'consume_saw=(\S+)',out)[1]; allsaw[val]+=1
        if seed=='none': continue
        saw[val]+=1
        if out.index('"consume"')<out.index('"produce"'): before+=1
    print("non-None seeds:",sum(saw.values()),"consume_saw:",dict(saw),"consume_before_produce:",before)
    print("all values incl None:",sum(allsaw.values()),"consume_saw:",dict(allsaw))
    print("distinct orders incl None:",len(set(v[0].split(' consume_saw')[0] for (x,_),v in s.items() if x==sc)))
