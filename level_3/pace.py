import sys

with open(sys.argv[1]) as input:
    num_paces = int(input.readline())
    output = open(sys.argv[1].strip(".in") + ".out", 'w')
    for i in range(0, num_paces):
        trip = input.readline().split(" ")
        distance = int(trip[0])
        target_time = int(trip[1])
        if distance > 0:
            direction = 1
        else:
            direction = -1
        sequence = []

        speed = direction * 6
        time_to_slowdown = 0
        sequence.append(0)
        if abs(distance) == 1:
            sequence.append(direction * 5)
            sequence.append(0)
            output.write(str(sequence).replace("[", "").replace("]", "").replace(",", "") + '\n')
            continue
        while abs(distance) > time_to_slowdown:
            if time_to_slowdown + 1 == abs(distance):
                sequence.append(speed)
                break
            if abs(speed) != 1 and abs(distance) - 1 >= 2:
                speed -= direction * 1
            target_time -= abs(speed)
            distance -= direction * 1
            time_to_slowdown = abs(5 - abs(speed))
            sequence.append(speed)
        while abs(speed) != 5:
            speed += direction * 1
            sequence.append(speed)
        sequence.append(0)
        output.write(str(sequence).replace("[", "").replace("]", "").replace(",", "") + '\n')
    output.close()
