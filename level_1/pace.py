import sys

with open(sys.argv[1]) as input:
    num_paces = int(input.readline())
    output = open(sys.argv[1].strip(".in") + ".out", 'w')
    for i in range(0, num_paces):
        trip = input.readline().split(" ")
        trip_len = 0
        for j in trip:
            trip_len += int(j)
        output.write(str(trip_len) + '\n')
    output.close()        
