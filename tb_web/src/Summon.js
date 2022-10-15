
import { ALL_BANNERS, Banner, BANNER_TYPE } from "./Banner.js";
import { R_UNITS, SR_UNITS, SSR_UNITS, ALL_UNITS } from "./Database.js";
import { filterUnitsByRarity } from "./Filter.js";
import { randint } from "./Random.js";

const SR_ROLL_MIN = 30;
const SR_ROLL_MAX = 89;

const SSR_ROLL_MIN = 90;
const SSR_ROLL_MAX = 100;

const gotSSR = (roll) => {
    return roll >= SSR_ROLL_MIN && roll <= SSR_ROLL_MAX;
}

const gotSR = (roll) => {
    return roll >= SR_ROLL_MIN && roll <= SSR_ROLL_MAX;
}

const gotR = (roll) => {
    return roll >= 0 && roll <= SR_ROLL_MIN - 1;
}

const gotFeaturedUnit = (roll) => {
    return ((roll >= (SR_ROLL_MAX - SR_ROLL_MIN) / 2 + SR_ROLL_MIN) && roll <= SR_ROLL_MAX) ||
           ((roll >= (SSR_ROLL_MAX - SSR_ROLL_MIN) / 2 + SSR_ROLL_MIN) && roll <= SSR_ROLL_MAX);
}

/**
 * 
 * @param {Unit} unit 
 * @param {Unit[]} featuredUnits 
 * @returns Boolean
 */
const isFeaturedUnit = (unit, featuredUnits) => {
    featuredUnits.forEach(u => {
        if (u === unit)
            return true;
    });
    return false;
}

/**
 * 
 * @param {Banner} banner 
 * @returns Unit
 */
const performRareSummon = (banner) =>
{
    const roll = randint(0, 101);

    if (gotR(roll))
        return ALL_UNITS()[R_UNITS[randint(0, R_UNITS.length)]];
    if (gotFeaturedUnit(roll))
    {
        const featured = gotSR(roll) ? filterUnitsByRarity(banner.featuredUnits, "SR") : filterUnitsByRarity(banner.featuredUnits, "SSR");
        if (featured.length !== 0)
            return ALL_UNITS()[featured[randint(0, featured.length)]];
    }

    // Extremely-low statistical chance for this loop to go on long at all
    // still... i feel like this deserves a better solution.

    // We need to make sure the non-featured unit pulled is not a featured one
    while (true)
    {
        const pulledUnitURL = gotSR(roll) ? SR_UNITS[randint(0, SR_UNITS.length)] : SSR_UNITS[randint(0, SSR_UNITS.length)];
        if (!isFeaturedUnit(pulledUnitURL, banner.featuredUnits))
            return ALL_UNITS()[pulledUnitURL];
    }
}

/**
 * 
 * @param {Banner} banner 
 * @returns Banner[]
 */
const performMultiSummon = (banner) =>
{
    const numOfRolls = 10;
    let pulledUnits = [];

    for (let i = 0; i < numOfRolls; ++i)
        pulledUnits.push(performRareSummon(banner));
    return pulledUnits;
}


const banner = ALL_BANNERS[0];
const unitsPulled = performMultiSummon(banner);

unitsPulled.forEach(u => {
    console.log(`Name: ${u.name}\nRarity: ${u.rarity}\n`);
})