
/**
 * 
 * @param {String} s 
 * @returns 
 */
export const invalidURLMessage = (s) => {
    return `Invalid URL: ${s}`;
}

export const invalidRarityMessage = (s) => {
    return `Invalid rarity: ${s}\nPossible rarities are: [N, R, SR, SSR, UR, LR]`;
}

export const invalidTypeMessage = (s) => {
    return `Invalid type: ${s}\nPossible types are: [STR, AGL, TEQ, PHY, INT]`;
}