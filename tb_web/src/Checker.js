
/**
 * @param {String} url The HTTP/HTTPS URL you want to check is valid or not
 * @returns {Boolean} 
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
 * @returns {Boolean}
 */
export const isValidType = (type) =>
{
    return type == "STR" ||
           type == "PHY" ||
           type == "AGL" ||
           type == "TEQ" ||
           type == "INT";
}

/**
 * 
 * @param {String} rarity 
 * @returns {Boolean}
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