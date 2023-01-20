time = input("Enter time: ")

# parse the last character of the time string, d for days, h for hours, m for minutes, M for months, s for seconds, w for weeks, y for years
time_unit = time[-1]

# parse the time string to an integer
time = int(time[:-1])

# convert the time to seconds
if time_unit == "d":
    time = time * 86400
elif time_unit == "h":
    time = time * 3600
elif time_unit == "m":
    time = time * 60
elif time_unit == "M":
    time = time * 2592000
elif time_unit == "s":
    time = time
elif time_unit == "w":
    time = time * 604800
elif time_unit == "y":
    time = time * 31536000
else:
    print("Invalid time unit")
#END

hashrate = input("Enter hashrate: ")
# If N/A assume 4GH/s
if hashrate == "allti":
    hashrate = 1.34e+15
elif hashrate == "":
    hashrate = 4000000000
elif hashrate == "btc":
    hashrate = 254_212_114_979_426_800_000
else:
    hashrate = int(hashrate)
#END

# Calculate the % chance of guessing a correct aes key in the given time
chance = (1 / 2**128) * (time * hashrate)

# Print the chance
print("Chance of guessing a correct aes key in the given time:\n1 in " + '{:f}'.format(1 / chance))

1.34e+15
