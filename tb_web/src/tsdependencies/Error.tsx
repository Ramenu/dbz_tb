
export const invalidURLMessage = (s : string) : Error => new Error(`Invalid URL: ${s}`);

export const invalidRarityMessage = (s : string) : Error => new Error(`Invalid rarity: ${s}\nPossible rarities are: [N, R, SR, SSR, UR, LR]`);

export const invalidTypeMessage = (s : string) : Error => new Error(`Invalid type: ${s}\nPossible types are: "Super|Extreme [STR, AGL, TEQ, PHY, INT]"`);
