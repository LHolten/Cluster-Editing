import json

# fmt: off
edges = [10, 20, 20, 30, 30, 30, 40, 40, 50, 50, 50, 60, 60, 70, 70, 70, 80, 80, 90, 90, 90, 100, 100, 100]
# fmt: on

# for i in [1, 3, 5, 7, 9, 11, 13, 15, 21, 23, 25, 31, 35, 41, 47]:
#     edge = edges[(i - 1) // 2]

#     f = open(f"target/criterion/exact/{i}/incremental/estimates.json")
#     data1 = json.load(f)
#     f.close()

#     f = open(f"target/criterion/exact/{i}/none/estimates.json")
#     data2 = json.load(f)
#     f.close()

#     value = data1["mean"]["point_estimate"] / data2["mean"]["point_estimate"]
#     string = f"({edge}, {value})"
#     print(string, end=" ")


for i in [1, 3, 5, 7, 9, 11, 13, 15, 21, 23, 25, 31, 35, 41, 47]:
    f = open(f"target/criterion/exact/{i}/incremental alt-cost/estimates.json")
    data1 = json.load(f)
    f.close()

    value = data1["mean"]["point_estimate"] / 1000
    string = f"({i}, {value})"
    print(string, end=" ")
