def C(n,base=128):
  if n==0:return[0]
  c=[]
  while n>0:c=[n%base]+c;n//=base
  return c
def E(c,neg=False):
  o=[]
  o.append(f"sav {c[0]} r01")
  for x in c[1:]:
    o.append("sav 7 r02");o.append("cal shl r01 r02")
    if x!=0:o.append(f"cal add {x} r01")
  if neg:o.append("cal not r01 r01"); o.append("cal add 1 r01")
  return o
def i(n,I,T):
  N=n
  a=abs(n)
  b=bin(n if n>=0 else (n+(1<<32)))
  p=" "*I
  if T:print(f"{p}{n}\n{p}Binary: {b}\n{p}127 Blocks: {n//127}\n{p}Remainder: {n%127}")
  if a==0 and T:print(f"{p}Immediate: sav 0 r01");return
  elif a==0:return
  s=0; t=a
  while t&1==0:s+=1;t>>=1
  if s:
    if T:print(f"{p}Shift form: {a>>s} << {s}")
    if (a>>s)>127:i(a>>s,I+2,T)
  c=C(a,128)
  if T:print(f"{p}Base-128 chunks: {c}")
  if T:print(f"{p}Chunk expression:",end=" ")
  e=str(c[0])
  for x in c[1:]:e=f"({e} << 7) + {x}"
  if n<0:e=f"-({e})"
  if T:print(f"{e}\n{p}Crawssembly:")
  if T:
    for line in E(c,n<0):print(f"{p}  {line}")
  return c
if __name__ == "__main__":
  print("\033[H\033[2J")
  while True:
    try:n=int(input("\nEnter number > "))
    except ValueError:continue
    if n>2147483647 or n<-2147483648:print("Out of 32-bit range");continue
    i(n,0,True)
