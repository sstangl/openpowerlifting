"""Module and script to compute DOTS scores """

import sys

def dots_coefficient(sex, bw):
    if sex == "M":
        adj_bw = min(max(40, bw), 210)
        a = -0.0000010930
        b = 0.0007391293
        c = -0.1918759221
        d = 24.0900756
        e = -307.75076
    elif sex == "F":
        adj_bw = min(max(40, bw), 150)
        a = -0.0000010706
        b = 0.0005158568
        c = -0.1126655495
        d = 13.6175032
        e = -57.96288
    else:
        raise ValueError(f"Unknown sex {sex}")
    return 500 / ((a * adj_bw ** 4) + (b * adj_bw ** 3) + (c * adj_bw ** 2) + (d * adj_bw) + e)

def dots_score(sex, bw, total):
    coeff = dots_coefficient(sex, bw)
    return total * coeff

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print(f"Usage: {sys.argv[0]} sex bw total", file=sys.stderr)
        sys.exit(1)
    sex = sys.argv[1]
    bw = float(sys.argv[2])
    total = float(sys.argv[3])
    score = dots_score(sex, bw, total)
    print(score)