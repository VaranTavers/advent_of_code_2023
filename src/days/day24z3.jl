using CSV
using DataFrames
using Folds
using Z3

# This in conjunction with day24.jl

meteors = CSV.read("/var/home/tasnadiz/2024/AdventOfCode/inputs/input24jl", DataFrame; header=false)


a = Float64.(Vector(meteors[1, :]))
b = Float64.(Vector(meteors[2, :]))
c = Float64.(Vector(meteors[3, :]))
d = Float64.(Vector(meteors[4, :]))
e = Float64.(Vector(meteors[5, :]))
f = Float64.(Vector(meteors[6, :]))
g = Float64.(Vector(meteors[7, :]))
h = Float64.(Vector(meteors[8, :]))

ctx = Context()
x = real_const(ctx, "x")
y = real_const(ctx, "y")
z = real_const(ctx, "z")

xs = real_const(ctx, "xs")
ys = real_const(ctx, "ys")
zs = real_const(ctx, "zs")

t0 = real_const(ctx, "t0")
t1 = real_const(ctx, "t1")
t2 = real_const(ctx, "t2")
t3 = real_const(ctx, "t3")
t4 = real_const(ctx, "t4")
t5 = real_const(ctx, "t5")
t6 = real_const(ctx, "t6")
t7 = real_const(ctx, "t7")
t8 = real_const(ctx, "t8")
t9 = real_const(ctx, "t9")
t10 = real_const(ctx, "t10")
t11 = real_const(ctx, "t11")
t12 = real_const(ctx, "t12")
t13 = real_const(ctx, "t13")
t14 = real_const(ctx, "t14")
t15 = real_const(ctx, "t15")


x1 = real_const(ctx, "x1")
y1 = real_const(ctx, "y1")
z1 = real_const(ctx, "z1")
x1s = real_const(ctx, "x1s")
y1s = real_const(ctx, "y1s")
z1s = real_const(ctx, "z1s")
x2 = real_const(ctx, "x2")
y2 = real_const(ctx, "y2")
z2 = real_const(ctx, "z2")
x2s = real_const(ctx, "x2s")
y2s = real_const(ctx, "y2s")
z2s = real_const(ctx, "z2s")
x3 = real_const(ctx, "x3")
y3 = real_const(ctx, "y3")
z3 = real_const(ctx, "z3")
x3s = real_const(ctx, "x3s")
y3s = real_const(ctx, "y3s")
z3s = real_const(ctx, "z3s")
x4 = real_const(ctx, "x4")
y4 = real_const(ctx, "y4")
z4 = real_const(ctx, "z4")
x4s = real_const(ctx, "x4s")
y4s = real_const(ctx, "y4s")
z4s = real_const(ctx, "z4s")
x5 = real_const(ctx, "x5")
y5 = real_const(ctx, "y5")
z5 = real_const(ctx, "z5")
x5s = real_const(ctx, "x5s")
y5s = real_const(ctx, "y5s")
z5s = real_const(ctx, "z5s")
x6 = real_const(ctx, "x6")
y6 = real_const(ctx, "y6")
z6 = real_const(ctx, "z6")
x6s = real_const(ctx, "x6s")
y6s = real_const(ctx, "y6s")
z6s = real_const(ctx, "z6s")
x7 = real_const(ctx, "x7")
y7 = real_const(ctx, "y7")
z7 = real_const(ctx, "z7")
x7s = real_const(ctx, "x7s")
y7s = real_const(ctx, "y7s")
z7s = real_const(ctx, "z7s")
x8 = real_const(ctx, "x8")
y8 = real_const(ctx, "y8")
z8 = real_const(ctx, "z8")
x8s = real_const(ctx, "x8s")
y8s = real_const(ctx, "y8s")
z8s = real_const(ctx, "z8s")


s = Solver(ctx, "QF_NRA")

