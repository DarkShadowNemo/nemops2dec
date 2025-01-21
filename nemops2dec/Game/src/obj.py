from struct import unpack

def ReadOBJ(f):
    f.seek(0)
    FileSize = f.read()
    f.seek(0)
    flag1 = unpack("B", f.read(1))[0] # unknown Flag
    for i in range(len(FileSize)-1):
        unk0001 = unpack("B", f.read(1))[0]
    
