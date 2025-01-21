from struct import unpack
import glob

def ReadLGT(f):
    Version = unpack("<I", f.read(4))[0]
    lightCount = unpack("<I", f.read(4))[0]
    for i in range(lightCount):
        Red = unpack("B", f.read(1))[0]
        Green = unpack("B", f.read(1))[0]
        Blue = unpack("B", f.read(1))[0]
        Alpha = unpack("B", f.read(1))[0]
        unknown1 = unpack("<I", f.read(4))[0]
        unknown2 = unpack("<I", f.read(4))[0]
        Scale = unpack("<f", f.read(4))[0]
        PositionX = unpack("<f", f.read(4))[0]
        PositionY = unpack("<f", f.read(4))[0]
        PositionZ = unpack("<f", f.read(4))[0]
        
