#!/bin/python

import requests
import re

ROOT_URL = "https://dbz-dokkanbattle.fandom.com"
URL = "https://dbz-dokkanbattle.fandom.com/wiki/Timeline_of_Events_(Japan)#January_2015"

RARITY_REG = "<td>.*?(?:<center>|<span>)<a href=\"/wiki/Category:[NRSUL]?[SR]?R?\" title=\"Category:([NRSUL]?[SR]?R?)\">"

UNIT_LIST_URL_REG = "(?s)(Cards available since release:</b><br />\s</p>.*?</p>)"
UNIT_URL_REG = "(?s)<a href=\"(.*?)\" title=\".*?\".*?/>"

resp = requests.get(URL).text
cards = re.search(UNIT_LIST_URL_REG, resp)[0]

relativeURLS = re.findall(UNIT_URL_REG, cards)

nUnits = []
rUnits = []
srUnits = []
ssrUnits = []


if relativeURLS:
    for relativeURL in relativeURLS:
        fullURL = ROOT_URL + relativeURL
        unitPageResp = requests.get(fullURL).text
        rarity = re.search(RARITY_REG, unitPageResp)
        if rarity:
            rarity = rarity.group(1)
            urlWithQuotes = "\"" + fullURL + "\""
            if rarity == "N":
                nUnits.append(urlWithQuotes)
            elif rarity == "R":
                rUnits.append(urlWithQuotes)
            elif rarity == "SR":
                srUnits.append(urlWithQuotes)
            elif rarity == "SSR":
                ssrUnits.append(urlWithQuotes)

NUM_SPACES = "   "

def printUnits(arr):
    size = len(arr)
    for i in range(size):
        if i == size - 1:
            print(f"{NUM_SPACES}{arr[i]}")
        else:
            print(f"{NUM_SPACES}{arr[i]},")


print("export const N_UNITS = [")
printUnits(nUnits)
print("];\n\nexport const R_UNITS = [")
printUnits(rUnits)
print("];\n\nexport const SR_UNITS = [")
printUnits(srUnits)
print("];\n\nexport const SSR_UNITS = [")
printUnits(ssrUnits)
print("];")