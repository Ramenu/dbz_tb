import { isValidRarity, isValidType } from "./Checker"
import { invalidRarityMessage, invalidTypeMessage } from "./Error"

/**
 * 
 * @param {String[]} units 
 * @param {String} rarityFilter 
 * @returns String[]
 */
export const filterUnitsByRarity = (units, rarityFilter) => {
    if (!isValidRarity(rarityFilter))
        throw invalidRarityMessage(rarityFilter);
    
    let filteredUnits = [];
    units.forEach(unit => {
        if (unit.rarity == rarityFilter)
            filteredUnits.push(unit);
    });
    return units;
}

/**
 * 
 * @param {String[]} units 
 * @param {String} typeFilter 
 * @returns String[]
 */
export const filterUnitsByType = (units, typeFilter) => {
    if (!isValidType(typeFilter))
        throw invalidTypeMessage(typeFilter);

    let filteredUnits = [];
    units.forEach(unit => {
        if (unit.type == typeFilter)
            filteredUnits.push(unit);
    });
    return units;
}