import sys

with open(sys.argv[1]) as input:
    num_paces = int(input.readline())
    output = open(sys.argv[1].strip(".in") + ".out", 'w')
    for i in range(0, num_paces):
        trip = input.readline().split(" ")
        trip_len = 0
        trip_time = 0
        for j in trip:
            movement = int(j)
            if movement == 0:
                trip_time += 1
            else:
                trip_time += abs(movement)
                if movement > 0:
                    trip_len += 1
                else:
                    trip_len -= 1
        output.write(str(trip_len) + " " + str(trip_time) + '\n')
    output.close()        
