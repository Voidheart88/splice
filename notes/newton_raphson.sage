# Spice Netlist:
# R1 1 2 10
# D1 2 0
# V1 0 1 10
# 
# Mod Conductance Matrix should have 3x3 shape

r = 10.0
IS = 1e-12

KB = 1.380649e-23
ELE_CHRG = 1.602176634e-19
TEMP = 293.15
U1 = 10.0

#IS,KB,ELE_CHRG,TEMP = var("IS,KB,ELE_CHRG,TEMP")
UT = KB * TEMP / ELE_CHRG

# Diode with second Node connected to ground
gd(u2) = IS*(e^(u2/UT)-1.0)/u2
gr(r) = 1.0/r

gd = var("gd")
gr = var("gr")
b = Matrix([[0],[0],[U1]])

cond = Matrix([
    [gr,-gr,1],
    [-gr,gr+gd,0],
    [1,0,0]
])

cond_nl = Matrix([
    [0, 0, 0],
    [0,gd, 0],
    [0, 0, 0]
])

condi = cond.inverse()

x0 = Matrix([[10],[0.7],[1]])

x1 = condi*(-cond_nl*x0+b)