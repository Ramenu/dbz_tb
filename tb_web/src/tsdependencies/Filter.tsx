import { isValidRarity, isValidType } from "./Checker.js"
import { invalidRarityMessage, invalidTypeMessage } from "./Error.js"
import Unit from "./Unit.js";

export enum FilterAction
{
    Rarity,
    Type
}

export const filterUnits = (units : Unit[], filter : string, action : FilterAction) : Unit[] => {
    if (action == FilterAction.Rarity)
    {
        if (!isValidRarity(filter))
            throw invalidRarityMessage(filter);
    }
    else
    {
        if (!isValidType(filter))
            throw invalidTypeMessage(filter);
    }
    
    let filteredUnits : Unit[] = [];
    units.forEach(unit => {
        if (action == FilterAction.Rarity)
        {
            if (unit.rarity == filter)
                filteredUnits.push(unit);
        }
        else
        {
            if (unit.type == filter)
                filteredUnits.push(unit);
        }
    });
    return filteredUnits;
}