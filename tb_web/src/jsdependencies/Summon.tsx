
import { ALL_BANNERS, Banner } from "./Banner";
import { R_UNITS, SR_UNITS, SSR_UNITS, getUnit} from "./Database";
import { filterUnits } from "./Filter";
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
    return ((roll >= (SR_ROLL_MAX - SR_ROLL_MIN) / 2 + SR_ROLL_MIN) && roll <= SR_ROLL_MAX) ||
           ((roll >= (SSR_ROLL_MAX - SSR_ROLL_MIN) / 2 + SSR_ROLL_MIN) && roll <= SSR_ROLL_MAX);
}

const isFeaturedUnit = (unit : Unit, featuredUnits : Unit[]) : boolean => {
    return featuredUnits.includes(unit);
}

const performRareSummon = (banner : Banner) : Unit =>
{
    const roll : number = randint(0, 101);

    if (gotR(roll))
        return getUnit(R_UNITS[randint(0, R_UNITS.length)]);
    if (gotFeaturedUnit(roll))
    {
        const featured : Unit[] = gotSR(roll) ? filterUnits(banner.featuredUnits, "SR", 3) : filterUnits(banner.featuredUnits, "SSR", 5);
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

const performMultiSummon = (banner : Banner) : Unit[] =>
{
    const numOfRolls : number = 10;
    let pulledUnits : Unit[] = [];

    for (let i = 0; i < numOfRolls; ++i)
        pulledUnits.push(performRareSummon(banner));
    return pulledUnits;
}


const banner : Banner = ALL_BANNERS[0];
const unitsPulled : Unit[] = performMultiSummon(banner);

unitsPulled.forEach(u => {
    console.log(`Name: ${u.name}\nRarity: ${u.rarity}\n`);
});