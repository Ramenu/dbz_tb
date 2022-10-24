import { isValidRarity, isValidType } from "./Checker"
import { invalidRarityMessage, invalidTypeMessage } from "./Error"
import Unit from "./Unit";

export enum FilterAction
{
    Rarity = "RARITY",
    Type = "TYPE"
}

export const filterUnits = (units : Unit[], filter : string, action : FilterAction) : Unit[] => {
    let filteredUnits : Unit[] = [];
    if (action === FilterAction.Rarity)
    {
        if (!isValidRarity(filter))
            throw invalidRarityMessage(filter);
        units.forEach((unit) => {
            if (unit.rarity === filter)
                filteredUnits.push(unit);
        });
    }
    else
    {
        if (!isValidType(filter))
            throw invalidTypeMessage(filter);
        units.forEach((unit) => {
            if (unit.type === filter)
                filteredUnits.push(unit);
        });
    }
    
    return filteredUnits;
}