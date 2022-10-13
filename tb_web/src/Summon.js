
import { Banner, BANNER_TYPE } from "./Banner";
import { R_UNITS, SR_UNITS, SSR_UNITS } from "./Database";
import { filterUnitsByRarity } from "./Filter";

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
    return roll >= 0 && roll <= SR_ROLL_MAX - 1;
}

const gotFeaturedUnit = (roll) => {
    return ((roll >= (SR_ROLL_MAX - SR_ROLL_MIN) / 2 + SR_ROLL_MIN) && roll <= SR_ROLL_MAX) ||
           ((roll >= (SSR_ROLL_MAX - SSR_ROLL_MIN) / 2 + SSR_ROLL_MIN) && roll <= SSR_ROLL_MAX);
}

/**
 * 
 * @param {Banner} banner 
 * @returns {Unit}
 */
const performRareSummon = (banner) =>
{
    const roll = randint(0, 101);

    if (gotR(roll))
        return R_UNITS[randint(0, R_UNITS.length)];
    if (gotFeaturedUnit(roll))
    {
        if (gotSR(roll))
        {
            const featuredSrUnits = filterUnitsByRarity(banner.featuredUnits, "SR");
            return featuredSrUnits[randint(0, featuredSrUnits.length)];
        }
        const featuredSsrUnits = filterUnitsByRarity(banner.featuredUnits, "SSR");
        return featuredSsrUnits[randint(0, featuredSsrUnits.length)];
    }

    if (gotSR(roll))
        return SR_UNITS[randint(0, SR_UNITS.length)];
    return SSR_UNITS[randint(0, SSR_UNITS.length)];
}

