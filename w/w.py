import sys, requests, time, json, folium

IP_API_URL = "http://ip-api.com/json/"
user = set()

m = folium.Map()
with open(sys.argv[1]) as w:
    for line in w:
        tokens = line.split()
        if tokens[0] not in user: # handle multiple terminals by the same user.
            user.add(tokens[0])
            r = json.loads(requests.get(IP_API_URL + tokens[2]).content)
            time.sleep(5) # avoid throttling
            if "lat" in r and "lon" in r:
                folium.Marker([r["lat"], r["lon"]], tooltip=tokens[2], popup=tokens[0]).add_to(m)

m.save("index.html")
