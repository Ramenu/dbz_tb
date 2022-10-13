
/** need to work on fixing this
 * @param {String} url The HTTP/HTTPS URL you want to check is valid or not
 * @returns Boolean
 */
export const isValidURL = (url) =>
{
    let urlObj;
    try {
        urlObj = new URL(url);
    } catch (_) {
        return false;
    }
    return urlObj.protocol == "http:" || urlObj.protocol == "https:";
}

/**
 * 
 * @param {String} type 
 * @returns Boolean
 */
export const isValidType = (type) =>
{
    return type == "Super STR" || type == "Extreme STR" ||
           type == "Super PHY" || type == "Extreme PHY" ||
           type == "Super AGL" || type == "Extreme AGL" ||
           type == "Super TEQ" || type == "Extreme TEQ" ||
           type == "Super INT" || type == "Extreme INT"
}

/**
 * 
 * @param {String} rarity 
 * @returns Boolean
 */
export const isValidRarity = (rarity) =>
{
    return rarity == "N" ||
           rarity == "R" ||
           rarity == "SR" ||
           rarity == "SSR" ||
           rarity == "UR" ||
           rarity == "LR";
}