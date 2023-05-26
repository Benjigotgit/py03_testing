import json
import rython 
import os


def run_rython():
    with open("./python/sample.json") as json_file:
        geojson = json.load(json_file)
        rython.get_geojson_area(geojson, "what")
        # print(geojson)
        
run_rython()