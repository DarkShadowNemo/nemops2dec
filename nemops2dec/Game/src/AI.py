from struct import unpack
import glob

coordinate1 = []
coordinate2 = []

def ReadAI(f):
    Version = unpack("<I", f.read(4))[0]
    AICount = unpack("<I", f.read(4))[0]
    for i in range(AICount):
        if AICount:
            
            AIName = f.read(16)
            AICoordinateCount = unpack("<I", f.read(4))[0]
            if AICoordinateCount == 1:
                AIScale = unpack("<f", f.read(4))[0]
                AIPosX = unpack("<f", f.read(4))[0]
                AIPosY = unpack("<f", f.read(4))[0]
                AIPosZ = unpack("<f", f.read(4))[0]
                coordinate1.append([AIScale, AIPosX, AIPosY, AIPosZ])
            elif AICoordinateCount == 2:
                AIScale = unpack("<f", f.read(4))[0]
                AIPosX = unpack("<f", f.read(4))[0]
                AIPosY = unpack("<f", f.read(4))[0]
                AIPosZ = unpack("<f", f.read(4))[0]
                AIRotX = unpack("<f", f.read(4))[0]
                AIRotY = unpack("<f", f.read(4))[0]
                AIRotZ = unpack("<f", f.read(4))[0]
                coordinate2.append([AIScale, AIPosX, AIPosY, AIPosZ, AIRotX, AIRotY, AIRotZ])
            else:
                raise Exception("must contain 1 or 2 not more than 2")
        elif AICount == 0:
            pass
