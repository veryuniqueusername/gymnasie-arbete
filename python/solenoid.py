import math

mu_0: float = 0.0000012566
N: float = 500
l: float = 0.05
U_0: float = 25
C: float = 0.11
R: float = 0.01
M: float = 1000000
V: float = 0.000007
m: float = M * V
mass: float = 0.05

z: float = -l/2
v: float = 0
dt: float = 0.001
t = 0

while t < 100:
    U: float = U_0 * math.exp(-t / (R*C))
    I: float = U / R
    zpos: float = math.pow(z+l/2, 2)
    zneg: float = math.pow(z-l/2, 2)
    R2: float = R*R
    B: float = mu_0*N*I/2 * (
        (z+l/2) / (l * math.pow(R2 + zpos, 1/2)) -
        (z-l/2) / (l * math.pow(R2 + zneg, 1/2))
    )
    B_delta: float = mu_0*N*I/2 * (
        zneg / (l * math.pow(R2 + zneg, 3/2)) -
        1 / (l * math.pow(R2 + zneg, 1/2)) +
        1 / (l * math.pow(R2 + zpos, 1/2)) -
        zpos / (l * math.pow(R2 + zpos, 3/2))
    )
    F: float = m * B_delta
    a: float = F / mass
    v += a * dt
    z += v * dt

    print(f"time: {t}, pos: {z}, velo: {v}, accel: {a}, I: {I}")
    t += dt
