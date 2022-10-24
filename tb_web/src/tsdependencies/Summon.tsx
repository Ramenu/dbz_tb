
import { Banner } from "./Banner";
import { MULTI_SUMMON_ROLL_COUNT } from "./Constants";
import { R_UNITS, SR_UNITS, SSR_UNITS, getUnit} from "./Database";
import { FilterAction, filterUnits } from "./Filter";
import { randint } from "./Random";
import Unit from "./Unit";

const SR_ROLL_MIN : number = 30;
const SR_ROLL_MAX : number = 89;

const SSR_ROLL_MIN : number = 90;
const SSR_ROLL_MAX : number = 100;

const gotSSR = (roll : number) : boolean => {
    return roll >= SSR_ROLL_MIN && roll <= SSR_ROLL_MAX;
}

const gotSR = (roll : number) : boolean => {
    return roll >= SR_ROLL_MIN && roll <= SR_ROLL_MAX;
}

const gotR = (roll : number) : boolean => {
    return roll >= 0 && roll <= SR_ROLL_MIN - 1;
}

const gotFeaturedUnit = (roll : number) : boolean => {
    return ((roll >= Math.round((SR_ROLL_MAX - SR_ROLL_MIN) / 2) + SR_ROLL_MIN) && roll <= SR_ROLL_MAX) ||
           ((roll >= Math.round((SSR_ROLL_MAX - SSR_ROLL_MIN) / 2) + SSR_ROLL_MIN) && roll <= SSR_ROLL_MAX);
}

const isFeaturedUnit = (unit : Unit, featuredUnits : Unit[]) : boolean => {
    return featuredUnits.includes(unit);
}

export const performSingleSummon = (banner : Banner) : Unit =>
{
    const roll : number = randint(0, 101);

    if (gotR(roll))
        return getUnit(R_UNITS[randint(0, R_UNITS.length)]);
    if (gotFeaturedUnit(roll))
    {
        let featured : Unit[];

        if (banner.featuredUnits.some((unit) => unit.rarity === "SSR"))
            featured = gotSR(roll) ? 
                       filterUnits(banner.featuredUnits, "SR", FilterAction.Rarity) : 
                       filterUnits(banner.featuredUnits, "SSR", FilterAction.Rarity);
        else
            featured = filterUnits(banner.featuredUnits, "SR", FilterAction.Rarity);

        if (featured.length !== 0)
            return featured[randint(0, featured.length)];
    }

    // Extremely-low statistical chance for this loop to go on long at all
    // still... i feel like this deserves a better solution.

    // We need to make sure the non-featured unit pulled is not a featured one
    while (true)
    {
        const pulledUnitURL : string = gotSR(roll) ? SR_UNITS[randint(0, SR_UNITS.length)] : SSR_UNITS[randint(0, SSR_UNITS.length)];
        const pulledUnit : Unit = getUnit(pulledUnitURL);
        if (!isFeaturedUnit(pulledUnit, banner.featuredUnits))
            return pulledUnit;
    }
}

export const performMultiSummon = (banner : Banner) : Unit[] =>
{
    let pulledUnits : Unit[] = [];

    for (let i = 0; i < MULTI_SUMMON_ROLL_COUNT; ++i)
        pulledUnits.push(performSingleSummon(banner));
    return pulledUnits;
}