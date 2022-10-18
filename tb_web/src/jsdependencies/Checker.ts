
export const isValidURL = (url: string) : boolean =>
{
    let urlObj : URL;
    try {
        urlObj = new URL(url);
    } catch (_) {
        return false;
    }
    return urlObj.protocol === "http:" || urlObj.protocol === "https:";
}

export const isValidType = (type : string) : boolean =>
{
    return type === "Super STR" || type === "Extreme STR" ||
           type === "Super PHY" || type === "Extreme PHY" ||
           type === "Super AGL" || type === "Extreme AGL" ||
           type === "Super TEQ" || type === "Extreme TEQ" ||
           type === "Super INT" || type === "Extreme INT";
}

export const isValidRarity = (rarity : string) : boolean =>
{
    return rarity === "N" ||
           rarity === "R" ||
           rarity === "SR" ||
           rarity === "SSR" ||
           rarity === "UR" ||
           rarity === "LR";
}