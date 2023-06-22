import sys, requests, time, json, folium

IP_API_URL = "http://ip-api.com/json/"
users = set()

m = folium.Map()
with open(sys.argv[1]) as w:
    for line in w:
        tokens = line.split()
        user = tokens[0]
        ip = tokens[2]
        if user not in users: # handle multiple terminals by the same user.
            users.add(user)
            r = json.loads(requests.get(IP_API_URL + ip).content)
            time.sleep(5) # avoid throttling
            if "lat" in r and "lon" in r:
                folium.Marker([r["lat"], r["lon"]], tooltip=ip, popup=user).add_to(m)

m.save("index.html")
