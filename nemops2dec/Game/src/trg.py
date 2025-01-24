from struct import unpack
import glob

def ReadTRG(f):
    Version = unpack("<I", f.read(4))[0]
    TRGCount = unpack("<I", f.read(4))[0]
    for i in range(TRGCount):
        TRGName = f.read(16)
        id1 = unpack("<I", f.read(4))[0]
        zero1 = unpack("<I", f.read(4))[0]
        posx = unpack("<f", f.read(4))[0]
        posy = unpack("<f", f.read(4))[0]
        posz = unpack("<f", f.read(4))[0]
        radius_ = unpack("<f", f.read(4))[0]
        padding1 = unpack("<I", f.read(4))[0]
        padding2 = unpack("<I", f.read(4))[0]
        padding3 = unpack("<I", f.read(4))[0]
        padding4 = unpack("<I", f.read(4))[0]
        padding5 = unpack("<I", f.read(4))[0]
        type1 = unpack("<I", f.read(4))[0]
        type2 = unpack("<I", f.read(4))[0]
        zero2 = unpack("<I", f.read(4))[0]
        zero3 = unpack("<I", f.read(4))[0]
        scalex = unpack("<f", f.read(4))[0]
        scaley = unpack("<f", f.read(4))[0]
        scalez = unpack("<f", f.read(4))[0]
        unknown01 = unpack("<I", f.read(4))[0]
        r = unpack("B", f.read(1))[0] / 255
        g = unpack("B", f.read(1))[0] / 255
        b = unpack("B", f.read(1))[0] / 255
        a = unpack("B", f.read(1))[0] / 255
        unknown = unpack("<I", f.read(4))[0]
    if unknown == 32:
        f.read()
    elif unknown != 32:
        pass
    
