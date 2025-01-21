from struct import unpack
import glob
from io import BytesIO as bio

def truncate_cstr(s: bytes) -> bytes:
    index = s.find(0)
    if index == -1: return s
    return s[:index]
def fetch_cstr(f: 'filelike') -> bytearray:
    build = bytearray()
    while 1:
        strbyte = f.read(1)
        if strbyte == b'\0' or not strbyte: break
        build += strbyte
    return build

def ReadVIS(f):
    ea = -1
    eb = 0
    Vertices = []
    edges = []
    Red = unpack("B", f.read(1))[0]
    Green = unpack("B", f.read(1))[0]
    Blue = unpack("B", f.read(1))[0]
    Alpha = unpack("B", f.read(1))[0]
    TotalVertexCount = unpack("<I", f.read(4))[0]
    for i in range(TotalVertexCount):
        VertexCount = unpack("<I", f.read(4))[0]
        for j in range(VertexCount):
            vx = unpack("<f", f.read(4))[0]
            vy = unpack("<f", f.read(4))[0]
            vz = unpack("<f", f.read(4))[0]
            Vertices.append([vx,vz,vy])
        for e in range(VertexCount-1):
            ea+=1
            eb+=1
            edges.append([ea+i,eb+i])
            