add(s, x1 == a[4])
add(s, y1 == a[5])
add(s, z1 == a[6])
add(s, x1s == a[1])
add(s, y1s == a[2])
add(s, z1s == a[3])
add(s, x2 == b[4])
add(s, y2 == b[5])
add(s, z2 == b[6])
add(s, x2s == b[1])
add(s, y2s == b[2])
add(s, z2s == b[3])
add(s, x3 == c[4])
add(s, y3 == c[5])
add(s, z3 == c[6])
add(s, x3s == c[1])
add(s, y3s == c[2])
add(s, z3s == c[3])
add(s, x4 == d[4])
add(s, y4 == d[5])
add(s, z4 == d[6])
add(s, x4s == d[1])
add(s, y4s == d[2])
add(s, z4s == d[3])
add(s, x5 == e[4])
add(s, y5 == e[5])
add(s, z5 == e[6])
add(s, x5s == e[1])
add(s, y5s == e[2])
add(s, z5s == e[3])
add(s, x6 == f[4])
add(s, y6 == f[5])
add(s, z6 == f[6])
add(s, x6s == f[1])
add(s, y6s == f[2])
add(s, z6s == f[3])
add(s, x7 == g[4])
add(s, y7 == g[5])
add(s, z7 == g[6])
add(s, x7s == g[1])
add(s, y7s == g[2])
add(s, z7s == g[3])
add(s, x8 == h[4])
add(s, y8 == h[5])
add(s, z8 == h[6])
add(s, x8s == h[1])
add(s, y8s == h[2])
add(s, z8s == h[3])

add(s, x == -11)
add(s, y == 330)
add(s, z == 91)

add(s, t0 == t1)
add(s, t2 == t3)
add(s, t4 == t5)
add(s, t6 == t7)
add(s, t8 == t9)
add(s, t10 == t11)
add(s, t12 == t13)
add(s, t14 == t15)

add(s, t0 > 0)
add(s, t2 > 0)
add(s, t4 > 0)
add(s, t6 > 0)
add(s, t8 > 0)
add(s, t10 > 0)
add(s, t12 > 0)
add(s, t14 > 0)

add(s, t1 > 0)
add(s, t3 > 0)
add(s, t5 > 0)
add(s, t7 > 0)
add(s, t9 > 0)
add(s, t11 > 0)
add(s, t13 > 0)
add(s, t15 > 0)


add(s, x1 * t0 - x * t1 + x1s - xs == 0)
add(s, y1 * t0 - y * t1 + y1s - ys == 0)
add(s, z1 * t0 - z * t1 + z1s - zs == 0)

add(s, x2 * t2 - x * t3 + x2s - xs == 0)
add(s, y2 * t2 - y * t3 + y2s - ys == 0)
add(s, z2 * t2 - z * t3 + z2s - zs == 0)

add(s, x3 * t4 - x * t5 + x3s - xs == 0)
add(s, y3 * t4 - y * t5 + y3s - ys == 0)
add(s, z3 * t4 - z * t5 + z3s - zs == 0)

add(s, x4 * t6 - x * t7 + x4s - xs == 0)
add(s, y4 * t6 - y * t7 + y4s - ys == 0)
add(s, z4 * t6 - z * t7 + z4s - zs == 0)

add(s, x5 * t8 - x * t9 + x5s - xs == 0)
add(s, y5 * t8 - y * t9 + y5s - ys == 0)
add(s, z5 * t8 - z * t9 + z5s - zs == 0)

add(s, x6 * t10 - x * t11 + x6s - xs == 0)
add(s, y6 * t10 - y * t11 + y6s - ys == 0)
add(s, z6 * t10 - z * t11 + z6s - zs == 0)

add(s, x7 * t12 - x * t13 + x7s - xs == 0)
add(s, y7 * t12 - y * t13 + y7s - ys == 0)
add(s, z7 * t12 - z * t13 + z7s - zs == 0)

add(s, x8 * t14 - x * t15 + x8s - xs == 0)
add(s, y8 * t14 - y * t15 + y8s - ys == 0)
add(s, z8 * t14 - z * t15 + z8s - zs == 0)

res = check(s)
@assert res == Z3.sat

m = get_model(s)

for (k, v) in consts(m)
    println("$k = $v")
end

println("$(consts[m]["xs"])")
println("$(consts[m]["ys"])")
println("$(consts[m]["zs"])")